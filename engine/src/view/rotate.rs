use crate::view::Renderable;
use pixelfield::pixelfield::{PixelField, Rotation};
use std::future::Future;
use std::os::macos::raw::stat;
use std::pin::Pin;
use crate::model::ModelManager;

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
    fn render<'r>(&'r self, state_manager: &'r ModelManager) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move {
            self.inner
                .render(state_manager)
                .await
                .map(|inner| inner.rotate(self.rotation))
        })
    }
}
