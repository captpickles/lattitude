use pixelfield::pixelfield::PixelField;

pub mod bmp;

pub trait Display {
    fn display(&mut self, pixel_field: &PixelField);
}
