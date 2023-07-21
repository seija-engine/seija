use std::{collections::HashMap, sync::Arc};
use bevy_ecs::system::Resource;
use crate::font::Font;

#[derive(Resource)]
pub struct FontSystem {
   pub db:fontdb::Database,
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

    pub fn query_family(&self,family_name:&str) -> Option<fontdb::ID> {
        self.db.query(&fontdb::Query { 
            families: &[fontdb::Family::Name(family_name)],
            ..Default::default() 
         })
    }

    pub fn get_font(&mut self, id: fontdb::ID) -> Option<Arc<Font>> {
        self.font_cache.entry(id).or_insert_with(|| {
            unsafe { self.db.make_shared_face_data(id) };
            let face = self.db.face(id)?;
            match Font::new(face) {
                Some(font) => Some(Arc::new(font)),
                None => { None }
            }
        }).clone()
    }
}