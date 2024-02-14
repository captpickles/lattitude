use ab_glyph::{FontRef, InvalidFont};
use engine::font::FontRegistry;

#[derive(Hash, PartialEq, Eq)]
pub enum Font {
    Typewriter,
}

pub fn build_font_registry() -> Result<FontRegistry<Font>, InvalidFont> {
    let font = FontRef::try_from_slice(include_bytes!("../../font/JMH Typewriter dry.otf"))?;

    let mut registry = FontRegistry::new(font);

    Ok(registry)
}
