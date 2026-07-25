mod controller;
mod keyboard;

use controller::ControllerState;

use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use iced::wgpu::rwh::RawWindowHandle;
use iced::widget::canvas::{self, Canvas, Frame, Geometry};
use iced::{Color, Element, Fill, Point, Rectangle, Renderer, Subscription, Theme, theme, window};
use iced::{Size, Task, mouse};

use log::info;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt};
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt as SyncExt;

fn main() -> iced::Result {
    simple_logger::init().expect("Failed to init logger");

    iced::application(Stadio::new, Stadio::update, Stadio::view)
        .subscription(Stadio::subscription)
        .theme(Stadio::theme)
        .style(|_, _| theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .window(window::Settings {
            maximized: false,
            decorations: false,
            transparent: true,
            level: window::Level::AlwaysOnTop,
            resizable: false,
            size: Size::new(280.0, 280.0), // 140.0 * 2.0
            ..window::Settings::default()
        })
        .run()
}

struct Stadio {
    controller: ControllerState,
    keyboard_position: Point,
    enigo: Option<Enigo>,
    mouse_passthrough: bool,
    window_id: Option<iced::window::Id>,
    raw_window_id: Option<u32>,
    x11_connection: Option<RustConnection>
}

#[derive(Debug, Clone)]
enum Message {
    Controller(ControllerState),
    WindowId(Option<window::Id>),
    ReportPosition(Option<Point>),
    RawWindowId(Option<u32>)
}

impl Stadio {
    fn new() -> (Self, Task<Message>) {
        let x11_connection = if is_x11() {
            RustConnection::connect(None)
                .map(|(conn, _)| conn)
                .ok()
        } else {
            None
        };

        (
            Self {
                controller: ControllerState::default(),
                keyboard_position: Point::new(550.0, 390.0),
                enigo: Enigo::new(&Settings::default()).ok(),
                mouse_passthrough: false,
                window_id: None,
                raw_window_id: None,
                x11_connection
            },
            window::latest().map(Message::WindowId),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowId(id) => {
                self.window_id = id;
                if let Some(id) = id && is_x11() {
                    return get_x11_id(id);
                }
                Task::none()
            }
            Message::Controller(controller) => {
                let keyboard_opened = magnitude(self.controller.left_stick) <= 0.1
                    && magnitude(controller.left_stick) > 0.1;
                self.controller = controller;

                if !self.mouse_passthrough {
                    self.mouse_passthrough = enable_mouse_passthrough();
                }

                let Some(enigo) = self.enigo.as_mut() else {
                    return Task::none();
                };

                if keyboard_opened && let Ok((x, y)) = enigo.location() {
                    info!("keyboard opened, mouse at ({},{})", x, y);
                    self.keyboard_position = Point::new(x as f32 - 140.0, y as f32 - 140.0);
                }

                if controller.right_trigger {
                    let _ = enigo.button(Button::Left, Direction::Click);
                }

                let (x, y) = controller.right_stick;
                if x.abs() > 0.05 || y.abs() > 0.05 {
                    let _ =
                        enigo.move_mouse((x * 18.0) as i32, (y * -18.0) as i32, Coordinate::Rel);
                }

                if keyboard_opened && let Some(id) = self.window_id {
                    return move_window(id, self.keyboard_position, self.raw_window_id, &mut self.x11_connection);
                }

                Task::none()
            }
            Message::ReportPosition(position) => {
                info!("window moved to {:?}", position);
                Task::none()
            }
            Message::RawWindowId(id) => {
                self.raw_window_id = id;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(controller::listen).map(Message::Controller)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<'_, Message> {
        Canvas::new(self).width(Fill).height(Fill).into()
    }
}

impl canvas::Program<Message> for Stadio {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        keyboard::draw_keyboard(
            &mut frame,
            self.controller.left_stick,
        );
        vec![frame.into_geometry()]
    }
}

fn magnitude(stick: (f32, f32)) -> f32 {
    stick.0.hypot(stick.1)
}

#[cfg(target_os = "macos")]
fn enable_mouse_passthrough() -> bool {
    use objc2::MainThreadMarker;
    use objc2_app_kit::NSApplication;

    let Some(main_thread) = MainThreadMarker::new() else {
        return false;
    };
    let windows = NSApplication::sharedApplication(main_thread).windows();
    let Some(window) = windows.firstObject() else {
        return false;
    };
    window.setIgnoresMouseEvents(true);
    true
}

#[cfg(not(target_os = "macos"))]
fn enable_mouse_passthrough() -> bool {
    false
}

fn move_window(id: window::Id, dest: Point<f32>, raw_window_id: Option<u32>, x11_connection: &mut Option<RustConnection>) -> Task<Message> {
    info!("moving window to {:?}", dest);

    if is_x11() && let Some(raw_id) = raw_window_id && let Some(conn) = x11_connection {
        conn.configure_window(raw_id, &ConfigureWindowAux::new().x(dest.x as i32 - 50).y(dest.y as i32 - 50)).expect("Failed to move window");
        conn.flush().expect("Failed to flush connection");
        conn.sync().expect("Failed to sync connection");
        Task::none()
    } else {
        window::move_to(id, dest).chain(window::position(id).map(Message::ReportPosition))
    }
}

fn get_x11_id(id: window::Id) -> Task<Message> {
    window::run(id, |window| {
        window.window_handle().ok().and_then(|handle| {
            if let RawWindowHandle::Xlib(xlib) = handle.as_raw() {
                Some(xlib.window as u32)
            } else {
                None
            }
        })
    }).map(Message::RawWindowId)
}

fn is_x11() -> bool {
    std::env::var("XDG_SESSION_TYPE")
        .map(|v| v == "x11")
        .unwrap_or(false)
}
