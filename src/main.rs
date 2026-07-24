mod controller;
mod keyboard;

use std::sync::{Arc, Mutex};

use controller::*;
use enigo::{Enigo, Settings};

use crate::keyboard::KeyboardPopup;

fn main() {
    simple_logger::init().expect("Failed to initialize logger");

    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default()).expect("Failed to init enigo")));
    let mut keyboard_popup = KeyboardPopup::new(enigo.clone());
}
