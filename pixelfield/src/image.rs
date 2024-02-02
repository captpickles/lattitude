

mod bmp {
    use std::io::Read;
    use bmp::BmpError;
    use crate::color::Color;
    use crate::pixelfield::PixelField;
    pub fn from_reader<C: Color, R: Read>(reader: &mut R) -> Result<PixelField<C>, BmpError> {
        let image = bmp::from_reader(reader)?;

        let mut field = PixelField::<C>::default();

        for x in 0..image.get_width() {
            for y in 0..image.get_height() {
                let pixel = image.get_pixel(x, y);
                let color = C::from_pixel(pixel);
                if color != C::WHITE {
                    field.set(
                        (x, y),
                        C::from_pixel(pixel),
                    )
                }
            }
        }

        Ok(field)
    }
}