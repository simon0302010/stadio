use gilrs::{Axis, Button, EventType, GamepadId, Gilrs};
use iced::futures::{Stream, stream};
use tokio::time::{Duration, sleep};

#[derive(Debug, Default, Clone, Copy)]
pub struct ControllerState {
    pub left_stick: (f32, f32),
    pub right_stick: (f32, f32),
    pub clicked: bool,
}

struct Controller {
    gilrs: Gilrs,
    gamepad: Option<GamepadId>,
}

impl Controller {
    fn new() -> Option<Self> {
        let gilrs = Gilrs::new().ok()?;
        let gamepad = gilrs.gamepads().next().map(|(id, _)| id);
        Some(Self { gilrs, gamepad })
    }

    fn read(&mut self) -> ControllerState {
        let mut clicked = false;
        while let Some(event) = self.gilrs.next_event() {
            match event.event {
                EventType::Disconnected if self.gamepad == Some(event.id) => self.gamepad = None,
                EventType::Disconnected => {}
                EventType::ButtonPressed(Button::RightTrigger2, _) => {
                    self.gamepad = Some(event.id);
                    clicked = true;
                }
                _ => self.gamepad = Some(event.id),
            }
        }

        self.gamepad = self
            .gamepad
            .filter(|id| self.gilrs.connected_gamepad(*id).is_some())
            .or_else(|| self.gilrs.gamepads().next().map(|(id, _)| id));

        let Some(id) = self.gamepad else {
            return ControllerState {
                clicked,
                ..ControllerState::default()
            };
        };
        let gamepad = self.gilrs.gamepad(id);
        ControllerState {
            left_stick: (
                gamepad.value(Axis::LeftStickX),
                gamepad.value(Axis::LeftStickY),
            ),
            right_stick: (
                gamepad.value(Axis::RightStickX),
                gamepad.value(Axis::RightStickY),
            ),
            clicked,
        }
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
                let old_state = previous.unwrap_or_default();
                let keyboard_moved = stick_distance(state.left_stick, old_state.left_stick) > 0.005;
                let mouse_moving =
                    magnitude(state.right_stick) > 0.05 || magnitude(old_state.right_stick) > 0.05;

                if previous.is_none() || keyboard_moved || mouse_moving || state.clicked {
                    return Some((state, (controller, Some(state))));
                }
            }
        },
    )
}

fn is_active(state: ControllerState) -> bool {
    magnitude(state.left_stick) > 0.05 || magnitude(state.right_stick) > 0.05
}

fn magnitude(stick: (f32, f32)) -> f32 {
    stick.0.hypot(stick.1)
}

fn stick_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}
