use glyph_brush::ab_glyph::FontArc;
use seija_asset::IAssetLoader;

mod font;

pub struct Font {
    asset:FontArc
}

#[derive(Default)]
pub(crate) struct FontLoader;
