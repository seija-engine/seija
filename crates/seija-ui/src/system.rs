use std::sync::Arc;
use seija_core::{log, math::Mat4};
use bevy_ecs::{world::World, system::{Resource, SystemParam, Query, Commands, Res}, prelude::Entity, query::{Or, Changed}};
use seija_asset::{AssetServer, Assets};
use seija_render::{material::{MaterialDefineAsset, MaterialDef}, resource::Texture};
use spritesheet::SpriteSheet;

use crate::{components::{sprite::Sprite, rect2d::Rect2D}, render::UIRender2D};

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
}

pub fn update_render_mesh_system(mut params:RenderMeshParams) {
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
        }
    }
}

#[derive(SystemParam)]
pub struct CanvasRenderParams<'w,'s> {
    pub(crate) update_renders:Query<'w,'s,Entity,Changed<UIRender2D>>,
    pub(crate) render2d:Query<'w,'s,&'static UIRender2D>,
    pub(crate) spritesheets:Res<'w,Assets<SpriteSheet>>,
    pub(crate) commands:Commands<'w,'s>,
}

pub fn update_canvas_render(params:CanvasRenderParams) {

}
