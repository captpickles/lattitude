use crate::view::Renderable;
use ab_glyph::{Font, FontRef, PxScale};
use pixelfield::pixelfield::{Pixel, PixelField, Rectangle};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use glyph_brush_layout::{BuiltInLineBreaker, FontId, GlyphPositioner, HorizontalAlign, Layout, SectionGeometry, SectionGlyph, SectionText, VerticalAlign};
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;
use pixelfield::color::{Color, Rgb};

pub enum Source {
    Static(String),
    Dynamic(Arc<Mutex<Option<String>>>),
}

pub struct Text {
    bbox: Rectangle,
    font: FontRef<'static>,
    size: f32,
    //value: Option<String>,
    source: Source,
}

impl Text {
    pub fn new(bbox: Rectangle, font: FontRef<'static>, size: f32, source: Source) -> Self {
        Self {
            bbox,
            font,
            size,
            source,
        }
    }

    pub fn render(&self, text: &str) -> Option<PixelField> {
        let scale = PxScale::from(self.size);

        let layout = Layout::Wrap {
            line_breaker: BuiltInLineBreaker::default(),
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
        };

        let screen_position = (self.bbox.nw.x as f32 , self.bbox.nw.y as f32);
        let bounds = (self.bbox.se.x as f32, self.bbox.se.y as f32);

        let glpyhs = layout.calculate_glyphs(
            &[self.font.clone()],
            &SectionGeometry {
                screen_position,
                bounds,
            },
            &[SectionText {
                text,
                scale,
                font_id: FontId(0),
            }],
        );


        let mut pixel_field = PixelField::default();

        for glyph in &glpyhs {
            self.glyph(&mut pixel_field, glyph);
        }

        Some(pixel_field)
    }

    fn glyph(&self, pixel_field: &mut PixelField, glyph: &SectionGlyph) {
        if let Some(glyph) = self.font.outline_glyph(glyph.glyph.clone()) {
            let x_offset = glyph.px_bounds().min.x;
            let y_offset = glyph.px_bounds().min.y;
            glyph.draw(|x, y, c| {
                let color_val =  (255 - ((255 as f32 * c) as u8));
                if c > 0.10 {
                    pixel_field.set(
                        (
                            (x as f32 + x_offset) as i32,
                            (y as f32 + y_offset) as i32,
                        ),
                        Color::Rgb(Rgb {
                            r: color_val,
                            g: color_val,
                            b: color_val,
                        })
                    );
                }
            });
        }
    }
}

impl Renderable for Text {
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move {
            let text = match &self.source {
                Source::Static(inner) => {
                    Some(inner.clone())
                }
                Source::Dynamic(inner) => {
                    let locked = inner.lock().await;
                    if let Some(locked) = &*locked {
                        Some(locked.clone())
                    } else {
                        None
                    }
                }
            };

            if let Some(text) = text {
                self.render(&text)
            } else {
                None
            }
        })
    }
}

pub struct FormattedText<Input, FnIn>
where
    FnIn: From<Input> + Send,
{
    formatter: Box<dyn Fn(FnIn) -> String + Send + Sync>,
    state: Arc<Mutex<Option<Input>>>,
    text: Text,
}

impl<Input, FnIn> FormattedText<Input, FnIn>
where
    Input: Clone + Send,
    FnIn: From<Input> + Send,
{
    pub fn new<F: Fn(FnIn) -> String + Send + Sync + 'static>(
        state: Arc<Mutex<Option<Input>>>,
        bbox: Rectangle,
        font: FontRef<'static>,
        size: f32,
        formatter: F,
    ) -> Self {
        Self {
            formatter: Box::new(formatter),
            state: state.clone(),
            text: Text::new(bbox, font, size, Source::Static("howdy".to_string())),
        }
    }
}

impl<Input, FnIn> Renderable for FormattedText<Input, FnIn>
where
    Input: Clone + Send,
    FnIn: From<Input> + Send,
{
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move {
            if let Some(locked) = &*self.state.lock().await {
                let s = (self.formatter)(locked.clone().into());
                self.text.render(&s)
            } else {
                None
            }
        })
    }
}
