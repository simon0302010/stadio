use std::f32::consts::{FRAC_PI_4, PI, TAU};

use iced::widget::canvas::{Frame, Path};
use iced::{Color, Point};

pub fn draw_keyboard(frame: &mut Frame, center: Point, stick: (f32, f32)) {
    let center = Point::new(140.0, 140.0);

    let distance = stick.0.hypot(stick.1);
    if distance <= 0.1 {
        return;
    }

    let selected_slice =
        ((stick.0.atan2(stick.1).rem_euclid(TAU) + FRAC_PI_4 / 2.0) / FRAC_PI_4) as usize % 8;
    let selected_ring = ((((distance - 0.1) / 0.9) * 3.0) as usize).min(2);
    let rings = [20.0, 60.0, 100.0, 140.0];

    for direction in 0..8 {
        let middle = direction as f32 * FRAC_PI_4 - PI / 2.0;
        let start = middle - FRAC_PI_4 / 2.0;
        let end = middle + FRAC_PI_4 / 2.0;

        for ring in 0..3 {
            let path = Path::new(|path| {
                path.move_to(point_on_circle(center, rings[ring], start));
                path.line_to(point_on_circle(center, rings[ring + 1], start));
                path.line_to(point_on_circle(center, rings[ring + 1], end));
                path.line_to(point_on_circle(center, rings[ring], end));
                path.close();
            });

            let color = if direction == selected_slice && ring == selected_ring {
                Color::from_rgb8(0, 220, 80)
            } else if (direction + ring) % 2 == 0 {
                Color::from_rgba8(35, 35, 35, 0.85)
            } else {
                Color::from_rgba8(60, 60, 60, 0.85)
            };
            frame.fill(&path, color);
        }
    }
}

fn point_on_circle(center: Point, radius: f32, angle: f32) -> Point {
    Point::new(
        center.x + angle.cos() * radius,
        center.y + angle.sin() * radius,
    )
}
