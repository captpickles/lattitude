use std::sync::{Arc, Mutex};
use actix::Recipient;
use pixelfield::color::{Color, };
use pixelfield::pixelfield::{PixelField, Point};
use layout::canvas::{Canvas, CanvasActor, HorizontalAlign, VerticalAlign};
use layout::view;

pub mod main;

#[derive(Clone, Debug)]
pub struct PageComponent<C: Color> {
    point: Point,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
    source: Recipient<view::PubSub<C>>,
}

pub struct Page<C: Color> {
    pixels: Arc<Mutex<PixelField<C>>>,
    components: Vec<PageComponent<C>>,
}


impl<C: Color> Default for Page<C> {
    fn default() -> Self {
        Self {
            pixels: Arc::new( Mutex::new( PixelField::default() )),
            components: vec![],
        }
    }
}

impl<C: Color> Page<C> {

    pub fn new<F: FnOnce(&mut Self)>(f: F) -> CanvasActor<Self, C> {
        let mut this = Self {
            pixels: Arc::new( Mutex::new( PixelField::default() )),
            components: vec![],
        };

        f(&mut this);

        this.into_actor()
    }

    pub fn place<P: Into<Point>, R: Into<Recipient<view::PubSub<C>>>>(&mut self, source: R, point: P, horizontal_align: HorizontalAlign, vertical_align: VerticalAlign) {
        let component = PageComponent {
            point: point.into(),
            h_align: horizontal_align,
            v_align: vertical_align,
            source: source.into()
        };

        self.components.push( component );

    }


    pub fn into_actor(self) -> CanvasActor<Self, C>
        where Self: Sized
    {
        let components = self.components.clone();
        let mut actor = CanvasActor::new(self);

        for (i, component) in components.iter().enumerate() {
            actor.add(i as u32, component.source.clone())
        }

        actor
    }
}

impl<C: Color> Canvas<C> for Page<C> {
    type Discriminant = u32;

    fn draw(&mut self, pixel_field: Arc<Mutex<PixelField<C>>>, discriminant: Self::Discriminant) -> Option<Arc<Mutex<PixelField<C>>>> {
        let pixel_field = pixel_field.lock().unwrap();
        if let Some(component) = self.components.get(discriminant as usize) {
            println!("draw {:?}", component);
            println!("pixels {}", pixel_field.len());

            let mut dest = self.pixels.lock().unwrap();

            for pixel in pixel_field.iter() {

                let x = component.point.x + pixel.point().x;
                let y = component.point.y + pixel.point().y;

                dest.set(
                    (x,y),
                    pixel.color()
                );

            }

        }

        Some(self.pixels.clone())
    }
}