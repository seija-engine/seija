use bevy_ecs::prelude::Component;
use seija_core::{smol_str::SmolStr, math::Vec4};

use crate::{mesh2d::Mesh2D};

use super::{IBuildMesh2D, rect2d::Rect2D, image_info::{ImageGenericInfo, ImageType}};


#[derive(Component)]
pub struct Sprite {
    pub info:ImageGenericInfo,
    pub sprite_name:Option<SmolStr>,
    pub is_dirty:bool
}

impl Sprite {
    pub fn simple(sprite:SmolStr,color:Vec4) -> Sprite {
        Sprite {
            info:ImageGenericInfo { typ: ImageType::Simple, color },
            sprite_name:Some(sprite),
            is_dirty:true
        }
    }
}


impl IBuildMesh2D for Sprite {
    fn build(&self,rect2d:&Rect2D) -> Mesh2D {
        let color = self.info.color;

        Mesh2D { color, 
            points: vec![], 
            indexs:  vec![]
        }
    }
}