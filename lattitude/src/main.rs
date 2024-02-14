#![allow(unused)]

use std::env;
use toml::toml;
use engine::controller::Controllers;
use engine::page::{PageManager};
use engine::view::canvas::Canvas;
use engine::view::{HorizontalAlignment, VerticalAlignment};
use engine::view::pixels::Pixels;
use engine::view::rotate::Rotate;
use engine::view::text::{Source, Text};
use pixelfield::pixelfield::{Rectangle, Rotation};
use crate::art::{Art, build_art_registry};
use crate::coordinator::Coordinator;
use crate::font::{build_font_registry, Font};
use crate::integration::birdnet::{BirdList, BirdNet};
use crate::page::{build_page_manager, LattitudePage};

pub mod integration;
pub mod font;
mod art;
mod page;
mod coordinator;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let font = build_font_registry()?;
    let art = build_art_registry()?;

    let mut controllers = Controllers::new();
    let birds = controllers.register( BirdNet::new() );

    let config = toml! {
        keep = 10
        token = "ovphnKX4kQhdy7DzzUULLWUc"
    };

    controllers.configure( "birdNET", config.into()).await;
    controllers.update().await;

    let coordinator = Coordinator::new(
        controllers,
        build_page_manager::<808, 1404>(&font, &art)
    );

    println!("start coord");
    coordinator.run(
        LattitudePage::Splash,
        LattitudePage::Splash,
    ).await;
    println!("ended coord");

    /*
    let splash = page( |canvas| {
        canvas.place(
            (0,0),
            HorizontalAlignment::Left,
            VerticalAlignment::Top,
            Rotate::new(
                art.get( Art::Logo ),
                Rotation::Clockwise(25.0),
            )
        );

        canvas.place(
            (800, 500),
            HorizontalAlignment::Right,
            VerticalAlignment::Top,
            BirdList::new(
                birds.clone(),
                300,
                font.get(Font::Typewriter),
                10.0,
            )
        );

        canvas.place(
            (400, 200),
            HorizontalAlignment::Center,
            VerticalAlignment::Top,
            Text::new(
                400,
                font.get(Font::Typewriter),
                20.0,
                Source::Static("Låttitüdé".to_string())
            )
        );
    });

    //let mut pages = PageManager::new();
    //pages.register()

    let output = splash.render().await;


    println!("--> {}", output.len());
    let bmp = output.to_bmp();

    let path = env::current_dir().unwrap();
    let path = path.join("lattitude.bmp");

    bmp.save(
        path
    )?;
     */

    println!("Hello, world!");

    Ok(())
}
