use engine::page::Page;
use engine::page;
use engine::view::{HorizontalAlignment, VerticalAlignment};
use engine::view::rotate::Rotate;
use engine::view::text::{Source, Text};
use pixelfield::pixelfield::Rotation;
use crate::art::Art;
use crate::font::Font;
use crate::integration::birdnet::BirdList;
use crate::page::PageContext;

pub fn splash_page(ctx: &PageContext) -> Page {
    page( |canvas| {
        canvas.place(
            (0, 0),
            HorizontalAlignment::Left,
            VerticalAlignment::Top,
            Rotate::new(
                ctx.art(Art::Logo),
                Rotation::Clockwise(25.0),
            )
        );

        /*
        canvas.place(
            (800, 500),
            HorizontalAlignment::Right,
            VerticalAlignment::Top,
            BirdList::new(
                birds.clone(),
                300,
                ctx.font(Font::Typewriter),
                10.0,
            )
        );

         */

        canvas.place(
            (400, 200),
            HorizontalAlignment::Center,
            VerticalAlignment::Top,
            Text::new(
                400,
                ctx.font(Font::Typewriter),
                20.0,
                Source::Static("Låttitüdé".to_string())
            )
        );
    })
}