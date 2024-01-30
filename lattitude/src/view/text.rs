use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use ab_glyph::{Font, FontRef, PxScale};
use actix::Message;
use glyph_brush_layout::{BuiltInLineBreaker, FontId, GlyphPositioner, HorizontalAlign, Layout, SectionGeometry, SectionText, VerticalAlign};
use effigy::color::Gray16;
use effigy::pixelfield::{PixelField};
use liein::view::View;

pub struct Text<I> {
    formatter: Box<dyn Fn(I) -> String>,
    font: FontRef<'static>,
    size: f32,
    _marker: PhantomData<I>,
    pixel_field: Arc<Mutex<PixelField<Gray16>>>,
}

impl<I> Text<I>
    where
        I: Unpin
{
    pub fn new<F: Fn(I) -> String + 'static>(font: FontRef<'static>, size: f32, formatter: F) -> Self {
        Self {
            formatter: Box::new(formatter),
            font,
            size,
            _marker: Default::default(),
            pixel_field: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl<I> View<Gray16> for Text<I>
    where
        I: Unpin + Debug + Clone + Send + Message<Result=()> + 'static,
{
    type Input = I;

    fn update<IN: Into<Self::Input>>(&mut self, state: IN) -> Option<Arc<Mutex<PixelField<Gray16>>>>{
        let text = (self.formatter)(state.into());

        let scale = PxScale::from(self.size);

        let layout = Layout::Wrap {
            line_breaker: BuiltInLineBreaker::default(),
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
        };

        let glpyhs = layout.calculate_glyphs(
            &[&self.font.clone()],
            &SectionGeometry::default(),
            &[SectionText {
                text: &text,
                scale,
                font_id: FontId(0),
            }],
        );


        let mut pixel_field = PixelField::default();

        for glyph in glpyhs {
            if let Some(glyph) = self.font.outline_glyph(glyph.glyph.clone()) {
                let x_offset = glyph.px_bounds().min.x as u32;
                let y_offset = glyph.px_bounds().min.y as u32;
                glyph.draw(|x, y, c| {
                    let color = if c > 0.9 {
                        Gray16::Black
                    } else if c > 0.7 {
                        Gray16::Gray3
                    } else if c > 0.5 {
                        Gray16::Gray8
                    } else if c > 0.2 {
                        Gray16::Gray12
                    } else {
                        Gray16::White
                    };

                    if !matches!(color, Gray16::White) {
                        pixel_field.set(
                            (x + x_offset, y + y_offset),
                            color,
                        );
                    }
                });
            }
        }

        println!("out {}", pixel_field.len());
        *self.pixel_field.lock().unwrap() = pixel_field;

        Some(self.pixel_field.clone())

    }

}