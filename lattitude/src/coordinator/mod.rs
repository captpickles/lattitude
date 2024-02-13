use crate::page::PageId;
use engine::controller::Controllers;
use engine::page::{Page, PageManager};
use std::hash::Hash;
use std::time::Duration;

pub struct Coordinator<PageId, const WIDTH: u32, const HEIGHT: u32>
where
    PageId: Hash + PartialEq + Eq,
{
    controllers: Controllers,
    page_manager: PageManager<PageId, WIDTH, HEIGHT>,
}

pub enum Display<PageId> {
    PageRef(PageId),
    Page(Page),
}

impl From<PageId> for Display<PageId> {
    fn from(value: PageId) -> Self {
        Self::PageRef(value)
    }
}

impl From<Page> for Display<PageId> {
    fn from(value: Page) -> Self {
        Self::Page(value)
    }
}

impl<PageId, const WIDTH: u32, const HEIGHT: u32> Coordinator<PageId, WIDTH, HEIGHT>
where
    PageId: Copy + Hash + PartialEq + Eq + for<'d> Into<&'d Display<PageId>>,
{
    pub fn new(controllers: Controllers, page_manager: PageManager<PageId, WIDTH, HEIGHT>) -> Self {
        Self {
            controllers,
            page_manager,
        }
    }

    pub async fn display<'d, D: Into<&'d Display<PageId>>>(&self, display: D)
    where
        PageId: 'd,
    {
        let display = display.into();
        match display {
            Display::PageRef(page_id) => {
                let pixels = self.page_manager.render(*page_id).await;
            }
            Display::Page(page) => {
                let pixels = page.render().await;
            }
        }
    }

    pub async fn run(&self, initial_page: PageId, home_page: PageId) {
        let mut page = initial_page;

        let mut navigation_stack: Vec<Display<PageId>> = Vec::new();

        self.page_manager.render(page).await;
        loop {
            self.controllers.update().await;
            if navigation_stack.is_empty() {
                navigation_stack.push(Display::PageRef(home_page));
            }

            if let Some(cur_page) = navigation_stack.last() {
                self.display(cur_page).await;
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

pub enum Interaction {
    Forward(Display<PageId>),
    Back,
    Home,
    Delay(Duration, Box<Interaction>),
}
