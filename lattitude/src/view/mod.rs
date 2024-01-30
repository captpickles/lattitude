use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use effigy::color::Gray16;
use effigy::pixelfield::PixelField;
use liein::view::View;
use crate::integration::clock::CurrentDateTime;

pub mod text;


#[derive(Default)]
pub struct StatusBar {
    now: DateTime<Utc>,
}

impl View<Gray16> for StatusBar {
    type Input = CurrentDateTime;

    fn update<I: Into<CurrentDateTime>>(&mut self, state: I) -> Option<Arc<Mutex<PixelField<Gray16>>>>{
        let state = state.into();
        self.now = state.0;
        println!("now {}", self.now);
        None
    }

}