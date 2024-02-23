use crate::display::Display;
use crate::page::LattitudePage;
use engine::controller::Controllers;
use engine::page::{Page, PageManager};
use std::env;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use engine::integration::{Integration, Integrations};
use engine::model::ModelManager;

pub enum DisplayPage<PageId> {
    PageRef(PageId),
    Page(Page),
}

impl From<LattitudePage> for DisplayPage<LattitudePage> {
    fn from(value: LattitudePage) -> Self {
        Self::PageRef(value)
    }
}

impl From<Page> for DisplayPage<LattitudePage> {
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
    displays: Vec<Box<dyn Display>>,
    sender: Sender<Interaction>,
    receiver: Receiver<Interaction>,
}

impl<PageId, const WIDTH: u32, const HEIGHT: u32> Coordinator<PageId, WIDTH, HEIGHT>
where
    PageId: Copy + Hash + PartialEq + Eq + Send + Sync,
{
    pub fn new(controllers: Controllers, page_manager: PageManager<PageId, WIDTH, HEIGHT>) -> Self {
        let (sender, receiver) = channel(12);

        Self {
            controllers: Arc::new(Mutex::new(controllers)),
            page_manager,
            displays: vec![],
            sender,
            receiver,
        }
    }

    pub fn add_display<D: Display + 'static>(&mut self, display: D) {
        self.displays.push(Box::new(display));
    }

    pub async fn display<'d, D: Into<&'d DisplayPage<PageId>>>(&mut self, state_manager: &ModelManager, display: D)
    where
        PageId: 'd,
    {
        let display = display.into();
        let pixels = match display {
            DisplayPage::PageRef(page_id) => self.page_manager.render(state_manager, *page_id).await,
            DisplayPage::Page(page) => page.render(state_manager).await,
        };

        for display in self.displays.iter_mut() {
            display.display(&pixels);
        }

        /*
        let bmp = pixels.to_bmp();

        let path = env::current_dir().unwrap();
        let path = path.join("lattitude.bmp");
        println!("wrote image {:?}", path);
        bmp.save(path);
         */
    }

    pub fn sender(&self) -> Sender<Interaction> {
        self.sender.clone()
    }

    pub async fn run(&mut self, state_manager: &ModelManager, initial_page: PageId, home_page: PageId) {
        let mut page = initial_page;
        let mut navigation_stack: Vec<DisplayPage<PageId>> = Vec::new();

        let controllers = self.controllers.clone();

        let handle = tokio::spawn(async move {
            loop {
                println!("running update loop");
                controllers.lock().await.update().await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        // boot screen
        self.page_manager.render(state_manager, page).await;
        tokio::time::sleep(Duration::from_secs(5)).await;

        // regular loop-de-loop
        loop {
            if navigation_stack.is_empty() {
                navigation_stack.push(DisplayPage::PageRef(home_page));
            }

            if let Some(cur_page) = navigation_stack.last() {
                self.display(state_manager, cur_page).await;
            }

            // TODO: select() on timeout or interaction channel
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}

pub enum Interaction {
    Push(DisplayPage<LattitudePage>),
    Pop,
    Clear,
}
