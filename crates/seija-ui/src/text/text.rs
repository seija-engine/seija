use bevy_ecs::prelude::*;
use seija_asset::Handle;
use seija_core::math::Vec4;
use crate::types::AnchorAlign;
use super::Font;
use num_enum::{TryFromPrimitive,IntoPrimitive};
#[derive(Debug, Clone, Eq, PartialEq,Copy,TryFromPrimitive,IntoPrimitive)]
#[repr(u8)]
pub enum LineMode {
    Single,
    Wrap,
}

#[derive(Component, Debug)]
pub struct Text {
    pub text:String,
    pub font_size:u32,
    pub font:Option<Handle<Font>>,
    pub anchor:AnchorAlign,
    pub line_mode:LineMode,
    pub color:Vec4,
}

impl Text {
    pub fn new(font:Handle<Font>) -> Self {
        Self {
            text:String::new(),
            font_size:12,
            font:Some(font),
            anchor:AnchorAlign::Left,
            line_mode:LineMode::Single,
            color:Vec4::new(1.0,1.0,1.0,1.0),
        }
    }
}