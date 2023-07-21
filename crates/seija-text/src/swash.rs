use std::collections::HashMap;
use bevy_ecs::system::Resource;
use swash::{scale::{ScaleContext, Render, Source, StrikeWith}, zeno::Format};
pub use swash::scale::image::{Content as SwashContent, Image as SwashImage};

use crate::FontSystem;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CacheKey {
    /// Font ID
    pub font_id: fontdb::ID,
    /// Glyph ID
    pub glyph_id: u16,
    /// `f32` bits of font size
    pub font_size_bits: u32
}

#[derive(Resource)]
pub struct SwashCache {
    context:ScaleContext,
    pub image_cache: HashMap<CacheKey, Option<SwashImage>>,
}

impl SwashCache {
    pub fn new() -> Self {
        Self {
            context: ScaleContext::new(),
            image_cache: HashMap::new()
        }
    }

    pub fn get_image(&self,font_system:&mut FontSystem,cache_key:CacheKey) {

    }
}

fn swash_image(font_system: &mut FontSystem,context: &mut ScaleContext,cache_key: CacheKey) -> Option<SwashImage> {
    let font = match font_system.get_font(cache_key.font_id) {
        Some(some) => some,
        None => { log::warn!("not found font {:?}", cache_key.font_id); return None; }
    };

    let mut scaler = context.builder(font.as_swash_font())
                            .size(f32::from_bits(cache_key.font_size_bits))
                            .hint(true).build();
    
    Render::new(&[
        Source::ColorOutline(0),
        Source::ColorBitmap(StrikeWith::BestFit),
        Source::Outline,
    ]).format(Format::Alpha).render(&mut scaler, cache_key.glyph_id);
    None
}