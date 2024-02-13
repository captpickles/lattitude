use crate::view::{HorizontalAlignment, Renderable, VerticalAlignment};
use pixelfield::pixelfield::{Pixel, PixelField, Point, Rectangle};
use std::future::Future;
use std::pin::Pin;

pub struct Canvas {
    components: Vec<Component>,
}

impl Canvas {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn place<P: Into<Point>>(
        &mut self, point: P,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
        renderable: impl Renderable + 'static
    ) {
        let component = Component {
            point: point.into(),
            horizontal_alignment,
            vertical_alignment,
            renderable: Box::new(renderable),
        };

        self.components.push(component);
    }
}



impl Renderable for Canvas {
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r >> {
        Box::pin(async move {
            let mut pixel_field = PixelField::default();

            for component in &self.components {
                component.render(&mut pixel_field).await;
            }

            Some(pixel_field)
        })
    }
}


pub struct Component {
    point: Point,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
    renderable: Box<dyn Renderable>,
}

impl Component {

    pub async fn render(&self, pixel_field: &mut PixelField) {
        if let Some(rendered) = self.renderable.render().await {

            let dimensions = rendered.dimensions();

            let x_offset = match self.horizontal_alignment {
                HorizontalAlignment::Left => {
                    self.point.x
                }
                HorizontalAlignment::Center => {
                    self.point.x - (dimensions.width() as i32 /2)

                }
                HorizontalAlignment::Right => {
                    self.point.x - dimensions.width() as i32
                }
            };

            let y_offset = match self.vertical_alignment {
                VerticalAlignment::Top => {
                    self.point.y
                }
                VerticalAlignment::Middle => {
                    self.point.y - (dimensions.height() as i32 /2)
                }
                VerticalAlignment::Bottom => {
                    self.point.y - dimensions.height() as i32
                }
            };

            for pixel in rendered.iter() {
                pixel_field.set(
                    (
                        pixel.point().x + x_offset,
                        pixel.point().y + y_offset
                    ),
                    pixel.color(),
                );
            }
        }

    }

}