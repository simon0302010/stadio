mod controller;
mod keyboard;

use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use iced::{Size, mouse};
use iced::widget::canvas::{self, Canvas, Frame, Geometry};
use iced::{Color, Element, Fill, Point, Rectangle, Renderer, Subscription, Theme, theme, window};

use controller::ControllerState;

fn main() -> iced::Result {
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
}

impl Stadio {
    fn new() -> Self {
        Self {
            controller: ControllerState::default(),
            keyboard_position: Point::new(550.0, 390.0),
            enigo: Enigo::new(&Settings::default()).ok(),
            mouse_passthrough: false,
        }
    }

    fn update(&mut self, controller: ControllerState) {
        let keyboard_opened =
            magnitude(self.controller.left_stick) <= 0.1 && magnitude(controller.left_stick) > 0.1;
        self.controller = controller;

        if !self.mouse_passthrough {
            self.mouse_passthrough = enable_mouse_passthrough();
        }

        let Some(enigo) = self.enigo.as_mut() else {
            return;
        };

        if keyboard_opened && let Ok((x, y)) = enigo.location() {
            self.keyboard_position = Point::new(x as f32, y as f32);
        }

        if controller.right_trigger {
            let _ = enigo.button(Button::Left, Direction::Click);
        }

        let (x, y) = controller.right_stick;
        if x.abs() > 0.05 || y.abs() > 0.05 {
            let _ = enigo.move_mouse((x * 18.0) as i32, (y * -18.0) as i32, Coordinate::Rel);
        }
    }

    fn subscription(&self) -> Subscription<ControllerState> {
        Subscription::run(controller::listen)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<'_, ControllerState> {
        Canvas::new(self).width(Fill).height(Fill).into()
    }
}

impl canvas::Program<ControllerState> for Stadio {
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
            self.keyboard_position,
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
