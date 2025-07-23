#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color {
            r: ((value >> 24) & 0xFF) as u8,
            g: ((value >> 16) & 0xFF) as u8,
            b: ((value >> 8) & 0xFF) as u8,
            a: (value & 0xFF) as u8,
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color {
            r: value.0,
            g: value.1,
            b: value.2,
            a: 255,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Color {
            r: value.0,
            g: value.1,
            b: value.2,
            a: value.3,
        }
    }
}

pub enum Colors {
    White,
    Black,
    Transparent,
    Red,
    Green,
    Blue,
}

impl From<Colors> for (u8, u8, u8, u8) {
    fn from(color: Colors) -> Self {
        match color {
            Colors::White => (255, 255, 255, 255),
            Colors::Black => (0, 0, 0, 255),
            Colors::Transparent => (0, 0, 0, 0),
            Colors::Red => (255, 0, 0, 255),
            Colors::Green => (0, 255, 0, 255),
            Colors::Blue => (0, 0, 255, 255),
        }
    }
}

impl From<Colors> for Color {
    fn from(color: Colors) -> Self {
        let (r, g, b, a) = color.into();
        Color { r, g, b, a }
    }
}
