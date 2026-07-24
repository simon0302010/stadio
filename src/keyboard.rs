use std::sync::{Arc, Mutex};

use enigo::{Enigo, Mouse};
use log::info;

pub struct KeyboardPopup {
    enigo: Arc<Mutex<Enigo>>
}

impl KeyboardPopup {
    pub fn new(enigo: Arc<Mutex<Enigo>>) -> Self {
        Self { open: false, enigo }
    }

    pub fn tick(&mut self, t: f64, pos: (f64, f64)) -> Result<(), String> {
        if let Ok(enigo) = self.enigo.lock()
            && let Ok(location) = enigo.location()
        {
            info!("Mouse is at {:?}", location);
        }

        Ok(())
    }
}
