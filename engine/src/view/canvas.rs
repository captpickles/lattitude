use crate::view::{HorizontalAlignment, Renderable, VerticalAlignment};
use pixelfield::pixelfield::{Pixel, PixelField, Point, Rectangle};

pub struct Canvas {
    components: Vec<Component>,
}

impl Canvas {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn place<P: Into<Point>>(&mut self, point: P, renderable: impl Renderable + 'static) {
        let component = Component {
            point: point.into(),
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            renderable: Box::new(renderable),
        };

        self.components.push(component);
    }
}

pub struct Component {
    point: Point,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
    renderable: Box<dyn Renderable>,
}

impl Renderable for Canvas {
    fn render(&self) -> Option<PixelField> {
        let mut pixel_field = PixelField::default();

        for component in &self.components {
            if let Some(rendered) = component.renderable.render() {
                for pixel in rendered.iter() {
                    pixel_field.set(
                        (
                            pixel.point().x + component.point.x,
                            pixel.point().y + component.point.y,
                        ),
                        pixel.color(),
                    );
                }
            }
        }

        Some(pixel_field)
    }
}
