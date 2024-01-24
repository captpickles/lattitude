use actix::Recipient;
use effigy::color::{Color, };
use effigy::pixelfield::{PixelField, Point};
use liein::canvas::{Canvas, CanvasActor, HorizontalAlign, VerticalAlign};
use liein::view;

pub mod main;

#[derive(Clone, Debug)]
pub struct PageComponent<C: Color> {
    point: Point,
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
    source: Recipient<view::PubSub<C>>,
}

pub struct Page<C: Color> {
    pixels: PixelField<C>,
    components: Vec<PageComponent<C>>,
}


impl<C: Color> Default for Page<C> {
    fn default() -> Self {
        Self {
            pixels: PixelField::default(),
            components: vec![],
        }
    }
}

impl<C: Color> Page<C> {

    pub fn new<F: FnOnce(&mut Self)>(f: F) -> CanvasActor<Self, C> {
        let mut this = Self {
            pixels: PixelField::default(),
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

    fn draw(&mut self, _pixel_field: PixelField<C>, discriminant: Self::Discriminant) -> Option<&PixelField<C>> {
        if let Some(component) = self.components.get(discriminant as usize) {
            println!("draw {:?}", component);

        }

        Some(&self.pixels)
    }
}