use bevy_ecs::prelude::*;
use seija_asset::Handle;
use seija_core::math::{Vec4};
use seija_render::resource::{Texture, TextureType};
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

pub fn glyph_to_mesh(vert:glyph_brush::GlyphVertex) -> Vec<Vertex2D> {
    let left = vert.pixel_coords.min.x as f32;
    let right = vert.pixel_coords.max.x as f32;
    let top = -vert.pixel_coords.min.y as f32;
    let bottom = -vert.pixel_coords.max.y as f32;
    let uv = vert.tex_coords;
    let verts = vec![
      Vertex2D {
        pos:[left,top,0f32].into(),
        uv:[uv.min.x,uv.min.y].into(),
      },
      Vertex2D {
        pos:[right,top,0f32].into(),
        uv:[uv.max.x,uv.min.y].into(),
      },
      Vertex2D {
        pos:[left,bottom,0f32].into(),
        uv:[uv.min.x,uv.max.y].into(),
      },
      Vertex2D {
        pos:[right,bottom,0f32].into(),
        uv:[uv.max.x,uv.max.y].into(),
      }];
    verts
}


pub fn write_font_texture(texture:&mut Texture,rect:glyph_brush::Rectangle<u32>,bytes:&[u8]) {
    if let TextureType::Image(image) = &mut texture.texture {
       let min_x = rect.min[0] as usize;
       let min_y = rect.min[1] as usize;
       //seija_core::log::error!("write_font_texture:{:?} min_x:{},min_y:{}",rect,min_x,min_y);
       
       for (index,row) in bytes.chunks_exact(rect.width() as usize).enumerate() {
          let mut offset = (index + min_y) * 1024;
          offset = offset + min_x;
          image.data[offset..(offset + rect.width() as usize)].copy_from_slice(row);
       }
    }
}