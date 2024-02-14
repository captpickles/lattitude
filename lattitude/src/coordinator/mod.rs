use std::env;
use crate::page::LattitudePage;
use engine::controller::Controllers;
use engine::page::{Page, PageManager};
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender, channel};


pub enum Display<PageId> {
    PageRef(PageId),
    Page(Page),
}

impl From<LattitudePage> for Display<LattitudePage> {
    fn from(value: LattitudePage) -> Self {
        Self::PageRef(value)
    }
}

impl From<Page> for Display<LattitudePage> {
    fn from(value: Page) -> Self {
        Self::Page(value)
    }
}

pub struct Coordinator<PageId, const WIDTH: u32, const HEIGHT: u32>
    where
        PageId: Hash + PartialEq + Eq,
{
    controllers: Arc<Mutex<Controllers>>,
    page_manager: PageManager<PageId, WIDTH, HEIGHT>,
    sender: Sender<Interaction>,
    receiver: Receiver<Interaction>,
}

impl<PageId, const WIDTH: u32, const HEIGHT: u32> Coordinator<PageId, WIDTH, HEIGHT>
where
    PageId: Copy + Hash + PartialEq + Eq + Send + Sync
{
    pub fn new(controllers: Controllers, page_manager: PageManager<PageId, WIDTH, HEIGHT>) -> Self {
        let (sender, receiver) =  channel(12);

        Self {
            controllers: Arc::new(Mutex::new(controllers)),
            page_manager,
            sender,
            receiver
        }
    }

    pub async fn display<'d, D: Into<&'d Display<PageId>>>(&self, display: D)
    where
        PageId: 'd,
    {
        let display = display.into();
        let pixels = match display {
            Display::PageRef(page_id) => {
                self.page_manager.render(*page_id).await
            }
            Display::Page(page) => {
                page.render().await
            }
        };
        let bmp = pixels.to_bmp();

        let path = env::current_dir().unwrap();
        let path = path.join("lattitude.bmp");
        println!("wrote image {:?}", path);
        bmp.save(path);
    }

    pub fn sender(&self) -> Sender<Interaction> {
        self.sender.clone()
    }

    pub async fn run(&self, initial_page: PageId, home_page: PageId) {
        let mut page = initial_page;
        let mut navigation_stack: Vec<Display<PageId>> = Vec::new();

        let controllers = self.controllers.clone();

        let handle = tokio::spawn( async move {
            loop {
                println!("running update loop");
                controllers.lock().await.update().await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        } );

        // boot screen
        self.page_manager.render(page).await;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // regular loop-de-loop
        loop {
            if navigation_stack.is_empty() {
                navigation_stack.push(Display::PageRef(home_page));
            }

            if let Some(cur_page) = navigation_stack.last() {
                self.display(cur_page).await;
            }

            // TODO: select() on timeout or interaction channel
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}

pub enum Interaction {
    Push(Display<LattitudePage>),
    Pop,
    Clear,
}
