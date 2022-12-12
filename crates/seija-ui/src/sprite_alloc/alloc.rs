use seija_render::resource::ImageInfo;
use seija_core::log;
use super::atlas::DynamicAtlas;

#[derive(Default)]
pub struct SpriteAllocator {
    atlas_list:Vec<DynamicAtlas>
}

impl SpriteAllocator {
    pub fn alloc(&mut self,image_info:ImageInfo) {
        let atlas = self.get_or_create_last_atlas();
        atlas.insert(image_info);
    }

    fn get_or_create_last_atlas(&mut self) -> &mut DynamicAtlas {
        for idx in 0..self.atlas_list.len() {
            if !self.atlas_list[idx].is_full {
                return &mut self.atlas_list[idx];
            }
        }
        let new_atlas = DynamicAtlas::new(2048, 2048);
        self.atlas_list.push(new_atlas);
        self.atlas_list.last_mut().unwrap()
    }
}