use bevy_ecs::prelude::*;
use glyph_brush::{Section, ab_glyph::PxScale, Layout,VerticalAlign,HorizontalAlign,BuiltInLineBreaker, LineBreaker};
use seija_asset::Handle;
use seija_core::math::{Vec4, Vec2};
use seija_render::resource::{Texture, TextureType};
use crate::{types::AnchorAlign, mesh2d::{Vertex2D, Mesh2D}, components::rect2d::Rect2D};
use super::Font;
use num_enum::{TryFromPrimitive,IntoPrimitive};
#[derive(Debug, Clone, Eq, PartialEq,Copy,TryFromPrimitive,IntoPrimitive)]
#[repr(u8)]
pub enum LineMode {
    Single,
    Wrap,
}

impl Into<(HorizontalAlign,VerticalAlign)> for AnchorAlign {
    fn into(self) -> (HorizontalAlign,VerticalAlign) {
        match self {
            AnchorAlign::TopLeft => (HorizontalAlign::Left,VerticalAlign::Top),
            AnchorAlign::Top => (HorizontalAlign::Center,VerticalAlign::Top),
            AnchorAlign::TopRight => (HorizontalAlign::Right,VerticalAlign::Top),
            AnchorAlign::Left => (HorizontalAlign::Left,VerticalAlign::Center),
            AnchorAlign::Center => (HorizontalAlign::Center,VerticalAlign::Center),
            AnchorAlign::Right => (HorizontalAlign::Right,VerticalAlign::Center),
            AnchorAlign::BottomLeft => (HorizontalAlign::Left,VerticalAlign::Bottom),
            AnchorAlign::Bottom => (HorizontalAlign::Center,VerticalAlign::Bottom),
            AnchorAlign::BottomRight => (HorizontalAlign::Right,VerticalAlign::Bottom),
        }
    }
}


#[derive(Component,Debug,Clone)]
#[repr(C)]
pub struct Text {
    pub color:Vec4,
    pub anchor:AnchorAlign,
    pub line_mode:LineMode,
    pub is_auto_size:bool,
    pub font_size:u32,
    pub font:Option<Handle<Font>>,
    pub text:String,
    
}

impl Text {
    pub fn new(font:Handle<Font>,text:String) -> Self {
        Self {
            text,
            font_size:24,
            font:Some(font),
            is_auto_size:true,
            anchor:AnchorAlign::Center,
            line_mode:LineMode::Single,
            color:Vec4::new(1.0,1.0,1.0,1.0),
        }
    }


    pub fn build_mesh(verts:Vec<Vec<Vertex2D>>,color:Vec4) -> Mesh2D {
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
            color,
            indexs
        }
    }

    pub fn build_section(&self,rect:&Rect2D) -> Section {
      let text = glyph_brush::Text::new(&self.text).with_scale(PxScale::from(self.font_size as f32));
      let (h_align,v_align) = self.anchor.into();
      let section = match self.line_mode {
          LineMode::Single => {
              Section::default().with_layout(Layout::SingleLine {
                  h_align,
                  v_align,
                  line_breaker: BuiltInLineBreaker::UnicodeLineBreaker,
              }).add_text(text)
          },
          LineMode::Wrap => {
              Section::default().with_layout(Layout::Wrap {
                  h_align,
                  v_align,
                  line_breaker: BuiltInLineBreaker::UnicodeLineBreaker,
              }).add_text(text)
          }
      };
      if !self.is_auto_size {
        let mut screen_pos = Vec2::new(0f32, 0f32);
        match self.anchor  {
            AnchorAlign::Left => {
                screen_pos.x -= rect.width * 0.5f32;
            }
            _ => {}
        };
        section.with_bounds((rect.width,rect.height)).with_screen_position(screen_pos)
      } else {
        section
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
       //log::error!("write_font_texture:{:?} min_x:{},min_y:{}",rect,min_x,min_y);
       
       for (index,row) in bytes.chunks_exact(rect.width() as usize).enumerate() {
          let mut offset = (index + min_y) * 1024;
          offset = offset + min_x;
          image.data[offset..(offset + rect.width() as usize)].copy_from_slice(row);
       }
    }
}

#[derive(Debug, Hash, Clone, Copy)]
enum CustomLineBreaker {
    BuiltIn(BuiltInLineBreaker),
    None,
}

impl LineBreaker for CustomLineBreaker {
    fn line_breaks<'a>(&self, glyph_info: &'a str) -> Box<dyn Iterator<Item = glyph_brush::LineBreak> + 'a> {
      match self {
        CustomLineBreaker::BuiltIn(inner) => inner.line_breaks(glyph_info),
        CustomLineBreaker::None => Box::new(std::iter::empty()),
    }
    }
}