use crate::view::Renderable;
use ab_glyph::FontRef;
use pixelfield::pixelfield::{PixelField, Rectangle};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;

pub struct Text {
    bbox: Rectangle,
    font: FontRef<'static>,
    size: f32,
    value: Option<String>,
}

impl Text {
    pub fn new(bbox: Rectangle, font: FontRef<'static>, size: f32, value: Option<String>) -> Self {
        Self {
            bbox,
            font,
            size,
            value,
        }
    }

    pub fn render(&self, value: &str) -> Option<PixelField> {
        None
    }
}

impl Renderable for Text {
    fn render(&self) -> Option<PixelField> {
        if let Some(value) = &self.value {
            self.render(value)
        } else {
            None
        }
    }
}

pub struct FormattedText<Input, FnIn>
where
    FnIn: From<Input> + Send,
{
    formatter: Box<dyn Fn(FnIn) -> String + Send + Sync>,
    state: Arc<Mutex<Option<Input>>>,
    text: Text,
}

impl<Input, FnIn> FormattedText<Input, FnIn>
where
    Input: Clone + Send,
    FnIn: From<Input> + Send,
{
    pub fn new<F: Fn(FnIn) -> String + Send + Sync + 'static>(
        state: Arc<Mutex<Option<Input>>>,
        bbox: Rectangle,
        font: FontRef<'static>,
        size: f32,
        formatter: F,
    ) -> Self {
        Self {
            formatter: Box::new(formatter),
            state: state.clone(),
            text: Text::new(bbox, font, size, None),
        }
    }
}

impl<Input, FnIn> Renderable for FormattedText<Input, FnIn>
where
    Input: Clone + Send,
    FnIn: From<Input> + Send,
{
    fn render(&self) -> Option<PixelField> {
        if let Some(locked) = &*self.state.blocking_lock() {
            let s = (self.formatter)(locked.clone().into());
            self.text.render(&s)
        } else {
            None
        }
    }
}
