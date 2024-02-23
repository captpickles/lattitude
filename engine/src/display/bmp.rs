use crate::display::Display;
use pixelfield::color::Rgb;
use pixelfield::pixelfield::PixelField;
use std::path::PathBuf;

pub struct BmpDisplay<const WIDTH: u32, const HEIGHT: u32> {
    path: PathBuf,
}

impl<const WIDTH: u32, const HEIGHT: u32> BmpDisplay<WIDTH, HEIGHT> {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl<const WIDTH: u32, const HEIGHT: u32> Display for BmpDisplay<WIDTH, HEIGHT> {
    fn display(&mut self, pixel_field: &PixelField) {
        let mut greyscale = PixelField::default();

        for pixel in pixel_field.iter() {
            let luma = pixel.color().luma();
            greyscale.set(
                pixel.point(),
                Rgb {
                    r: luma,
                    g: luma,
                    b: luma,
                }
                .into(),
            )
        }
        greyscale
            .to_bmp(Some((WIDTH, HEIGHT)))
            .save(&self.path)
            .ok();
    }
}
