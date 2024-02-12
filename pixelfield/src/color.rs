use bmp::Pixel;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/*
pub trait Color: Copy + Clone + IntoEnumIterator + PartialEq + Debug + Unpin + Send + 'static {
    const BITS_PER_PIXEL: u8;
    const BLACK: Self;
    const WHITE: Self;

    fn color_cutpoints() -> Vec<u8> {
        let slices = 2u8.pow(Self::BITS_PER_PIXEL as _) - 1;
        let slice_width = 255 / slices;

        // black is always 0
        let mut cutpoints = vec![0];
        let mut cur = 0;

        for _ in 0..slices {
            cur += slice_width;
            cutpoints.push(cur);
        }

        cutpoints
    }

    fn pixel_cutpoints() -> Vec<u8> {
        Self::color_cutpoints()
    }

    fn from_pixel(pixel: Pixel) -> Self {
        let r = pixel.r as f32;
        let g = pixel.g as f32;
        let b = pixel.b as f32;

        let y = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;

        let cutpoints = Self::pixel_cutpoints();

        let color = cutpoints.iter().zip(Self::iter()).find(|(cutpoint, _color)| {
            y <= **cutpoint
        }).map(|(_, color)|  color );

        color.unwrap_or(Self::BLACK)
    }

    fn as_pixel(&self) -> Pixel {
        let cutpoints = Self::color_cutpoints();
        for (i, color) in Self::iter().enumerate() {
            if *self == color {
                return Pixel {
                    r: cutpoints[i],
                    g: cutpoints[i],
                    b: cutpoints[i],
                }
            }
        }

        Pixel {
            r: 0,
            g: 0,
            b: 0,
        }
    }

    fn map<M:Color>(&self) -> M {
        let pixel = self.as_pixel();
        M::from_pixel(pixel)
    }

}

 */

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Rgb(Rgb),
    Binary(BlackAndWhite),
    Gray4(Gray4),
    Gray8(Gray8),
    Gray16(Gray16),
}

impl From<Color> for Pixel {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgb(inner) => inner.into(),
            Color::Binary(inner) => inner.into(),
            Color::Gray4(inner) => inner.into(),
            Color::Gray8(inner) => inner.into(),
            Color::Gray16(inner) => inner.into(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
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

impl From<Rgb> for Pixel {
    fn from(value: Rgb) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum BlackAndWhite {
    Black,
    White,
}

impl From<BlackAndWhite> for Pixel {
    fn from(value: BlackAndWhite) -> Self {
        match value {
            BlackAndWhite::Black => Pixel { r: 0, g: 0, b: 0 },
            BlackAndWhite::White => Pixel {
                r: 255,
                g: 255,
                b: 255,
            },
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

impl From<Gray4> for Pixel {
    fn from(value: Gray4) -> Self {
        match value {
            Gray4::Black => Pixel { r: 0, g: 0, b: 0 },
            Gray4::Gray1 => Pixel {
                r: 86,
                g: 86,
                b: 86,
            },
            Gray4::Gray2 => Pixel {
                r: 171,
                g: 171,
                b: 171,
            },
            Gray4::White => Pixel {
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

impl From<Gray8> for Pixel {
    fn from(value: Gray8) -> Self {
        match value {
            Gray8::Black => Pixel { r: 0, g: 0, b: 0 },
            Gray8::Gray1 => Pixel {
                r: 36,
                g: 36,
                b: 36,
            },
            Gray8::Gray2 => Pixel {
                r: 72,
                g: 72,
                b: 72,
            },
            Gray8::Gray3 => Pixel {
                r: 108,
                g: 108,
                b: 108,
            },
            Gray8::Gray4 => Pixel {
                r: 144,
                g: 144,
                b: 144,
            },
            Gray8::Gray5 => Pixel {
                r: 180,
                g: 180,
                b: 180,
            },
            Gray8::Gray6 => Pixel {
                r: 216,
                g: 216,
                b: 216,
            },
            Gray8::White => Pixel {
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

impl From<Gray16> for Pixel {
    fn from(value: Gray16) -> Self {
        match value {
            Gray16::Black => Pixel { r: 0, g: 0, b: 0 },
            Gray16::Gray1 => Pixel {
                r: 17,
                g: 17,
                b: 17,
            },
            Gray16::Gray2 => Pixel {
                r: 34,
                g: 34,
                b: 34,
            },
            Gray16::Gray3 => Pixel {
                r: 51,
                g: 51,
                b: 51,
            },
            Gray16::Gray4 => Pixel {
                r: 68,
                g: 68,
                b: 68,
            },
            Gray16::Gray5 => Pixel {
                r: 85,
                g: 85,
                b: 85,
            },
            Gray16::Gray6 => Pixel {
                r: 102,
                g: 102,
                b: 102,
            },
            Gray16::Gray7 => Pixel {
                r: 119,
                g: 119,
                b: 119,
            },
            Gray16::Gray8 => Pixel {
                r: 136,
                g: 136,
                b: 136,
            },
            Gray16::Gray9 => Pixel {
                r: 153,
                g: 153,
                b: 153,
            },
            Gray16::Gray10 => Pixel {
                r: 170,
                g: 170,
                b: 170,
            },
            Gray16::Gray11 => Pixel {
                r: 187,
                g: 187,
                b: 187,
            },
            Gray16::Gray12 => Pixel {
                r: 204,
                g: 204,
                b: 204,
            },
            Gray16::Gray13 => Pixel {
                r: 221,
                g: 221,
                b: 221,
            },
            Gray16::Gray14 => Pixel {
                r: 238,
                g: 238,
                b: 238,
            },
            Gray16::White => Pixel {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}
