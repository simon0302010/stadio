use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use enigo::{Enigo, Mouse};
use log::info;
use tiny_skia::{Paint, PathBuilder, Pixmap};

pub struct KeyboardPopup {
    enigo: Arc<Mutex<Enigo>>,
}

impl KeyboardPopup {
    pub fn new(enigo: Arc<Mutex<Enigo>>) -> Self {
        Self { enigo }
    }

    pub fn tick(&mut self, dt: Duration, pos: (f32, f32)) -> Result<(), String> {
        if let Ok(enigo) = self.enigo.lock()
            && let Ok(location) = enigo.location()
        {
            info!(
                "Mouse: {:?}, Left Stick: {:?}, dt: {}ms",
                location,
                pos,
                dt.as_millis()
            );
        } else {
            return Err("Failed to get mouse location".to_string());
        }

        Ok(())
    }
}

pub struct KeyboardKey {
    key: char,
    layer: u8,
    angle: f32,
}

impl KeyboardKey {
    pub fn render(&self, cx: u32, cy: u32, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(255, 255, 255, 255);

        let radius = (100 * self.layer) as f32;
        let angle = self.angle.to_radians();

        let rx = radius * angle.cos();
        let ry = radius * angle.sin();

        let circle = PathBuilder::from_circle(cx as f32 + rx, cy as f32 + ry, radius);
    }
}
