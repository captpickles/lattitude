use crate::model::ModelManager;
use crate::view::Renderable;
use pixelfield::pixelfield::PixelField;
use std::future::Future;
use std::pin::Pin;

pub struct Scale<R: Renderable> {
    inner: R,
    scale: f32,
}

impl<R: Renderable> Scale<R> {
    pub fn new(inner: R, scale: f32) -> Self {
        Self { inner, scale }
    }
}

impl<R: Renderable> Renderable for Scale<R> {
    fn render<'r>(
        &'r self,
        state_manager: &'r ModelManager,
    ) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move {
            self.inner
                .render(state_manager)
                .await
                .map(|inner| inner.scale(self.scale))
        })
    }
}
