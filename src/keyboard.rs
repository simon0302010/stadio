use std::{sync::{Arc, Mutex}, time::Duration};

use enigo::{Enigo, Mouse};
use log::info;

pub struct KeyboardPopup {
    enigo: Arc<Mutex<Enigo>>
}

impl KeyboardPopup {
    pub fn new(enigo: Arc<Mutex<Enigo>>) -> Self {
        Self { enigo }
    }

    pub fn tick(&mut self, dt: Duration, pos: (f32, f32)) -> Result<(), String> {
        if let Ok(enigo) = self.enigo.lock()
            && let Ok(location) = enigo.location()
        {
            info!("Mouse: {:?}, Left Stick: {:?}, dt: {}ms", location, pos, dt.as_millis());
        } else {
            return Err("Failed to get mouse location".to_string());
        }

        Ok(())
    }
}

pub struct KeyboardKey {
    key: char,
    layer: u8,
    angle: i32
}

impl KeyboardKey {

}
