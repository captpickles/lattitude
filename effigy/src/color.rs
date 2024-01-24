use std::fmt::Debug;
use bmp::Pixel;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum BlackAndWhite {
    Black,
    White,
}

impl Color for BlackAndWhite {
    const BITS_PER_PIXEL: u8 = 1;
    const BLACK: Self = Self::Black;
    const WHITE: Self = Self::White;

    fn pixel_cutpoints() -> Vec<u8> {
        vec![254,255]
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

impl Color for Gray4 {
    const BITS_PER_PIXEL: u8 = 2;
    const BLACK: Self = Self::Black;
    const WHITE: Self = Self::White;
}

#[repr(u8)]
#[derive(Copy, Clone)]
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

impl Color for Gray16 {
    const BITS_PER_PIXEL: u8 = 4;
    const BLACK: Self = Self::Black;
    const WHITE: Self = Self::White;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_pixel_gray4() {
        let pixel = Gray4::White.as_pixel();

        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 255);
        assert_eq!(pixel.b, 255);

        let pixel = Gray4::Gray2.as_pixel();

        assert_eq!(pixel.r, 170);
        assert_eq!(pixel.g, 170);
        assert_eq!(pixel.b, 170);

        let pixel = Gray4::Gray1.as_pixel();

        assert_eq!(pixel.r, 85);
        assert_eq!(pixel.g, 85);
        assert_eq!(pixel.b, 85);

        let pixel = Gray4::Black.as_pixel();

        assert_eq!(pixel.r, 0);
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 0);
    }

    #[test]
    fn test_from_pixel_gray4() {
        let pixel = Pixel {
            r: 0,
            g: 0,
            b: 0,
        };

        let color = Gray4::from_pixel(pixel);
        assert_eq!( Gray4::Black, color);

        let pixel = Pixel {
            r: 170,
            g: 170,
            b: 170,
        };

        let color = Gray4::from_pixel(pixel);
        assert_eq!(Gray4::Gray2, color);

        let pixel = Pixel {
            r: 255,
            g: 255,
            b: 255,
        };

        let color = Gray4::from_pixel(pixel);
        assert_eq!(Gray4::White, color);
    }

    #[test]
    fn black_and_white() {
        let pixel = Pixel {
            r: 0,
            g: 0,
            b: 0,
        };

        let color = BlackAndWhite::from_pixel(pixel);
        assert_eq!(color, BlackAndWhite::Black);

        let pixel = Pixel {
            r: 255,
            g: 255,
            b: 255,
        };

        let color = BlackAndWhite::from_pixel(pixel);
        assert_eq!(color, BlackAndWhite::White);

        let pixel = Pixel {
            r: 250,
            g: 250,
            b: 250,
        };

        let color = BlackAndWhite::from_pixel(pixel);
        assert_eq!(color, BlackAndWhite::Black);
    }

    #[test]
    fn map() {
        let color = Gray16::Gray10;
        let color : Gray4 = color.map();

        assert_eq!(color, Gray4::Gray2);
    }

}