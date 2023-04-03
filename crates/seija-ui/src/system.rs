use std::{sync::Arc, collections::{HashSet, HashMap}};
use seija_core::{log, time::Time};
use bevy_ecs::{world::World, system::{Resource, SystemParam, Query, Commands, Res, ResMut}, prelude::Entity, query::{Or, Changed}};
use seija_asset::{AssetServer, Assets, Handle};
use seija_render::{material::{MaterialDefineAsset, MaterialDef, Material}, resource::{ Mesh}};
use seija_transform::{hierarchy::{Parent, Children}, Transform};
use spritesheet::SpriteSheet;
use glyph_brush::{GlyphBrush, GlyphBrushBuilder, FontId, ab_glyph::PxScale, Section, BrushAction};
use crate::{components::{sprite::Sprite, rect2d::Rect2D, canvas::{Canvas, ZOrder}}, render::UIRender2D, mesh2d::Vertex2D, text::{Text, Font}};
use glyph_brush::GlyphVertex;
#[derive(Resource)]
pub struct UIRenderRoot {
    pub(crate) baseui:Arc<MaterialDef>,
    pub(crate) text_brush:GlyphBrush<Vec<Vertex2D>>,
    font_caches:HashMap<Handle<Font>,FontId>,
}

pub(crate) fn on_ui_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
    let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
    let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
    //常驻
    h_baseui.forget();
    world.insert_resource(UIRenderRoot {
        baseui:arc_mat_define,
        text_brush:GlyphBrushBuilder::using_fonts(vec![]).cache_redraws(false).initial_cache_size((1024, 1024)).build(),
        font_caches:HashMap::default()
    });
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
    pub(crate) ui_roots:ResMut<'w,UIRenderRoot>,
    pub(crate) commands:Commands<'w,'s>,
    pub(crate) time:Res<'w,Time>,
    pub(crate) canvases:Query<'w,'s,&'static Canvas>,
    pub(crate) parents:Query<'w,'s,&'static Parent>,
    pub(crate) zorders:Query<'w,'s,&'static mut ZOrder>,
    pub(crate) children:Query<'w,'s,&'static Children>,
}

pub fn update_render_mesh_system(mut params:RenderMeshParams) {
    let mut top_changed_canvas:HashSet<Entity> = HashSet::default();

    //更新Sprite的Mesh
    for entity in params.update_sprites.iter() {
        if let Ok((sprite,rect)) = params.sprites.get(entity) {
            if let Some(atlas) = sprite.atlas.as_ref().map(|h| params.spritesheets.get(&h.id).unwrap()) {
               if let Some(render_2d) = sprite.build_render(rect,atlas) {
                    if let Ok(mut render) = params.render2d.get_mut(entity) {
                        render.mesh2d = render_2d.mesh2d;
                        render.texture = render_2d.texture;
                    } else {
                        params.commands.entity(entity).insert(render_2d);
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
                let section = Section::default().add_text(text);
                params.ui_roots.text_brush.queue(section);
            }

            let action = params.ui_roots.text_brush.process_queued(|r,bytes| {
                seija_core::log::error!("bytes:{:?} rect:{:?}",bytes.len(),r);
            },glyph_to_mesh);
            match action {
                Ok(BrushAction::Draw(verts)) => {
                   let mesh2d = Text::build_mesh(verts);
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
    pub(crate) ui_roots:Res<'w,UIRenderRoot>,
    pub(crate) time:Res<'w,Time>,
}

pub fn update_canvas_render(mut params:CanvasRenderParams) {
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
             &params.ui_roots,
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

