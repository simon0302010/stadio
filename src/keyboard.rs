use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use enigo::{Enigo, Mouse};
use log::info;
use tiny_skia::{Paint, PathBuilder, Pixmap, Transform};

pub struct KeyboardPopup {
    keys: Vec<KeyboardKey>,
}

impl KeyboardPopup {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn tick(
        &mut self,
        dt: Duration,
        pos: (f32, f32),
        cx: u32,
        cy: u32,
        pixmap: &mut Pixmap,
    ) -> Result<(), String> {
        if pos.0 < 0.05 && pos.0 > -0.05 && pos.1 < 0.05 && pos.1 > -0.05 {
            return Ok(());
        }

        let full_radius = 100; //* self.keys.len();

        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 255, 0, 255);
        let stick = PathBuilder::from_circle(
            cx as f32 + full_radius as f32 * pos.0,
            cy as f32 + (full_radius * -1) as f32 * pos.1,
            10.0,
        )
        .expect("Failed to create circle");
        pixmap.fill_path(
            &stick,
            &paint,
            tiny_skia::FillRule::EvenOdd,
            Transform::identity(),
            None,
        );

        for key in self.keys.iter() {
            key.render(cx, cy, pixmap);
        }

        Ok(())
    }

    pub fn create_keys(&mut self) {
        self.keys = vec![
            KeyboardKey::new('a', 1, 0),
            KeyboardKey::new('b', 1, 90),
            KeyboardKey::new('c', 1, 180),
            KeyboardKey::new('d', 1, 270),
        ]
    }
}

pub struct KeyboardKey {
    key: char,
    layer: u8,
    angle: f32,
}

impl KeyboardKey {
    pub fn new(key: char, layer: u8, angle: i32) -> Self {
        Self {
            key,
            layer,
            angle: angle as f32,
        }
    }

    pub fn render(&self, cx: u32, cy: u32, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(255, 255, 255, 255);

        let radius = (100 * self.layer) as f32;
        let angle = self.angle.to_radians();

        let rx = radius * angle.cos();
        let ry = radius * angle.sin();

        let circle = PathBuilder::from_circle(cx as f32 + rx, cy as f32 + ry, 15.0)
            .expect("Failed to draw circle");
        pixmap.fill_path(
            &circle,
            &paint,
            tiny_skia::FillRule::EvenOdd,
            Transform::identity(),
            None,
        );
    }
}
