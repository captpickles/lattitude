use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use pixelfield::color::Gray16;
use pixelfield::pixelfield::PixelField;
use layout::view::View;
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