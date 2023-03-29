use std::{sync::Arc, collections::HashSet};
use fixedbitset::FixedBitSet;
use seija_core::{log, time::Time};
use bevy_ecs::{world::World, system::{Resource, SystemParam, Query, Commands, Res, ResMut}, prelude::Entity, query::{Or, Changed, QueryState}};
use seija_asset::{AssetServer, Assets};
use seija_render::{material::{MaterialDefineAsset, MaterialDef}, resource::{Texture, Mesh}};
use seija_transform::hierarchy::{Parent, Children};
use spritesheet::SpriteSheet;

use crate::{components::{sprite::Sprite, rect2d::Rect2D, canvas::{Canvas, ZOrder}}, render::UIRender2D};

#[derive(Resource)]
pub struct UIRenderRoot {
    baseui:Arc<MaterialDef>,
}

pub(crate) fn on_ui_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
    let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
    let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
    //常驻
    h_baseui.forget();
    world.insert_resource(UIRenderRoot {
        baseui:arc_mat_define
    });
}

#[derive(SystemParam)]
pub struct RenderMeshParams<'w,'s> {
    pub(crate) update_sprites:Query<'w,'s,Entity,Or<(Changed<Sprite>,Changed<Rect2D>)>>,
    pub(crate) render2d:Query<'w,'s,&'static mut UIRender2D>,
    pub(crate) sprites:Query<'w,'s,(&'static Sprite,&'static Rect2D)>,
    pub(crate) spritesheets:Res<'w,Assets<SpriteSheet>>,
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

    //刷新ZOrder
    if !top_changed_canvas.is_empty() {
        for top_entity in top_changed_canvas {
            let start_z = params.zorders.get(top_entity).map(|z| z.value).unwrap_or(0);
            ZOrder::update(start_z,top_entity, &mut params.zorders, &params.children,&mut params.commands);
        }
    }

}

#[derive(SystemParam)]
pub struct CanvasRenderParams<'w,'s> {
    pub(crate) update_render2ds:Query<'w,'s,Entity,Changed<UIRender2D>>,
    pub(crate) render2d:Query<'w,'s,&'static UIRender2D>,
    pub(crate) spritesheets:Res<'w,Assets<SpriteSheet>>,
    pub(crate) canvases:Query<'w,'s,&'static mut Canvas>,
    pub(crate) parents:Query<'w,'s,&'static Parent>,
    pub(crate) zorders:Query<'w,'s,&'static ZOrder>,
    pub(crate) children:Query<'w,'s,&'static Children>,
    pub(crate) meshes:ResMut<'w,Assets<Mesh>>,
    pub(crate) commands:Commands<'w,'s>,
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
             &mut params.meshes,
             &mut params.commands);
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