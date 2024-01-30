use std::sync::{Arc, Mutex};
use actix::{Actor, Context, Handler, Message};
use effigy::color::Color;
use effigy::pixelfield::PixelField;
use liein::canvas::Canvas;

pub mod bmp;
pub mod epd10_3;

#[derive(Message)]
#[rtype(result="()")]
pub struct DisplayPixelField<C:Color>{
    pub pixel_field: Arc<Mutex<PixelField<C>>>,
}

pub trait Display<C: Color> : Send {
    fn paint(&mut self, pixel_field: Arc<Mutex<PixelField<C>>>);

}

impl<C: Color> Canvas<C> for Box<dyn Display<C>> {
    type Discriminant = u32;

    fn draw(&mut self, pixel_field: Arc<Mutex<PixelField<C>>>, discriminant: Self::Discriminant) -> Option<Arc<Mutex<PixelField<C>>>> {
        self.paint(pixel_field.clone());
        Some(pixel_field)
    }
}

impl<C: Color> Actor for Box<dyn Display<C>> {
    type Context = Context<Self>;
}

impl<C: Color> Handler<DisplayPixelField<C>> for Box<dyn Display<C>> {
    type Result = ();

    fn handle(&mut self, msg: DisplayPixelField<C>, ctx: &mut Self::Context) -> Self::Result {
        self.draw( msg.pixel_field, 0);
    }
}
