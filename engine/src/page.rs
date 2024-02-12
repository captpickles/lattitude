use crate::view::canvas::Canvas;
use crate::view::Renderable;
use pixelfield::color::Color;
use pixelfield::pixelfield::PixelField;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::task::spawn_blocking;

pub struct Page {
    canvas: Arc<Canvas>,
}

unsafe impl Send for Page {}

impl Page {
    pub fn new(canvas: Canvas) -> Self {
        Self {
            canvas: Arc::new(canvas),
        }
    }

    pub async fn render(&self) -> PixelField {
        let canvas = self.canvas.clone();
        spawn_blocking(move || canvas.render())
            .await
            .ok()
            .unwrap_or_default()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use pixelfield::pixelfield::Rectangle;
    use crate::view::text::Text;
    use super::*;

    #[tokio::test]
    async fn whut() {
        let mut canvas = Canvas::new();
        
        canvas.place(
            (0,0),
            Text::new(
                Rectangle {
                    nw: (0,0).into(),
                    se: (1000, 100).into(),
                },
                todo!(),
                24.3,
            )
        );

        let page = Page::new(canvas);

        let pixels = page.render().await;
    }
}
