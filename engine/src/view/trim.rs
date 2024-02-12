use pixelfield::color::Color;
use pixelfield::pixelfield::PixelField;
use crate::view::Renderable;

pub struct Trim<R: Renderable> {
    inner: R,
    background: Color,
}

impl<R: Renderable> Trim<R> {
    pub fn new(inner: R, background: Color) -> Self {
        Self {
            inner,
            background,
        }
    }
}

impl<R: Renderable> Renderable for Trim<R> {
    fn render(&self) -> Option<PixelField> {
        self.inner.render().map(|inner| {
            inner.trim(self.background)
        })
    }
}