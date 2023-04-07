use std::{sync::Arc, collections::{HashSet, HashMap}};
use seija_core::{log, time::Time};
use bevy_ecs::{world::World, system::{Resource, 
    SystemParam, Query, Commands, Res, ResMut}, prelude::{Entity, EventWriter}, query::{Or, Changed}};
use seija_asset::{AssetServer, Assets, Handle};
use seija_render::{material::{MaterialDefineAsset, MaterialDef, Material},
                   resource::{ Mesh, Texture, ImageInfo, TextureDescInfo, TextureType, BufferId}};
use seija_transform::{hierarchy::{Parent, Children}, Transform};
use spritesheet::SpriteSheet;
use glyph_brush::{GlyphBrush, GlyphBrushBuilder,VerticalAlign,HorizontalAlign,FontId,
    ab_glyph::PxScale, Section, BrushAction,Rectangle, Layout};
use crate::{components::{sprite::Sprite, rect2d::Rect2D, canvas::{Canvas, ZOrder}}, 
            render::{UIRender2D, WriteFontAtlas}, mesh2d::Vertex2D, text::{Text, Font}};
use glyph_brush::GlyphVertex;
use seija_render::wgpu::{TextureFormat};
#[derive(Resource)]
pub struct UIRenderRoot {
    pub(crate) baseui:Arc<MaterialDef>,
    pub(crate) basetext:Arc<MaterialDef>,
    pub(crate) text_brush:GlyphBrush<Vec<Vertex2D>>,
    pub(crate) font_texture:Handle<Texture>,
    font_caches:HashMap<Handle<Font>,FontId>,
    pub(crate) font_buffer:Option<BufferId>,

    pub(crate) despawn_next_frame:Vec<Entity>,
}

pub(crate) fn on_ui_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
    let mut h_basetext = server.load_sync::<MaterialDefineAsset>(world, "materials/text.mat.clj", None).unwrap();
    let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
    let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
    let arc_text_mat_define = mats.get(&h_basetext.id).unwrap().define.clone();
    //常驻
    h_baseui.forget();
    h_basetext.forget();

    let font_texture = create_font_texture(world);
    world.insert_resource(UIRenderRoot {
        baseui:arc_mat_define,
        basetext:arc_text_mat_define,
        text_brush:GlyphBrushBuilder::using_fonts(vec![])
                    .cache_redraws(false)
                    .initial_cache_size((1024, 1024)).build(),
        font_caches:HashMap::default(),
        font_texture,
        font_buffer:None,
        despawn_next_frame:vec![]
    });
}

fn create_font_texture(world:&mut World) -> Handle<Texture> {
    
    let image_info = ImageInfo {width:1024,height:1024,format:TextureFormat::R8Unorm,data:vec![0u8;1024 * 1024] };
    let mut texture_desc = TextureDescInfo::default();
    texture_desc.desc.label = "font_texture".into();
    //texture_desc.sampler_desc.min_filter = FilterMode::Linear;
    //texture_desc.sampler_desc.mag_filter = FilterMode::Linear;
    //texture_desc.sampler_desc.anisotropy_clamp = Some(NonZeroU8::new(16).unwrap());
    let font_texture = Texture::create_image(image_info, texture_desc);
    let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
    let h_texture = textures.add(font_texture);
    h_texture
}

#[derive(SystemParam)]
pub struct RenderMeshParams<'w,'s> {
    pub(crate) update_sprites:Query<'w,'s,Entity,Or<(Changed<Sprite>,Changed<Rect2D>)>>,
    pub(crate) update_texts:Query<'w,'s,Entity,Or<(Changed<Text>,Changed<Rect2D>)>>,
    pub(crate) render2d:Query<'w,'s,&'static mut UIRender2D>,
    pub(crate) font_assets:Res<'w,Assets<Font>>,
    pub(crate) sprites:Query<'w,'s,(&'static Sprite,&'static Rect2D)>,
    pub(crate) texts:Query<'w,'s,(&'static Text,&'static Rect2D)>,
    pub(crate) spritesheets:Res<'w,Assets<SpriteSheet>>,
    pub(crate) textures:ResMut<'w,Assets<Texture>>,
    pub(crate) ui_roots:ResMut<'w,UIRenderRoot>,
    pub(crate) commands:Commands<'w,'s>,
    pub(crate) canvases:Query<'w,'s,&'static Canvas>,
    pub(crate) parents:Query<'w,'s,&'static Parent>,
    pub(crate) zorders:Query<'w,'s,&'static mut ZOrder>,
    pub(crate) children:Query<'w,'s,&'static Children>,
    pub(crate) write_font_atlas:EventWriter<'w,'s,WriteFontAtlas>,
    pub(crate) time:Res<'w,Time>,
}

pub fn update_render_mesh_system(mut params:RenderMeshParams) {
    let mut top_changed_canvas:HashSet<Entity> = HashSet::default();

    //更新Sprite的Mesh
    for entity in params.update_sprites.iter() {
        if let Ok((sprite,rect)) = params.sprites.get(entity) {
            if let Some(atlas) = sprite.atlas.as_ref().map(|h| params.spritesheets.get(&h.id).unwrap()) {
               if let Some(render2d) = sprite.build_render(rect,atlas,params.ui_roots.baseui.clone()) {
                    if let Ok(mut render) = params.render2d.get_mut(entity) {
                        render.mesh2d = render2d.mesh2d;
                        render.texture = render2d.texture;
                    } else {
                        params.commands.entity(entity).insert(render2d);
                    }
               }
            }

            if let Some(top_canvas_entity) = find_top_canvas(entity, &params.parents, &params.canvases) {
                top_changed_canvas.insert(top_canvas_entity);
            }
            log::error!("update mesh init:{:?}",params.time.frame());
        }
    }
    //更新Text的Mesh
    for entity in params.update_texts.iter() {
        if let Ok((text,rect)) = params.texts.get(entity) {
            if let Some(h_font) = text.font.as_ref() {
                //更新字体缓存
                if !params.ui_roots.font_caches.contains_key(h_font) {
                   params.font_assets.get(&h_font.id).map(|font| {
                      let font_id = params.ui_roots.text_brush.add_font(font.asset.clone());
                      params.ui_roots.font_caches.insert(h_font.clone(),font_id);
                   });
                }
                let text = glyph_brush::Text::new(&text.text).with_scale(PxScale::from(text.font_size as f32));
                let section = Section::default().with_layout(Layout::default()
                                                .v_align(VerticalAlign::Center)
                                                .h_align(HorizontalAlign::Center))
                                                .add_text(text);
                params.ui_roots.text_brush.queue(section);
            }
            let font_texture = params.textures.get_mut(&params.ui_roots.font_texture.id).unwrap();
           
            let action = params.ui_roots.text_brush.process_queued(|r,bytes| {
                write_font_texture(font_texture,r,bytes);
                params.write_font_atlas.send(WriteFontAtlas { rect:r });
            },glyph_to_mesh);
            match action {
                Ok(BrushAction::Draw(verts)) => {
                   let mesh2d = Text::build_mesh(verts);
                   if let Ok(mut render) = params.render2d.get_mut(entity) {
                      render.texture = params.ui_roots.font_texture.clone();
                      render.mesh2d = mesh2d;
                    } else {
                        let render2d = UIRender2D {
                            mat:params.ui_roots.basetext.clone(),
                            texture:params.ui_roots.font_texture.clone(),
                            mesh2d 
                        };
                        params.commands.entity(entity).insert(render2d);
                    }
                }
                Ok(BrushAction::ReDraw) => {}
                Err(err) => {
                    seija_core::log::error!("text brush error:{:?}",err);
                }
            }
        }
    }

    //刷新ZOrder
    if !top_changed_canvas.is_empty() {
        for top_entity in top_changed_canvas {
            let start_z = params.zorders.get(top_entity).map(|z| z.value).unwrap_or(0);
            ZOrder::update(start_z,top_entity, &mut params.zorders, &params.children,&mut params.commands);
        }
    }
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

#[derive(SystemParam)]
pub struct CanvasRenderParams<'w,'s> {
    pub(crate) update_render2ds:Query<'w,'s,Entity,Changed<UIRender2D>>,
    pub(crate) render2d:Query<'w,'s,&'static UIRender2D>,
    pub(crate) canvases:Query<'w,'s,&'static mut Canvas>,
    pub(crate) parents:Query<'w,'s,&'static Parent>,
    pub(crate) zorders:Query<'w,'s,&'static ZOrder>,
    pub(crate) transforms:Query<'w,'s,&'static Transform>,
    pub(crate) children:Query<'w,'s,&'static Children>,
    pub(crate) meshes:ResMut<'w,Assets<Mesh>>,
    pub(crate) materials:ResMut<'w,Assets<Material>>,
    pub(crate) asset_server:Res<'w,AssetServer>,
    pub(crate) commands:Commands<'w,'s>,
    pub(crate) ui_roots:ResMut<'w,UIRenderRoot>,
    pub(crate) time:Res<'w,Time>,
}

pub fn update_canvas_render(mut params:CanvasRenderParams) {
    for del_entity in params.ui_roots.despawn_next_frame.drain(..) {
        params.commands.entity(del_entity).despawn();
    }


    let mut changed_canvas:HashSet<Entity> = HashSet::default();
    for entity in params.update_render2ds.iter() {
        if let Some(canvas_entity) = find_canvas(entity, &params.parents, &params.canvases) {
            changed_canvas.insert(canvas_entity);
        }
    }

    for entity in changed_canvas {
        Canvas::update_drawcall(entity,
             &params.children,
             &mut params.render2d,
             &mut params.canvases,
             &params.zorders,
             &params.transforms,
             &params.parents,
             &mut params.meshes,
             &mut params.materials,
             &mut params.commands,
             &mut params.ui_roots,
             &params.asset_server);
    }    
}

fn find_top_canvas(entity:Entity,parents:&Query<&Parent>,canvases:&Query<&Canvas>) -> Option<Entity> {
    let mut cur_entity = Some(entity);
    let mut last_canvas:Option<Entity> = None;
    while let Some(entity) = cur_entity {
        if canvases.contains(entity) {
            last_canvas = Some(entity);
        }
        if let Ok(parent) = parents.get(entity) {
            cur_entity = Some(parent.0);
        } else {
            cur_entity = None;
        }   
    }
    last_canvas
}

fn find_canvas(entity:Entity,parents:&Query<&Parent>,canvases:&Query<&mut Canvas>) -> Option<Entity> {
    let mut cur_entity = Some(entity);
    while let Some(entity) = cur_entity {
        if canvases.contains(entity) {
            return Some(entity);
        }
        if let Ok(parent) = parents.get(entity) {
            cur_entity = Some(parent.0);
        } else {
            cur_entity = None;
        }   
    }
    None
}

