use chrono::{DateTime, Utc};
use effigy::color::Gray16;
use effigy::pixelfield::PixelField;
use liein::view::View;
use crate::controller::clock::CurrentDateTime;

#[derive(Default)]
pub struct StatusBar {
    now: DateTime<Utc>,
}

impl View<Gray16> for StatusBar {
    type Input = CurrentDateTime;

    fn update<I: Into<CurrentDateTime>>(&mut self, state: I) {
        let state = state.into();
        self.now = state.0;
        println!("now {}", self.now);
    }

    fn repaint(&self) -> Option<PixelField<Gray16>> {
        println!("repaint");
        Some(PixelField::default())
    }
}