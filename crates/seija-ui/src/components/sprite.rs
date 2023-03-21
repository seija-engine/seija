use bevy_ecs::prelude::Component;
use seija_core::{ math::{Vec4, Mat4}};

use crate::{mesh2d::Mesh2D, types::{Rect, Thickness}};

use super::{ rect2d::Rect2D, image_info::{ImageGenericInfo, ImageType}};

type SpriteIndex = usize;

#[derive(Component)]
pub struct Sprite {
    pub info:ImageGenericInfo,
    pub sprite_index:Option<SpriteIndex>,
}

impl Sprite {
    pub fn simple(sprite:SpriteIndex,color:Vec4) -> Sprite {
        Sprite {
            info:ImageGenericInfo { typ: ImageType::Simple, color },
            sprite_index:Some(sprite)
        }
    }

    pub fn sliced(sprite:SpriteIndex,thickness:Thickness,color:Vec4) -> Sprite {
        Sprite {
            info:ImageGenericInfo { typ: ImageType::Sliced(thickness), color },
            sprite_index:Some(sprite)
        }
    }
}


impl Sprite {
   pub fn build(&self,rect2d:&Rect2D,uv:Rect<f32>,atlas_index:usize,mat:&Mat4,raw_size:&Rect<u32>,z_order:f32) -> Mesh2D {
        let mesh2d = self.info.build_mesh(mat, rect2d, uv,atlas_index,raw_size,z_order);
        mesh2d
    }
}