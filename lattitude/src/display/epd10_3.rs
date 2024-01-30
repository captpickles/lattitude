use std::sync::{Arc, Mutex};
use effigy::color::Gray16;
use effigy::pixelfield::PixelField;
use liein::canvas::Canvas;
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