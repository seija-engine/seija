use std::{collections::HashMap, sync::Arc};

use crate::font::Font;

pub struct FontSystem {
   pub(crate) db:fontdb::Database,
   font_cache: HashMap<fontdb::ID, Option<Arc<Font>>>,
}

impl Default for FontSystem {
    fn default() -> Self {
        FontSystem { 
            db:fontdb::Database::default(),
            font_cache:HashMap::default()
        }
    }
}

impl FontSystem {
    pub fn get_font(&mut self, id: fontdb::ID) -> Option<Arc<Font>> {
        self.font_cache.entry(id).or_insert_with(|| {
            self.db.make_face_data_unshared(id);
            let face = self.db.face(id)?;
            match Font::new(face) {
                Some(font) => Some(Arc::new(font)),
                None => { None }
            }
        }).clone()
    }
}