use bevy_ecs::prelude::Component;
use seija_core::{ math::{Vec4, Mat4}};

use crate::{mesh2d::Mesh2D, types::Rect, sprite_alloc::alloc::SpriteIndex};

use super::{IBuildMesh2D, rect2d::Rect2D, image_info::{ImageGenericInfo, ImageType}};


#[derive(Component)]
pub struct Sprite {
    pub info:ImageGenericInfo,
    pub sprite_index:Option<SpriteIndex>,
    pub is_dirty:bool
}

impl Sprite {
    pub fn simple(sprite:SpriteIndex,color:Vec4) -> Sprite {
        Sprite {
            info:ImageGenericInfo { typ: ImageType::Simple, color },
            sprite_index:Some(sprite),
            is_dirty:true
        }
    }
}


impl IBuildMesh2D for Sprite {
    fn build(&self,rect2d:&Rect2D,uv:Rect<f32>,mat:&Mat4) -> Mesh2D {
        let mesh2d = self.info.build_mesh(mat, rect2d, uv);
        mesh2d
    }
}