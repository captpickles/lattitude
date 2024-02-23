use crate::view::Renderable;
use pixelfield::pixelfield::PixelField;
use std::future::Future;
use std::pin::Pin;
use crate::model::ModelManager;

pub struct Trim<R: Renderable> {
    inner: R,
}

impl<R: Renderable> Trim<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: Renderable> Renderable for Trim<R> {
    fn render<'r>(&'r self, state_manager: &'r ModelManager) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move { self.inner.render(state_manager).await.map(|inner| inner.trim()) })
    }
}
