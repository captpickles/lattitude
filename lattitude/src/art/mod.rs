use bytes::Buf;
use pixelfield::image::bmp_from_reader;
use pixelfield::pixelfield::PixelField;
use std::collections::HashMap;
use engine::view::pixels::Pixels;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Art {
    Logo,
}

pub struct ArtRegistry {
    registry: HashMap<Art, Pixels>,
}

impl ArtRegistry {
    pub fn new() -> Self {
        Self {
            registry: Default::default(),
        }
    }

    pub fn register(&mut self, id: Art, pixels: Pixels) {
        self.registry.insert(id, pixels);
    }

    pub fn get(&self, id: Art) -> Pixels {
        if let Some(pixels) = self.registry.get(&id) {
            pixels.clone()
        } else {
            Pixels::from_pixel_field(PixelField::default())
        }

    }
}

pub fn build_art_registry() -> Result<ArtRegistry, bmp::BmpError> {
    let mut registry = ArtRegistry::new();

    registry.register(
        Art::Logo,
        Pixels::from_bmp(&mut include_bytes!("../../art/captpickles.bmp").reader())?,
    );

    Ok(registry)
}
