use bevy_ecs::component::Component;
use seija_asset::Handle;
use seija_core::math::Vec4;
use seija_render::material::Material;
use spritesheet::SpriteSheet;

use crate::common::{ImageGenericInfo, ImageType};

#[derive(Component)]
pub struct Sprite2D {
    pub(crate) common:ImageGenericInfo,
    pub(crate) sheet:Option<Handle<SpriteSheet>>,
    pub(crate) custom_material:Option<Handle<Material>>,
    pub(crate) sprite_index:usize
}

impl Sprite2D {
    pub fn simple(sheet:Option<Handle<SpriteSheet>>,sprite_index:usize,color:Vec4) -> Self { 
        Sprite2D { 
            common: ImageGenericInfo { color, typ: ImageType::Simple },
            sheet, 
            custom_material: None, 
            sprite_index 
        }
    }
}