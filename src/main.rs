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

    let mut last_frame = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now - last_frame;
        last_frame = now;

        if !controller.poll() {
            warn!("Failed to poll controller");
            sleep(Duration::from_millis(20));
            continue;
        }

        if let Err(e) = keyboard_popup.tick(dt, controller.l_stick) {
            warn!("Keyboard popup tick failed: {}", e);
        }

        // wait for at least 20ms
        sleep(Duration::from_millis(20).saturating_sub(now.elapsed()));
    }
}
