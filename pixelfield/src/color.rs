use bmp::Pixel;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Rgb(Rgb),
    Binary(BlackAndWhite),
    Gray4(Gray4),
    Gray8(Gray8),
    Gray16(Gray16),
}

impl Color {
    pub fn luma(&self) -> u8 {
        match self {
            Color::Rgb(inner) => inner.luma(),
            Color::Binary(inner) => inner.luma(),
            Color::Gray4(inner) => inner.luma(),
            Color::Gray8(inner) => inner.luma(),
            Color::Gray16(inner) => inner.luma(),
        }
    }
}

impl From<Color> for Gray4 {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgb(inner) => inner.into(),
            Color::Binary(inner) => Rgb::from(inner).into(),
            Color::Gray4(inner) => inner,
            Color::Gray8(inner) => Rgb::from(inner).into(),
            Color::Gray16(inner) => Rgb::from(inner).into(),
        }
    }
}

impl From<Color> for Pixel {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgb(inner) => inner.into(),
            Color::Binary(inner) => Rgb::from(inner).into(),
            Color::Gray4(inner) => Rgb::from(inner).into(),
            Color::Gray8(inner) => Rgb::from(inner).into(),
            Color::Gray16(inner) => Rgb::from(inner).into(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn luma(&self) -> u8 {
        (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32).round() as u8
    }
}

impl From<Pixel> for Rgb {
    fn from(value: Pixel) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

impl From<Rgb> for Pixel {
    fn from(value: Rgb) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<Rgb> for Gray4 {
    fn from(value: Rgb) -> Self {
        let y = (0.299 * value.r as f32 + 0.587 * value.g as f32 + 0.114 * value.b as f32).round()
            as u8;

        if y > 192 {
            Gray4::White
        } else if y > 128 {
            Gray4::Gray2
        } else if y > 64 {
            Gray4::Gray1
        } else {
            Gray4::Black
        }
    }
}

impl From<BlackAndWhite> for Rgb {
    fn from(value: BlackAndWhite) -> Self {
        match value {
            BlackAndWhite::Black => Rgb { r: 0, g: 0, b: 0 },
            BlackAndWhite::White => Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}

impl From<Rgb> for BlackAndWhite {
    fn from(value: Rgb) -> Self {
        let y = (0.299 * value.r as f32 + 0.587 * value.g as f32 + 0.114 * value.b as f32).round()
            as u8;

        if y > 128 {
            BlackAndWhite::White
        } else {
            BlackAndWhite::Black
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum BlackAndWhite {
    Black,
    White,
}

impl BlackAndWhite {
    pub fn luma(&self) -> u8 {
        match self {
            BlackAndWhite::Black => 0,
            BlackAndWhite::White => 255,
        }
    }
}

impl From<BlackAndWhite> for Gray4 {
    fn from(value: BlackAndWhite) -> Self {
        match value {
            BlackAndWhite::Black => Gray4::Black,
            BlackAndWhite::White => Gray4::White,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum Gray4 {
    Black,
    Gray1,
    Gray2,
    White,
}

impl Gray4 {
    pub fn luma(&self) -> u8 {
        match self {
            Gray4::Black => 0,
            Gray4::Gray1 => 86,
            Gray4::Gray2 => 171,
            Gray4::White => 255,
        }
    }
}

impl From<Gray4> for Rgb {
    fn from(value: Gray4) -> Self {
        match value {
            Gray4::Black => Rgb { r: 0, g: 0, b: 0 },
            Gray4::Gray1 => Rgb {
                r: 86,
                g: 86,
                b: 86,
            },
            Gray4::Gray2 => Rgb {
                r: 171,
                g: 171,
                b: 171,
            },
            Gray4::White => Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Gray8 {
    Black,
    Gray1,
    Gray2,
    Gray3,
    Gray4,
    Gray5,
    Gray6,
    White,
}

impl Gray8 {
    pub fn luma(&self) -> u8 {
        match self {
            Gray8::Black => 0,
            Gray8::Gray1 => 36,
            Gray8::Gray2 => 72,
            Gray8::Gray3 => 180,
            Gray8::Gray4 => 144,
            Gray8::Gray5 => 180,
            Gray8::Gray6 => 216,
            Gray8::White => 255,
        }
    }
}

impl From<Gray8> for Rgb {
    fn from(value: Gray8) -> Self {
        match value {
            Gray8::Black => Rgb { r: 0, g: 0, b: 0 },
            Gray8::Gray1 => Rgb {
                r: 36,
                g: 36,
                b: 36,
            },
            Gray8::Gray2 => Rgb {
                r: 72,
                g: 72,
                b: 72,
            },
            Gray8::Gray3 => Rgb {
                r: 108,
                g: 108,
                b: 108,
            },
            Gray8::Gray4 => Rgb {
                r: 144,
                g: 144,
                b: 144,
            },
            Gray8::Gray5 => Rgb {
                r: 180,
                g: 180,
                b: 180,
            },
            Gray8::Gray6 => Rgb {
                r: 216,
                g: 216,
                b: 216,
            },
            Gray8::White => Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum Gray16 {
    Black,
    Gray1,
    Gray2,
    Gray3,
    Gray4,
    Gray5,
    Gray6,
    Gray7,
    Gray8,
    Gray9,
    Gray10,
    Gray11,
    Gray12,
    Gray13,
    Gray14,
    White,
}

impl Gray16 {
    pub fn luma(&self) -> u8 {
        match self {
            Gray16::Black => 0,
            Gray16::Gray1 => 17,
            Gray16::Gray2 => 34,
            Gray16::Gray3 => 51,
            Gray16::Gray4 => 68,
            Gray16::Gray5 => 85,
            Gray16::Gray6 => 102,
            Gray16::Gray7 => 119,
            Gray16::Gray8 => 136,
            Gray16::Gray9 => 153,
            Gray16::Gray10 => 170,
            Gray16::Gray11 => 187,
            Gray16::Gray12 => 204,
            Gray16::Gray13 => 221,
            Gray16::Gray14 => 238,
            Gray16::White => 255,
        }
    }
}

impl From<Gray16> for Rgb {
    fn from(value: Gray16) -> Self {
        match value {
            Gray16::Black => Rgb { r: 0, g: 0, b: 0 },
            Gray16::Gray1 => Rgb {
                r: 17,
                g: 17,
                b: 17,
            },
            Gray16::Gray2 => Rgb {
                r: 34,
                g: 34,
                b: 34,
            },
            Gray16::Gray3 => Rgb {
                r: 51,
                g: 51,
                b: 51,
            },
            Gray16::Gray4 => Rgb {
                r: 68,
                g: 68,
                b: 68,
            },
            Gray16::Gray5 => Rgb {
                r: 85,
                g: 85,
                b: 85,
            },
            Gray16::Gray6 => Rgb {
                r: 102,
                g: 102,
                b: 102,
            },
            Gray16::Gray7 => Rgb {
                r: 119,
                g: 119,
                b: 119,
            },
            Gray16::Gray8 => Rgb {
                r: 136,
                g: 136,
                b: 136,
            },
            Gray16::Gray9 => Rgb {
                r: 153,
                g: 153,
                b: 153,
            },
            Gray16::Gray10 => Rgb {
                r: 170,
                g: 170,
                b: 170,
            },
            Gray16::Gray11 => Rgb {
                r: 187,
                g: 187,
                b: 187,
            },
            Gray16::Gray12 => Rgb {
                r: 204,
                g: 204,
                b: 204,
            },
            Gray16::Gray13 => Rgb {
                r: 221,
                g: 221,
                b: 221,
            },
            Gray16::Gray14 => Rgb {
                r: 238,
                g: 238,
                b: 238,
            },
            Gray16::White => Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}
