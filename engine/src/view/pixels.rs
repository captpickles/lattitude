use crate::model::ModelManager;
use crate::view::Renderable;
use bmp::BmpError;
use pixelfield::image::bmp_from_reader;
use pixelfield::pixelfield::PixelField;
use std::future::Future;
use std::io::Read;
use std::pin::Pin;

#[derive(Clone)]
pub struct Pixels {
    inner: PixelField,
}

impl Pixels {
    pub fn from_pixel_field(pixel_field: PixelField) -> Pixels {
        Pixels { inner: pixel_field }
    }

    pub fn from_bmp<R: Read>(reader: &mut R) -> Result<Pixels, BmpError> {
        Ok(Self::from_pixel_field(bmp_from_reader(reader)?))
    }
}

impl Renderable for Pixels {
    fn render<'r>(
        &'r self,
        _state_manager: &'r ModelManager,
    ) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move { Some(self.inner.clone()) })
    }
}
