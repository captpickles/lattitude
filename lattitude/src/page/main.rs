use std::sync::{Arc, Mutex};
use pixelfield::color::Gray16;
use pixelfield::pixelfield::PixelField;
use enum_primitive_derive::Primitive;
use layout::canvas::{Canvas};

#[derive(Primitive)]
pub enum MainPageComponents {
    StatusBar = 0,
}

#[derive(Default)]
pub struct MainPage {
    pixels: Arc<Mutex<PixelField<Gray16>>>
}

impl Canvas<Gray16> for MainPage {
    type Discriminant = MainPageComponents;

    fn draw(&mut self, _pixel_field: Arc<Mutex<PixelField<Gray16>>>, discriminant: Self::Discriminant)  -> Option<Arc<Mutex<PixelField<Gray16>>>>{
        match discriminant {
            MainPageComponents::StatusBar => {
                println!("draw statusbar on mainpage")
            }
        }

        Some(self.pixels.clone())
    }
}