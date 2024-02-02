use std::sync::{Arc, Mutex};
use pixelfield::color::Gray16;
use pixelfield::pixelfield::PixelField;
use layout::canvas::Canvas;
use crate::display::Display;

pub struct Vertical {

}

impl Vertical {

}


impl Display<Gray16> for Vertical {
    fn paint(&mut self, pixel_field: Arc<Mutex<PixelField<Gray16>>>) {
        todo!()
    }
}