use seija_render::resource::{ImageInfo, TextureId, BufferId};
use crate::types::Rect;

pub struct SpriteInfo {
   pub rect:Rect<u32>,
   pub uv:Rect<f32>,
   pub image:Option<ImageInfo>
}

pub struct DynamicAtlas {
    pub cache_buffer:Option<BufferId>,
    pub texture:Option<TextureId>,
    pub width:u32,
    pub height:u32,
    pub(crate) used_sprites:Vec<SpriteInfo>,
    free_rects:Vec<Rect<u32>>
}

impl DynamicAtlas {
    pub fn new(width:u32,height:u32) -> Self {
        let free_node = Rect { x:0 , y:0 ,width, height };
        DynamicAtlas {
            cache_buffer:None,
            texture:None, 
            width, 
            height, 
            used_sprites: vec![], 
            free_rects: vec![free_node] 
        }
    }

    pub fn insert(&mut self,w:u32,h:u32) -> Option<usize> {
        if let Some(rect) = self.find_bottom_left(w, h) {
    
           let mut index:i32 = self.free_rects.len() as i32 - 1;
           while index >= 0 {
              let free_rect = self.free_rects[index as usize].clone();
              if self.split_free_node(free_rect,&rect) {
                self.free_rects.remove(index as usize);
              }
              index -= 1;
           }
           self.prune_free_list();
           let f_width = self.width as f32;
           let f_height = self.height as f32;
           let sprite_info = SpriteInfo {
                uv:Rect { 
                    x: rect.x as f32 / f_width, 
                    y: rect.y as f32 / f_height, 
                    width: rect.width as f32 / f_width, 
                    height: rect.height as f32 / f_height
                },
                rect,
                image: None 
           };
           self.used_sprites.push(sprite_info);
           return Some(self.used_sprites.len() - 1);
        } else {
            return None;
        }
    }

    fn split_free_node(&mut self,free_node:Rect<u32>,used_node:&Rect<u32>) -> bool {
        if used_node.x >= free_node.x + free_node.width  ||
           used_node.x + used_node.width <= free_node.x  || 
           used_node.y >= free_node.y + free_node.height || 
           used_node.y + used_node.height <= free_node.y {
            return false
        }
        if used_node.x < free_node.x + free_node.width && used_node.x + used_node.width > free_node.x {
            if used_node.y > free_node.y && used_node.y < free_node.y + free_node.height {
                let mut new_rect = free_node.clone();
                new_rect.height = used_node.y - new_rect.y;
                self.free_rects.push(new_rect);
            }

            if used_node.y + used_node.height < free_node.y + free_node.height {
                let mut new_node = free_node.clone();
                new_node.y = used_node.y + used_node.height;
                new_node.height = free_node.y + free_node.height - (used_node.y + used_node.height);
                self.free_rects.push(new_node);
            }
        }

        if used_node.y < free_node.y + free_node.height && used_node.y + used_node.height > free_node.y {
            if used_node.x > free_node.x && used_node.x < free_node.x + free_node.width {
                let mut new_node = free_node.clone();
                new_node.width = used_node.x - new_node.x;
                self.free_rects.push(new_node);
            }
            if used_node.x + used_node.width < free_node.x + free_node.width {
                let mut new_node = free_node.clone();
                new_node.x = used_node.x + used_node.width;
                new_node.width = free_node.x + free_node.width - (used_node.x + used_node.width);
                self.free_rects.push(new_node);
            }
        }
        true
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

    fn prune_free_list(&mut self) {
        let mut i = 0;
        while i < self.free_rects.len() {
            let mut j = i + 1;
            while j < self.free_rects.len() {
                let ref_a = &self.free_rects[i];
                let ref_b = &self.free_rects[j];
                if Self::is_contained_in(ref_a,ref_b) {
                    self.free_rects.remove(i);
                    i-=1;
                    break;
                }
                if Self::is_contained_in(&ref_b,&ref_a) {
                    self.free_rects.remove(j);
                    j-=1;
                }
                j += 1;
            }
            i += 1;
        }
    }

    fn is_contained_in(a:&Rect<u32>,b:&Rect<u32>) -> bool {
        return a.x >= b.x && a.y >= b.y && a.x + a.width <= b.x + b.width && a.y + a.height <= b.y + b.height;
    }
}
