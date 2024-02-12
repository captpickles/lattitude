use pixelfield::pixelfield::PixelField;
use crate::view::Renderable;

pub struct Scale<R: Renderable> {
    inner: R,
    scale: f32,
}

impl<R: Renderable> Scale<R> {

    pub fn new(inner: R, scale: f32) -> Self {
        Self {
            inner,
            scale,
        }
    }
}

impl<R: Renderable> Renderable for Scale<R> {
    fn render(&self) -> Option<PixelField> {
        self.inner.render().map(|inner| {
            inner.scale(self.scale)
        })
    }
}