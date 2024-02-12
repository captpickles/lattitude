use crate::view::Renderable;
use pixelfield::image::bmp_from_reader;
use pixelfield::pixelfield::PixelField;
use std::io::Read;
use bmp::BmpError;

#[derive(Clone)]
pub struct Pixels {
    inner: PixelField,
}

impl Pixels {

    pub fn from_pixel_field(pixel_field: PixelField) -> Pixels {
        Pixels {
            inner: pixel_field
        }
    }

    pub fn from_bmp<R: Read>(reader: &mut R) -> Result<Pixels, BmpError> {
        Ok(Self::from_pixel_field(bmp_from_reader(reader)?))
    }
}

impl Renderable for Pixels {
    fn render(&self) -> Option<PixelField> {
        Some(self.inner.clone())
    }
}
