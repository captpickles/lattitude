use crate::art::{Art, ArtRegistry};
use crate::font::Font;
use crate::page::splash::splash_page;
use ab_glyph::FontRef;
use engine::font::FontRegistry;
use engine::page::PageManager;
use engine::view::pixels::Pixels;
use pixelfield::pixelfield::PixelField;
use crate::page::unbox::unbox_page;

pub mod splash;
pub mod unbox;

pub struct PageContext<'p> {
    font: &'p FontRegistry<Font>,
    art: &'p ArtRegistry,
}

impl<'p> PageContext<'p> {
    pub fn new(font: &'p FontRegistry<Font>, art: &'p ArtRegistry) -> Self {
        Self { font, art }
    }

    pub fn font(&self, id: Font) -> FontRef<'static> {
        self.font.get(id)
    }

    pub fn art(&self, id: Art) -> Pixels {
        self.art.get(id)
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum PageId {
    Unbox,
    Splash,
}

pub fn build_page_manager<const WIDTH: u32, const HEIGHT: u32>(
    font: &FontRegistry<Font>,
    art: &ArtRegistry,
) -> PageManager<PageId, WIDTH, HEIGHT> {
    let mut manager = PageManager::new();

    let ctx = PageContext::new(font, art);

    manager.register(PageId::Unbox, unbox_page(&ctx));
    manager.register(PageId::Splash, splash_page(&ctx));

    manager
}
