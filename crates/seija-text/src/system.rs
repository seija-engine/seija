use bevy_ecs::{system::{Query,ResMut}, query::Changed};
use crate::{text::Text, font_system::FontSystem};


pub fn update_text_size_system(mut font_system:ResMut<FontSystem>,mut changed_text:Query<&mut Text,Changed<Text>>,) {
   for mut text in changed_text.iter_mut() {
      if let Some(font) = text.font_id.and_then(|v| font_system.get_font(v)) {
         let swash_font = font.as_swash_font();
         let charmap = swash_font.charmap();
         let glyph_metrics = swash_font.glyph_metrics(&[]).scale(text.font_size as f32);
         let mut all_width = 0f32;
         for chr in text.text.chars() {
            let glyph_id = charmap.map(chr);
            let x_advance = glyph_metrics.advance_width(glyph_id);
            all_width += x_advance;
         }
         text.x_size = all_width;
      }
   }
}