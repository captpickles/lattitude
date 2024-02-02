use std::env;
use std::sync::{Arc, Mutex};
use bmp::Pixel;
use pixelfield::color::Color;
use pixelfield::pixelfield::PixelField;
use crate::display::Display;

pub struct BmpDisplay {
    width: u32,
    height: u32,
}

impl BmpDisplay {
    pub fn new((width, height): (u32, u32)) -> Self {
        Self {
            width,
            height,
        }

    }

}

impl<C: Color> Display<C> for BmpDisplay {
    fn paint(&mut self, pixel_field: Arc<Mutex<PixelField<C>>>) {
        println!("emit BMP");

        let pixel_field = pixel_field.lock().unwrap();

        let mut image = bmp::Image::new( self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                image.set_pixel(x,y,Pixel{
                    r: 255,
                    g: 255,
                    b: 255,
                })


            }
        }

        for pixel in pixel_field.iter() {
            let point = pixel.point();
            image.set_pixel(
                point.x as u32,
                point.y as u32,
                pixel.color().as_pixel()
            );
        }

        let pwd = env::current_dir().unwrap();
        let dest = pwd.join("lattitude.bmp");
        println!("write {:?}", dest);
        let _ = image.save(dest);
    }
}