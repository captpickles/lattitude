use crate::page::Page;
use crate::view::canvas::Canvas;

pub mod font;
pub mod page;
pub mod view;

pub mod integration;
pub mod model;

pub mod context;

pub mod engine;

pub fn page<F: Fn(&mut Canvas)>(configure: F) -> Page {
    let mut canvas = Canvas::new();
    configure(&mut canvas);
    Page::new(canvas)
}
