use bevy_ecs::prelude::Component;
use seija_core::{smol_str::SmolStr, math::Vec4};

use crate::types::Thickness;

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum ImageFilledType {
    HorizontalLeft,
    HorizontalRight,
    VerticalTop,
    VerticalBottom
}

impl From<u32> for ImageFilledType {
    fn from(n: u32) -> ImageFilledType {
        match n {
            0 => ImageFilledType::HorizontalLeft,
            1 => ImageFilledType::HorizontalRight,
            2 => ImageFilledType::VerticalTop,
            _ => ImageFilledType::VerticalBottom,
        }
    }
}

pub enum ImageType {
    Simple,
    Sliced(Thickness),
    Filled(ImageFilledType,f32),
    Tiled,
}

impl Default for ImageType {
    fn default() -> Self {
        ImageType::Simple
    }
}


pub struct ImageGenericInfo {
    pub typ:ImageType,
    pub color:Vec4,
}

impl Default for ImageGenericInfo {
    fn default() -> Self {
        ImageGenericInfo { typ: ImageType::default(), color: Vec4::ONE }
    }
}


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