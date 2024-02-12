use pixelfield::pixelfield::{PixelField, Rotation};
use crate::view::Renderable;

pub struct Rotate<R: Renderable> {
    inner: R,
    rotation: Rotation,
}

impl<R: Renderable> Rotate<R> {
    pub fn new(inner: R, rotation: Rotation) -> Self {
        Self {
            inner,
            rotation,
        }
    }
}

impl<R:Renderable> Renderable for Rotate<R> {
    fn render(&self) -> Option<PixelField> {
        self.inner.render().map(|inner| {
            inner.rotate(
                self.rotation
            )
        })
    }
}