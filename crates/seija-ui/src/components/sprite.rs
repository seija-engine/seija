use std::sync::Arc;

use bevy_ecs::prelude::Component;
use seija_asset::Handle;
use seija_core::{ math::{Vec4, Mat4}};
use seija_render::material::MaterialDef;
use spritesheet::SpriteSheet;
use seija_core::Rect;
use crate::{mesh2d::Mesh2D, types::{Thickness}, render::UIRender2D};
use super::{ rect2d::Rect2D, image_info::{ImageGenericInfo, ImageType}};


#[derive(Component)]
#[repr(C)]
pub struct Sprite {
    pub info:ImageGenericInfo,
    pub atlas:Option<Handle<SpriteSheet>>,
    pub sprite_index:usize,
}

impl Sprite {
    pub fn simple(sprite:usize,atlas:Option<Handle<SpriteSheet>>,color:Vec4) -> Sprite {
        Sprite {
            info:ImageGenericInfo { typ: ImageType::Simple, color },
            atlas,
            sprite_index:sprite
        }
    }

    pub fn sliced(sprite:usize,atlas:Option<Handle<SpriteSheet>>,thickness:Thickness,color:Vec4) -> Sprite {
        Sprite {
            info:ImageGenericInfo { typ: ImageType::Sliced(thickness), color },
            sprite_index:sprite,
            atlas
        }
    }
}


impl Sprite {
   pub fn build(&self,rect2d:&Rect2D,uv:&Rect<f32>,mat:&Mat4,raw_size:&Rect<u32>,z_order:f32) -> Mesh2D {
        let mesh2d = self.info.build_mesh(mat, rect2d, uv,raw_size,z_order);
        mesh2d
    }

    pub fn build_render(&self,rect2d:&Rect2D,atlas:&SpriteSheet,mat_def:Arc<MaterialDef>) -> Option<UIRender2D> {
        let info = atlas.get_info(self.sprite_index)?;
        let mesh2d = self.build(rect2d, &info.uv, &Mat4::IDENTITY,&info.rect, 0f32);
        Some(UIRender2D {
            mat_def,
            mesh2d,
            texture:Some(atlas.texture.clone()),
            custom_mat:None
        })  
    }
}