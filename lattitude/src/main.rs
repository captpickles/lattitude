#![allow(unused)]

use crate::art::{build_art_registry, Art};
use crate::cli::Cli;
use crate::display::bmp::BmpDisplay;
use crate::font::{build_font_registry, Font};
use crate::integration::birdnet::{BirdList, BirdNetRecentDetections};
use crate::page::{build_page_manager, LattitudePage};
use clap::Parser;
use engine::page::PageManager;
use engine::view::canvas::Canvas;
use engine::view::pixels::Pixels;
use engine::view::rotate::Rotate;
use engine::view::text::{Source, Text};
use engine::view::{HorizontalAlignment, VerticalAlignment};
use pixelfield::pixelfield::{Rectangle, Rotation};
use std::env;
use toml::toml;

mod art;
mod cli;
//mod coordinator;
mod display;
pub mod font;
pub mod integration;
mod page;

pub const WIDTH: u32 = 1404;
pub const HEIGHT: u32 = 1872;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let cli = Cli::parse();

    cli.command.run().await;
    /*
    let font = build_font_registry()?;
    let art = build_art_registry()?;

    let mut controllers = Controllers::new();
    let birds = controllers.register(BirdNet::new());

    let config = toml! {
        keep = 10
        token = "ovphnKX4kQhdy7DzzUULLWUc"
    };

    controllers.configure("birdNET", config.into()).await;
    controllers.update().await;

    let mut coordinator = Coordinator::new(controllers, build_page_manager::<808, 1404>(&font, &art));

    let path = env::current_dir().unwrap();
    let path = path.join("lattitude.bmp");

    coordinator.add_display(
        BmpDisplay::<1404, 1872>::new(path)
    );

    println!("start coord");
    coordinator
        .run(LattitudePage::Splash, LattitudePage::Splash)
        .await;
    println!("ended coord");

     */

    Ok(())
}
