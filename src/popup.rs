use std::sync::{Arc, Mutex};

use enigo::Enigo;

pub struct KeyboardPopup {
    open: bool,
    enigo: Arc<Mutex<Enigo>>
}

impl KeyboardPopup {
    pub fn new(enigo: Arc<Mutex<Enigo>>) -> Self {
        Self { open: false, enigo }
    }

    pub fn open(&mut self) {
        self.open = true;
    }
}
