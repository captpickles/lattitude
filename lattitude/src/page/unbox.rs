use crate::page::PageContext;
use engine::page;
use engine::page::Page;

pub fn unbox_page(ctx: &PageContext) -> Page {
    page(|canvas| {})
}
