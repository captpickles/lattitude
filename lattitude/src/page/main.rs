use effigy::color::Gray16;
use effigy::pixelfield::PixelField;
use enum_primitive_derive::Primitive;
use liein::canvas::{Canvas};

#[derive(Primitive)]
pub enum MainPageComponents {
    StatusBar = 0,
}

#[derive(Default)]
pub struct MainPage {
    pixels: PixelField<Gray16>
}

impl Canvas<Gray16> for MainPage {
    type Discriminant = MainPageComponents;

    fn draw(&mut self, _pixel_field: PixelField<Gray16>, discriminant: Self::Discriminant)  -> Option<&PixelField<Gray16>>{
        match discriminant {
            MainPageComponents::StatusBar => {
                println!("draw statusbar on mainpage")
            }
        }

        Some(&self.pixels)
    }
}