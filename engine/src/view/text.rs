use crate::model::{ModelKey, ModelManager};
use crate::view::Renderable;
use ab_glyph::{Font, FontRef, PxScale};
use glyph_brush_layout::{
    BuiltInLineBreaker, FontId, GlyphPositioner, HorizontalAlign, Layout, SectionGeometry,
    SectionGlyph, SectionText, VerticalAlign,
};
use pixelfield::color::{Color, Rgb};
use pixelfield::pixelfield::PixelField;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum Source {
    Static(String),
    Managed(ModelKey<String>),
    Dynamic(Arc<Mutex<Option<String>>>),
}

pub struct Text {
    width: u32,
    font: FontRef<'static>,
    size: f32,
    //value: Option<String>,
    source: Source,
}

impl Text {
    pub fn new(width: u32, font: FontRef<'static>, size: f32, source: Source) -> Self {
        Self {
            width,
            font,
            size,
            source,
        }
    }

    fn render_text(&self, text: &str) -> Option<PixelField> {
        let scale = PxScale::from(self.size);

        let layout = Layout::Wrap {
            line_breaker: BuiltInLineBreaker::default(),
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
        };

        let screen_position = (0.0, 0.0);
        let bounds = (self.width as f32, f32::INFINITY);

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
            self.render_glyph(&mut pixel_field, glyph);
        }

        Some(pixel_field)
    }

    fn render_glyph(&self, pixel_field: &mut PixelField, glyph: &SectionGlyph) {
        if let Some(glyph) = self.font.outline_glyph(glyph.glyph.clone()) {
            let x_offset = glyph.px_bounds().min.x;
            let y_offset = glyph.px_bounds().min.y;
            glyph.draw(|x, y, c| {
                let color_val = 255 - ((255.0 * c) as u8);
                if c > 0.10 {
                    pixel_field.set(
                        ((x as f32 + x_offset) as i32, (y as f32 + y_offset) as i32),
                        Color::Rgb(Rgb {
                            r: color_val,
                            g: color_val,
                            b: color_val,
                        }),
                    );
                }
            });
        }
    }
}

impl Renderable for Text {
    fn render<'r>(
        &'r self,
        state_manager: &'r ModelManager,
    ) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move {
            let text = match &self.source {
                Source::Static(inner) => Some(inner.clone()),
                Source::Managed(inner) => state_manager.get(inner).await,
                Source::Dynamic(inner) => inner.lock().await.as_ref().cloned(),
            };

            if let Some(text) = text {
                self.render_text(&text)
            } else {
                None
            }
        })
    }
}

pub struct FormattedText<Input, FnIn>
where
    FnIn: From<Input> + Send,
    Input: Debug + Clone + 'static,
{
    formatter: Box<dyn Fn(FnIn) -> Option<String> + Send + Sync>,
    input_state: ModelKey<Input>,
    output_state: Arc<Mutex<Option<String>>>,
    text: Text,
}

impl<Input, FnIn> FormattedText<Input, FnIn>
where
    Input: Clone + Debug + Send,
    FnIn: From<Input> + Send,
{
    pub fn new<F: Fn(FnIn) -> Option<String> + Send + Sync + 'static>(
        state: ModelKey<Input>,
        width: u32,
        font: FontRef<'static>,
        size: f32,
        formatter: F,
    ) -> Self {
        let output_state = Arc::new(Mutex::new(None));
        Self {
            formatter: Box::new(formatter),
            input_state: state.clone(),
            output_state: output_state.clone(),
            text: Text::new(width, font, size, Source::Dynamic(output_state.clone())),
        }
    }
}

impl<Input, FnIn> Renderable for FormattedText<Input, FnIn>
where
    Input: Sync + Debug + Clone + Send + 'static,
    FnIn: From<Input> + Send,
{
    fn render<'r>(
        &'r self,
        state_manager: &'r ModelManager,
    ) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        Box::pin(async move {
            //if let Some(locked) = &*self.input_state.lock().await {
            if let Some(value) = state_manager.get(&self.input_state).await {
                println!("some model");
                let s = (self.formatter)(value.clone().into());
                println!("formatted {:?}", s);
                *self.output_state.lock().await = s;
                println!("render inner");
                self.text.render(state_manager).await
            } else {
                println!("no data");
                None
            }
        })
    }
}
