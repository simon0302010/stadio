mod controller;
mod keyboard;

use enigo::{Axis, Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use iced::mouse;
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
            maximized: true,
            decorations: false,
            transparent: true,
            level: window::Level::AlwaysOnTop,
            ..window::Settings::default()
        })
        .run()
}

struct Stadio {
    controller: ControllerState,
    page: keyboard::Page,
    keyboard_position: Point,
    enigo: Option<Enigo>,
    mouse_passthrough: bool,
    scroll_remainder: f32,
}

impl Stadio {
    fn new() -> Self {
        Self {
            controller: ControllerState::default(),
            page: keyboard::Page::default(),
            keyboard_position: Point::new(550.0, 390.0),
            enigo: Enigo::new(&Settings::default()).ok(),
            mouse_passthrough: false,
            scroll_remainder: 0.0,
        }
    }

    fn update(&mut self, controller: ControllerState) {
        let keyboard_opened =
            magnitude(self.controller.left_stick) <= 0.1 && magnitude(controller.left_stick) > 0.1;
        self.controller = controller;

        if controller.next_page {
            self.page = self.page.next();
        }

        let selected_key = if controller.confirm {
            keyboard::selected_key(controller.left_stick, self.page)
        } else {
            None
        };

        if !self.mouse_passthrough {
            self.mouse_passthrough = enable_mouse_passthrough();
        }

        let Some(enigo) = self.enigo.as_mut() else {
            return;
        };

        if keyboard_opened && let Ok((x, y)) = enigo.location() {
            self.keyboard_position = Point::new(x as f32, y as f32);
        }

        if controller.left_click {
            let _ = enigo.button(Button::Left, Direction::Click);
        }
        if controller.right_click {
            let _ = enigo.button(Button::Right, Direction::Click);
        }
        if let Some(key) = selected_key {
            press_key(enigo, key);
        }

        let (x, y) = controller.right_stick;
        if x.abs() > 0.05 || y.abs() > 0.05 {
            let _ = enigo.move_mouse((x * 18.0) as i32, (y * -18.0) as i32, Coordinate::Rel);
        }

        self.scroll_remainder += controller.scroll * controller.scroll.abs() * 1.2;
        let scroll = self.scroll_remainder.trunc() as i32;
        if scroll != 0 {
            let _ = enigo.scroll(scroll, Axis::Vertical);
            self.scroll_remainder -= scroll as f32;
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
            self.page,
        );
        vec![frame.into_geometry()]
    }
}

fn magnitude(stick: (f32, f32)) -> f32 {
    stick.0.hypot(stick.1)
}

fn press_key(enigo: &mut Enigo, name: &str) {
    let key = match name {
        "SPACE" => Some(Key::Space),
        "ENTER" => Some(Key::Return),
        "BKSP" => Some(Key::Backspace),
        "TAB" => Some(Key::Tab),
        "SHIFT" => Some(Key::Shift),
        "CAPS" => Some(Key::CapsLock),
        "ESC" => Some(Key::Escape),
        "DEL" => Some(Key::Delete),
        "LEFT" => Some(Key::LeftArrow),
        "RIGHT" => Some(Key::RightArrow),
        "UP" => Some(Key::UpArrow),
        "DOWN" => Some(Key::DownArrow),
        "HOME" => Some(Key::Home),
        "END" => Some(Key::End),
        "PGUP" => Some(Key::PageUp),
        "PGDN" => Some(Key::PageDown),
        _ => None,
    };

    if let Some(key) = key {
        let _ = enigo.key(key, Direction::Click);
    } else if name.chars().count() == 1 {
        let _ = enigo.text(name);
    }
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
