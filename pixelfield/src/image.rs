use crate::color::Color;
use crate::pixelfield::PixelField;
use bmp::BmpError;
use std::io::Read;

pub fn bmp_from_reader<R: Read>(reader: &mut R) -> Result<PixelField, BmpError> {
    let image = bmp::from_reader(reader)?;

    let mut field = PixelField::default();

    for x in 0..image.get_width() {
        for y in 0..image.get_height() {
            let pixel = image.get_pixel(x, y);
            if pixel.r != 255 && pixel.b != 255 && pixel.g != 255 {
                field.set((x, y), Color::Rgb(pixel.into()))
            }
        }
    }

    Ok(field)
}
