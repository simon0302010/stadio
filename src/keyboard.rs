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

    pub fn tick(&mut self, t: Duration, pos: (f32, f32)) -> Result<(), String> {
        if let Ok(enigo) = self.enigo.lock()
            && let Ok(location) = enigo.location()
        {
            info!("Mouse is at {:?}", location);
        }

        Ok(())
    }
}
