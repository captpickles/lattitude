use crate::model::ModelManager;
use crate::view::canvas::Canvas;
use crate::view::Renderable;
use pixelfield::pixelfield::PixelField;
use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::Arc;

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

    pub fn render<'r>(
        &'r self,
        state_manager: &'r ModelManager,
    ) -> Pin<Box<dyn Future<Output = PixelField> + 'r>> {
        Box::pin(async move { self.canvas.render(state_manager).await.unwrap_or_default() })
    }
}

#[derive(Default)]
pub struct PageManager<PageId, const WIDTH: u32, const HEIGHT: u32>
where
    PageId: Hash + PartialEq + Eq,
{
    pages: HashMap<PageId, Page>,
}

impl<PageId, const WIDTH: u32, const HEIGHT: u32> PageManager<PageId, WIDTH, HEIGHT>
where
    PageId: Hash + PartialEq + Eq,
{
    pub fn new() -> Self {
        Self {
            pages: Default::default(),
        }
    }

    pub fn register(&mut self, id: PageId, page: Page) {
        self.pages.insert(id, page);
    }

    pub async fn render(&self, state_manager: &ModelManager, id: PageId) -> PixelField {
        if let Some(page) = self.pages.get(&id) {
            page.render(state_manager).await
        } else {
            PixelField::default()
        }
    }
}
