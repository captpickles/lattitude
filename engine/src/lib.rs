use crate::page::Page;
use crate::view::canvas::Canvas;

pub mod controller;
pub mod font;
pub mod page;
pub mod view;

pub fn page<F: Fn(&mut Canvas)>(configure: F) -> Page {
    let mut canvas = Canvas::new();
    configure(&mut canvas);
    Page::new(canvas)
}
