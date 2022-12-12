use seija_render::resource::ImageInfo;
use crate::types::Rect;
use seija_core::log;

pub struct SpriteInfo {
   pub rect:Rect<u32>,
   image:Option<ImageInfo>
}

pub struct DynamicAtlas {
    width:u32,
    height:u32,
    used_sprites:Vec<SpriteInfo>,
    free_rects:Vec<Rect<u32>>,
    pub is_full:bool
}

impl DynamicAtlas {
    pub fn new(width:u32,height:u32) -> Self {
        let free_node = Rect { x:0 , y:0 ,width, height };
        DynamicAtlas { width, height, used_sprites: vec![], free_rects: vec![free_node],is_full:false }
    }

    pub fn insert(&mut self,image_info:ImageInfo) {
        if let Some(rect) = self.find_bottom_left(image_info.width, image_info.height) {
           let sprite_info = SpriteInfo {rect,image:Some(image_info) };
           self.used_sprites.push(sprite_info);
        } else {
            self.is_full = true;
        }
    }

    fn find_bottom_left(&self,width:u32,height:u32) -> Option<Rect<u32>> {
        let mut best_node = None;
        let mut best_x = 0u32;
        let mut best_y = u32::MAX;
        for free_rect in self.free_rects.iter() {
            if free_rect.width >= width && free_rect.height >= height {
                let top_side_y = free_rect.y + height;
                if top_side_y < best_y || (top_side_y == best_y && free_rect.x < best_x) {
                    best_node = Some(Rect { x: free_rect.x, y: free_rect.y, width, height } );
                    best_y = top_side_y;
                    best_x = free_rect.x;
                }
            }
        }

        best_node
    }
}