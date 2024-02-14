use pixelfield::pixelfield::PixelField;
use std::future::Future;
use std::pin::Pin;

pub mod canvas;
pub mod pixels;
pub mod rotate;
pub mod scale;
pub mod text;
pub mod trim;

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

pub trait Renderable: Send + Sync {
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>>;
}
