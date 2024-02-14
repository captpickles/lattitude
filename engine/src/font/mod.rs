use ab_glyph::FontRef;
use std::collections::HashMap;
use std::hash::Hash;

pub struct FontRegistry<FontId>
where
    FontId: Hash + PartialEq + Eq,
{
    default_font: FontRef<'static>,
    fonts: HashMap<FontId, FontRef<'static>>,
}

impl<FontId> FontRegistry<FontId>
where
    FontId: Hash + PartialEq + Eq,
{
    pub fn new(default_font: FontRef<'static>) -> Self {
        Self {
            default_font,
            fonts: Default::default(),
        }
    }

    pub fn register(&mut self, id: FontId, font: FontRef<'static>) {
        self.fonts.insert(id, font);
    }

    pub fn get(&self, id: FontId) -> FontRef<'static> {
        self.fonts
            .get(&id)
            .cloned()
            .unwrap_or(self.default_font.clone())
    }
}
