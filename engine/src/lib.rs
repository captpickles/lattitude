use crate::page::Page;
use crate::view::canvas::Canvas;

pub mod controller;
pub mod view;
pub mod page;
pub mod font;

pub fn page<F: Fn(&mut Canvas)>(configure: F) -> Page {
    let mut canvas = Canvas::new();
    configure(&mut canvas);
    Page::new(canvas)
}