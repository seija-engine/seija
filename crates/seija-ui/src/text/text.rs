use bevy_ecs::prelude::*;
use seija_asset::Handle;
use seija_core::math::{Vec4, Mat4};
use crate::{types::AnchorAlign, mesh2d::{Vertex2D, Mesh2D}};
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
    pub fn new(font:Handle<Font>,text:String) -> Self {
        Self {
            text,
            font_size:24,
            font:Some(font),
            anchor:AnchorAlign::Left,
            line_mode:LineMode::Single,
            color:Vec4::new(1.0,1.0,1.0,1.0),
        }
    }


    pub fn build_mesh(verts:Vec<Vec<Vertex2D>>) -> Mesh2D {
        seija_core::log::error!("build text mesh:{:?}",verts);
        let mut points:Vec<Vertex2D> = vec![];
        let mut indexs:Vec<u32> = vec![];
        let mut index_offset:usize = 0;
        for char_mesh in verts {
            indexs.extend_from_slice(&[2,1,0,2,3,1].map(|v| v + index_offset as u32));
            index_offset += char_mesh.len();
            points.extend(char_mesh);
        }

        Mesh2D {
            points,
            color:Vec4::ONE,
            indexs
        }
    }
}