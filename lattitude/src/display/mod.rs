use pixelfield::pixelfield::PixelField;

pub mod bmp;
#[cfg(feature = "epd")]
pub mod epd;

pub trait Display {
    fn display(&mut self, pixel_field: &PixelField);
}
