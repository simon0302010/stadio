mod controller;
mod keyboard;

use std::{sync::{Arc, Mutex}, thread::sleep, time::{Duration, Instant}};

use controller::*;
use enigo::{Enigo, Settings};
use log::warn;

use crate::keyboard::KeyboardPopup;

fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("Failed to initialize logger");

    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default()).expect("Failed to init enigo")));
    let mut keyboard_popup = KeyboardPopup::new(enigo.clone());

    let mut controller = Controller::new();

    loop {
        let start = Instant::now();

        if !controller.poll() {
            warn!("Failed to poll controller");
            sleep(Duration::from_millis(20));
            continue;
        }

        // wait for at least 20ms
        sleep(Duration::from_millis(20).saturating_sub(start.elapsed()));

        if let Err(e) = keyboard_popup.tick(start.elapsed(), controller.l_stick) {
            warn!("Keyboard popup tick failed: {}", e);
        }
    }
}
