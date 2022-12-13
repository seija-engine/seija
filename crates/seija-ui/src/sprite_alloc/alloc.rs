use fnv::FnvHashMap;
use seija_render::resource::ImageInfo;
use seija_core::{IDGenU32, anyhow::anyhow};
use super::atlas::{DynamicAtlas, SpriteInfo};
use seija_core::anyhow::Result;
pub struct SpriteIndex {
    pub atlas_index:usize,
    pub sprite_index:usize
}

pub struct SpriteAllocator {
    id_map:FnvHashMap<u32,SpriteIndex>,
    id_gen:IDGenU32,
    pub(crate) atlas_list:Vec<DynamicAtlas>
}

impl SpriteAllocator {
    pub fn new() -> Self {
        SpriteAllocator { 
            id_map:FnvHashMap::default(),
            id_gen:IDGenU32::new(), 
            atlas_list: vec![] 
        }
    }
}

impl SpriteAllocator {
    pub fn alloc(&mut self,image_info:ImageInfo) -> Result<u32> {
        let index = self.insert_image(image_info)?;
        Ok(index)
    }

    fn insert_image(&mut self,image_info:ImageInfo) -> Result<u32> {
        let new_index = self.id_gen.next();
        for idx in 0.. self.atlas_list.len() {
            let dyn_atlas = &mut self.atlas_list[idx];
            if let Some(index) = dyn_atlas.insert(image_info.width,image_info.height) {
                dyn_atlas.used_sprites[index].image = Some(image_info);
                let sprite_index = SpriteIndex { atlas_index:idx,sprite_index:index };
                self.id_map.insert(new_index, sprite_index);
                return Ok(new_index);
            }
        }

        let mut new_atlas = DynamicAtlas::new(2048, 2048);
        if let Some(index) = new_atlas.insert(image_info.width, image_info.height) {
            new_atlas.used_sprites[index].image = Some(image_info);
            self.atlas_list.push(new_atlas);
            let sprite_index = SpriteIndex { atlas_index:self.atlas_list.len() - 1,sprite_index:index };
            self.id_map.insert(new_index, sprite_index);
            return Ok(new_index);
        }
        Err(anyhow!("image size > 2048"))
    }

    pub fn get_sprite_info(&self,key:u32) -> Option<&SpriteInfo> {
        if let Some(index) = self.id_map.get(&key) {
            let sprite_info = &self.atlas_list[index.atlas_index].used_sprites[index.sprite_index];
            return Some(sprite_info);
        }
        None
    }
}