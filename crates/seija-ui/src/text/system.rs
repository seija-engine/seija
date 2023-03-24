use std::collections::HashMap;

use bevy_ecs::{system::{Query,SystemParam, Local, Res,ResMut}, prelude::Entity, query::{Or, Changed}};
use seija_asset::{Handle, Assets, HandleId};
use seija_core::math::Vec2;
use seija_render::resource::{Texture, ImageInfo, TextureDescInfo, TextureType};
use crate::{components::rect2d::Rect2D, mesh2d::{Vertex2D}};
use super::{Text, Font};
use glyph_brush::{GlyphBrush, GlyphBrushBuilder, FontId, Section, GlyphVertex, BrushAction, Rectangle, ab_glyph::PxScale};

pub struct TextSystemEnv {
    font_texture:Option<Handle<Texture>>,
    font_cache:HashMap<Handle<Font>,FontId>,
    glyph_brush:GlyphBrush<Vec<Vertex2D>>,
}

impl Default for TextSystemEnv {
    fn default() -> Self {
        Self {
            font_texture:None,
            font_cache:HashMap::new(),
            glyph_brush:GlyphBrushBuilder::using_fonts(vec![]).cache_redraws(false).initial_cache_size((1024, 1024)).build(),
        }
    }
}


#[derive(SystemParam)]
pub struct TextParams<'w,'s> {
    pub(crate) update_texts:Query<'w,'s,Entity,Or<(Changed<Text>,Changed<Rect2D>)>>,
    pub(crate) texts:Query<'w,'s,(Entity,&'static Text, &'static Rect2D)>,
    pub(crate) font_assets:Res<'w,Assets<Font>>,
}

pub fn update_ui_text(mut text_env:Local<TextSystemEnv>,mut textures:ResMut<Assets<Texture>>,params:TextParams) {
    let font_texture_id = check_create_font_texture(&mut text_env,&mut textures);
    
    for entity in params.update_texts.iter() {
       if let Ok((_,text,rect)) = params.texts.get(entity) {
          if let Some(h_font) = text.font.as_ref() {
             //update font cache
             if !text_env.font_cache.contains_key(h_font) {
                params.font_assets.get(&h_font.id).map(|font| {
                   let font_id = text_env.glyph_brush.add_font(font.asset.clone());
                   text_env.font_cache.insert(h_font.clone(),font_id);
                });
             }
             let text = glyph_brush::Text::new(&text.text).with_scale(PxScale::from(30.0));
            
             let section = Section::default().add_text(text);//.with_bounds((rect.width,rect.height));
             text_env.glyph_brush.queue(section);
          }


          let action = text_env.glyph_brush.process_queued(|r,bytes| {
            if let Some(texture) = textures.get_mut(&font_texture_id) {
              write_font_texture(texture,r,bytes);
            }
          },glyph_to_mesh);

          match action {
              Ok(BrushAction::Draw(verts)) => {

              },
              Ok(BrushAction::ReDraw) => {},
              Err(e) => {
                 seija_core::log::error!("Error: {:?}",e);
              }
          }
       }
    }

    
}

fn glyph_to_mesh(vert:GlyphVertex) -> Vec<Vertex2D> {
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

fn check_create_font_texture(text_env:&mut TextSystemEnv,textures:&mut Assets<Texture>) -> HandleId {
    if let Some(h_texture) = text_env.font_texture.as_ref() {
        return h_texture.id;
    }
    use seija_render::wgpu::TextureFormat;
    let image_info = ImageInfo { width:1024,height:1024,format:TextureFormat::R8Unorm,data:vec![0u8;1024 * 1024] };
    let mut texture_desc = TextureDescInfo::default();
    texture_desc.desc.label = "font_texture".into();
    let font_texture = Texture::create_image(image_info, texture_desc);
    let h_texture = textures.add(font_texture);
    let id = h_texture.id;
    text_env.font_texture = Some(h_texture);
    return id;
}

fn write_font_texture(texture:&mut Texture,rect:Rectangle<u32>,bytes:&[u8]) {
  if let TextureType::Image(image) = &mut texture.texture {
     let min_x = rect.min[0] as usize;
     let min_y = rect.min[1] as usize;
     seija_core::log::error!("write_font_texture:{:?} min_x:{},min_y:{}",rect,min_x,min_y);
     
     for (index,row) in bytes.chunks_exact(rect.width() as usize).enumerate() {
        let mut offset = (index + min_y) * 1024;
        offset = offset + min_x;
        image.data[offset..(offset + rect.width() as usize)].copy_from_slice(row);
     }
  }
}