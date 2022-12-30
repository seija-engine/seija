use bevy_ecs::prelude::Entity;
use fixedbitset::FixedBitSet;
use std::{collections::{HashSet, HashMap}, sync::Arc};
use bevy_ecs::{entity::Entities};
use crate::SpriteAllocator;
use crate::components::IBuildMesh2D;
use seija_core::log;
use super::components::{sprite::Sprite, panel::Panel, rect2d::Rect2D};
use super::mesh2d::Mesh2D;
use seija_asset::{Assets, AssetServer};
use seija_app::ecs::prelude::*;
use seija_render::{resource::Mesh, material::{MaterialDefineAsset, MaterialDef, Material}};
use seija_transform::{hierarchy::{Parent, Children}, Transform};
pub(crate) struct RootRender {
    pub panel_entity:Entity,
    pub render_entity:Entity
}


#[derive(Default)]
pub(crate) struct UIRootDatas {
   baseui:Option<Arc<MaterialDef>>,
   pub(crate) renders:HashMap<Entity,RootRender>
}


pub(crate) fn on_after_start(world:&mut World) {
   let server = world.get_resource::<AssetServer>().unwrap().clone();
   
   let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
   h_baseui.forget(); //常驻

   let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
   let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
   if let Some(mut ui_data) = world.get_resource_mut::<UIRootDatas>() {
      ui_data.baseui = Some(arc_mat_define);
   }
}



/*
Panel0                ZOrder(0)
   Sprite0            ZOrder(1)
   Panel1(dynamic)    ZOrder(2)
     Sprite1          ZOrder(3)
   Panel2             ZOrder(4)
     Sprite3          ZOrder(5)
   Sprite2            ZOrder(6)

Panel10
  Sprite11

   1. 获取所有dirty Sprite
   2. 计算出所有变动的TopPanel
   3. 对TopPanel的ZOrder进行重排
   4. 进行UIDrawcall生成和更新
   5. 根据ZOrder进行UIDrawcall的Z位置调整
*/

pub(crate) fn update_render_system(
   dirty_sprites:Query<(Entity,&Sprite),Or<(Changed<Sprite>,Changed<Rect2D>,Changed<Transform>)>>,
   parents:Query<&Parent>,
   childrens:Query<&Children>
) {  
    
}