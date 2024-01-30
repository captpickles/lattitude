use ab_glyph::{FontRef, InvalidFont};

pub fn typewriter() -> Result<FontRef<'static>, InvalidFont> {
    let font = FontRef::try_from_slice(include_bytes!("../../fonts/JMH Typewriter dry.otf"))?;
    Ok(font)
}