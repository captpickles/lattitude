use crate::view::Renderable;
use pixelfield::pixelfield::{PixelField, Rotation};
use std::future::Future;
use std::pin::Pin;

pub struct Rotate<R: Renderable> {
    inner: R,
    rotation: Rotation,
}

impl<R: Renderable> Rotate<R> {
    pub fn new(inner: R, rotation: Rotation) -> Self {
        Self { inner, rotation }
    }
}

impl<R: Renderable> Renderable for Rotate<R> {
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move {
            self.inner
                .render()
                .await
                .map(|inner| inner.rotate(self.rotation))
        })
    }
}
