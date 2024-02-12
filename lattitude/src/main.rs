#![allow(unused)]

use std::env;
use engine::page::Page;
use engine::view::canvas::Canvas;
use engine::view::pixels::Pixels;
use engine::view::rotate::Rotate;
use engine::view::text::{Source, Text};
use pixelfield::pixelfield::{Rectangle, Rotation};
use crate::art::{Art, build_art_registry};
use crate::font::{build_font_registry, Font};

pub mod integration;
pub mod font;
mod art;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let font = build_font_registry()?;
    let art = build_art_registry()?;

    let mut splash = Canvas::new();
    splash.place(
        (0,0),
        Rotate::new(
            art.get( Art::Logo ),
            Rotation::Clockwise(25.0),
        )
    );

    splash.place(
        (200, 800),
        Text::new(
            ((0,0), (300,100)).into(),
            font.get(Font::Typewriter),
            36.0,
            Source::Static("Låttitüdé".to_string())
        )
    );

    let splash = Page::new(splash);

    let output = splash.render().await;

    println!("--> {}", output.len());
    let bmp = output.to_bmp();

    let path = env::current_dir().unwrap();
    let path = path.join("lattitude.bmp");

    bmp.save(
        path
    )?;

    println!("Hello, world!");

    Ok(())
}
