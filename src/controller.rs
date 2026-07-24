use gilrs::{Axis, Button, EventType, GamepadId, Gilrs};

pub struct Controller {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
    pub l_stick: (f32, f32),
    pub r_stick: (f32, f32),
    pub x: bool,
    pub y: bool,
    pub a: bool,
    pub b: bool,
    pub r_trigger: bool,
    pub l_trigger: bool,
}

impl Controller {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        let active_gamepad = gilrs.gamepads().next().map(|(id, _)| id);

        Controller  {
            gilrs,
            active_gamepad,
            l_stick: (0.0, 0.0),
            r_stick: (0.0, 0.0),
            x: false,
            y: false,
            a: false,
            b: false,
            r_trigger: false,
            l_trigger: false,
        }
    }

    pub fn poll(&mut self) -> bool {
        while let Some(event) = self.gilrs.next_event() {
            match event.event {
                EventType::Disconnected if self.active_gamepad == Some(event.id) => {
                    self.active_gamepad = None;
                }
                EventType::Disconnected => {}
                _ => self.active_gamepad = Some(event.id),
            }
        }

        let active_gamepad = self
            .active_gamepad
            .filter(|id| self.gilrs.connected_gamepad(*id).is_some())
            .or_else(|| self.gilrs.gamepads().next().map(|(id, _)| id));

        let Some(id) = active_gamepad else {
            self.clear();
            return false;
        };

        self.active_gamepad = Some(id);

        let gamepad = self.gilrs.gamepad(id);
        self.l_stick = (
            gamepad.value(Axis::LeftStickX),
            gamepad.value(Axis::LeftStickY),
        );
        self.r_stick = (
            gamepad.value(Axis::RightStickX),
            gamepad.value(Axis::RightStickY),
        );
        self.x = gamepad.is_pressed(Button::West);
        self.y = gamepad.is_pressed(Button::North);
        self.a = gamepad.is_pressed(Button::South);
        self.b = gamepad.is_pressed(Button::East);
        self.l_trigger = gamepad.is_pressed(Button::LeftTrigger2);
        self.r_trigger = gamepad.is_pressed(Button::RightTrigger2);

        true
    }

    fn clear(&mut self) {
        self.l_stick = (0.0, 0.0);
        self.r_stick = (0.0, 0.0);
        self.x = false;
        self.y = false;
        self.a = false;
        self.b = false;
        self.r_trigger = false;
        self.l_trigger = false;
    }
}
