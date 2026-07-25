use gilrs::{Axis, Button, EventType, Gilrs};
use iced::futures::{Stream, stream};
use tokio::time::{Duration, sleep};

#[derive(Debug, Default, Clone, Copy)]
pub struct ControllerState {
    pub left_stick: (f32, f32),
    pub right_stick: (f32, f32),
    pub left_click: bool,
    pub right_click: bool,
    pub confirm: bool,
    pub next_page: bool,
    pub scroll: f32,
}

struct Controller {
    gilrs: Gilrs,
}

impl Controller {
    fn new() -> Option<Self> {
        Some(Self {
            gilrs: Gilrs::new().ok()?,
        })
    }

    fn read(&mut self) -> ControllerState {
        let mut state = ControllerState::default();

        while let Some(event) = self.gilrs.next_event() {
            if let EventType::ButtonPressed(button, _) = event.event {
                match button {
                    Button::LeftTrigger => state.right_click = true,
                    Button::RightTrigger => state.left_click = true,
                    Button::South => state.confirm = true,
                    Button::LeftThumb => state.next_page = true,
                    _ => {}
                }
            }
        }

        let Some((_, gamepad)) = self.gilrs.gamepads().next() else {
            return state;
        };
        state.left_stick = (
            gamepad.value(Axis::LeftStickX),
            gamepad.value(Axis::LeftStickY),
        );
        state.right_stick = (
            gamepad.value(Axis::RightStickX),
            gamepad.value(Axis::RightStickY),
        );
        let left_trigger = gamepad
            .button_data(Button::LeftTrigger2)
            .map_or(0.0, |button| button.value());
        let right_trigger = gamepad
            .button_data(Button::RightTrigger2)
            .map_or(0.0, |button| button.value());
        state.scroll = right_trigger - left_trigger;
        state
    }
}

pub fn listen() -> impl Stream<Item = ControllerState> {
    stream::unfold(
        (Controller::new(), None),
        |(mut controller, previous)| async move {
            loop {
                let busy = previous.is_some_and(is_active);
                sleep(Duration::from_millis(if busy { 16 } else { 50 })).await;

                let Some(gamepad) = controller.as_mut() else {
                    sleep(Duration::from_secs(1)).await;
                    controller = Controller::new();
                    continue;
                };

                let state = gamepad.read();
                let old = previous.unwrap_or_default();
                let keyboard_moved = distance(state.left_stick, old.left_stick) > 0.005;
                let mouse_moving =
                    length(state.right_stick) > 0.05 || length(old.right_stick) > 0.05;
                let scrolling = state.scroll.abs() > 0.05 || old.scroll.abs() > 0.05;

                if previous.is_none()
                    || keyboard_moved
                    || mouse_moving
                    || scrolling
                    || state.left_click
                    || state.right_click
                    || state.confirm
                    || state.next_page
                {
                    return Some((state, (controller, Some(state))));
                }
            }
        },
    )
}

fn is_active(state: ControllerState) -> bool {
    length(state.left_stick) > 0.05 || length(state.right_stick) > 0.05 || state.scroll.abs() > 0.05
}

fn length(stick: (f32, f32)) -> f32 {
    stick.0.hypot(stick.1)
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}
