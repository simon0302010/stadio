use std::f32::consts::{FRAC_PI_4, PI, TAU};

use iced::widget::canvas::{self, Frame, Path};
use iced::{Alignment, Color, Font, Point};

#[derive(Debug, Default, Clone, Copy)]
pub enum Page {
    #[default]
    Letters,
    Lowercase,
    More,
    Numbers,
    Symbols,
    Navigation,
}

impl Page {
    pub fn next(self) -> Self {
        match self {
            Self::Letters => Self::Lowercase,
            Self::Lowercase => Self::More,
            Self::More => Self::Numbers,
            Self::Numbers => Self::Symbols,
            Self::Symbols => Self::Navigation,
            Self::Navigation => Self::Letters,
        }
    }

    fn data(self) -> (&'static [&'static str; 24], &'static str) {
        match self {
            Self::Letters => (&LETTERS, "ABC"),
            Self::Lowercase => (&LOWERCASE, "abc"),
            Self::More => (&MORE, "MORE"),
            Self::Numbers => (&NUMBERS, "123"),
            Self::Symbols => (&SYMBOLS, "SYM"),
            Self::Navigation => (&NAVIGATION, "NAV"),
        }
    }
}

const LETTERS: [&str; 24] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X",
];
const LOWERCASE: [&str; 24] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x",
];
const MORE: [&str; 24] = [
    "Y", "Z", "SPACE", "ENTER", "BKSP", "TAB", "SHIFT", "CAPS", "ESC", "DEL", "-", "=", "[", "]",
    "\\", ";", "'", ",", ".", "/", "`", "LEFT", "RIGHT", "UNDO",
];
const NUMBERS: [&str; 24] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ".", ",", "+", "-", "*", "/", "%", "=", "(",
    ")", "SPACE", "ENTER", "BKSP", "TAB",
];
const SYMBOLS: [&str; 24] = [
    "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+", "{", "}", "|", ":", "\"", "<", ">",
    "?", "~", "`", "[", "]",
];
const NAVIGATION: [&str; 24] = [
    "UP", "DOWN", "LEFT", "RIGHT", "HOME", "END", "PGUP", "PGDN", "TAB", "ESC", "DEL", "BKSP",
    "ENTER", "SPACE", "INS", "CAPS", "COPY", "PASTE", "CUT", "UNDO", "F1", "F2", "F3", "F4",
];

pub fn selected_key(stick: (f32, f32), page: Page) -> Option<&'static str> {
    let (direction, ring) = selection(stick)?;
    Some(page.data().0[direction * 3 + ring])
}

pub fn draw_keyboard(frame: &mut Frame, center: Point, stick: (f32, f32), page: Page) {
    let Some((selected_direction, selected_ring)) = selection(stick) else {
        return;
    };
    let rings = [20.0, 60.0, 100.0, 140.0];
    let (keys, page_name) = page.data();

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
            let selected = direction == selected_direction && ring == selected_ring;
            let color = if selected {
                Color::from_rgb8(0, 220, 80)
            } else if (direction + ring) % 2 == 0 {
                Color::from_rgba8(35, 35, 35, 0.85)
            } else {
                Color::from_rgba8(60, 60, 60, 0.85)
            };
            frame.fill(&path, color);

            let radius = (rings[ring] + rings[ring + 1]) / 2.0;
            draw_text(
                frame,
                keys[direction * 3 + ring],
                point_on_circle(center, radius, middle),
                if selected { Color::BLACK } else { Color::WHITE },
            );
        }
    }
    draw_text(frame, page_name, center, Color::WHITE);
}

fn selection(stick: (f32, f32)) -> Option<(usize, usize)> {
    let distance = stick.0.hypot(stick.1);
    if distance <= 0.1 {
        return None;
    }
    let direction =
        ((stick.0.atan2(stick.1).rem_euclid(TAU) + FRAC_PI_4 / 2.0) / FRAC_PI_4) as usize % 8;
    let ring = ((((distance - 0.1) / 0.9) * 3.0) as usize).min(2);
    Some((direction, ring))
}

fn point_on_circle(center: Point, radius: f32, angle: f32) -> Point {
    Point::new(
        center.x + angle.cos() * radius,
        center.y + angle.sin() * radius,
    )
}

fn draw_text(frame: &mut Frame, text: &str, position: Point, color: Color) {
    let size = match text.chars().count() {
        0 | 1 => 16.0,
        2 => 13.0,
        3 => 11.0,
        4 => 9.0,
        _ => 7.0,
    };
    frame.fill_text(canvas::Text {
        content: text.to_string(),
        position,
        color,
        size: size.into(),
        font: Font::DEFAULT,
        align_x: Alignment::Center.into(),
        align_y: Alignment::Center.into(),
        ..canvas::Text::default()
    });
}
