use crate::view::Renderable;
use pixelfield::pixelfield::PixelField;
use std::future::Future;
use std::pin::Pin;

pub struct Trim<R: Renderable> {
    inner: R,
}

impl<R: Renderable> Trim<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: Renderable> Renderable for Trim<R> {
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move { self.inner.render().await.map(|inner| inner.trim()) })
    }
}
