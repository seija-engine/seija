use bevy_ecs::prelude::Entity;
use std::{collections::HashSet, sync::Arc};

use crate::{ types::UIZOrder};

use seija_core::log;
use super::components::{sprite::Sprite, panel::Panel, rect2d::Rect2D};

use seija_asset::{Assets, AssetServer};
use seija_app::ecs::prelude::*;
use seija_render::{ material::{MaterialDefineAsset, MaterialDef}};
use seija_transform::{hierarchy::{Parent, Children}, Transform};
pub(crate) struct RootRender {
    pub panel_entity:Entity,
    pub render_entity:Entity
}


#[derive(Default)]
pub(crate) struct UISystemData {
   baseui:Option<Arc<MaterialDef>>,
}


pub(crate) fn on_after_start(world:&mut World) {
   let server = world.get_resource::<AssetServer>().unwrap().clone();
   
   let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
   h_baseui.forget(); //常驻

   let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
   let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
   if let Some(mut ui_data) = world.get_resource_mut::<UISystemData>() {
      ui_data.baseui = Some(arc_mat_define);
   }
}



/*example 1
Panel0                ZOrder(0)
   Sprite0            ZOrder(1)
   Panel1(dynamic)    ZOrder(2)
     Sprite1          ZOrder(3)
   Sprite2            ZOrder(4)
   Panel2             ZOrder(5)
     Sprite3          ZOrder(6)
   Sprite4            ZOrder(7)

to

Drawcall(1)[Panel0][Sprite0] max-0.1
Drawcall(2)[Panel1][Sprite1] max-0.3
Drawcall(3)[Panel1][Sprite2,Sprite3,Sprite4] max-0.7

   1. 获取所有dirty Sprite
   2. 计算出所有变动的TopPanel
   3. 对TopPanel的ZOrder进行重排
   4. 进行UIDrawcall的比对更新
   5. 根据ZOrder进行UIDrawcall的Z位置调整
*/


pub(crate) fn update_render_system(world:&mut World) {
   let mut dirty_sprites = world.query_filtered::<Entity,Or<(Changed<Sprite>,Changed<Rect2D>,Changed<Transform>)>>();
   let mut panels = world.query::<(Entity,&Panel)>();
   let mut parents = world.query::<&Parent>();
   let mut zorders = world.query::<&mut UIZOrder>();
   let mut childrens = world.query::<&Children>();
    //计算出所有变动的TopPanel
    let mut dirty_panels:HashSet<Entity> = HashSet::default();
    for e in dirty_sprites.iter(world) {
      log::info!("dirty_sprites iter:{:?}",e);
      if let Some(e) = calc_top_panel(world,e,&mut panels,&mut parents) {
         dirty_panels.insert(e); 
      }
    }
    
    //对TopPanel的ZOrder进行重排
    fill_ui_zorders(world,dirty_panels,&mut zorders,&mut childrens);

    //开始比对更新Drawcall

}

fn calc_top_panel(world:&World,entity:Entity,panels:&mut QueryState<(Entity,&Panel)>,parents:&mut QueryState<&Parent>) -> Option<Entity> {
   let mut cur_entity:Entity = entity;
   let mut top_panel_entity:Option<Entity> = None;
   while let Ok(parent) = parents.get(world,cur_entity) {
      cur_entity = parent.0;
      if panels.get(world,cur_entity).is_ok() {
         top_panel_entity = Some(cur_entity);
      }
   }
   top_panel_entity
}

fn fill_ui_zorders(world:&mut World,panels:HashSet<Entity>,zorders:&mut QueryState<&mut UIZOrder>,childrens:&mut QueryState<&Children>) {
   for panel_entity in panels.iter() {
      if let Ok(zorder) = zorders.get(world,*panel_entity) {
          let value = zorder.value;
          _fill_ui_zorders(world,*panel_entity, value + 1, zorders, childrens);
      } else {
         _fill_ui_zorders(world,*panel_entity, 1, zorders, childrens);
      }
   }
}

fn _fill_ui_zorders(world:&mut World,entity:Entity,number:i32,zorders:&mut QueryState<&mut UIZOrder>,childrens:&mut QueryState<&Children>) {
   let mut now_index = number;
   if let Ok(childs) = childrens.get(world,entity) {
      let children_ids = childs.children().iter().map(|v|*v).collect::<Vec<_>>();
      for child_entity in children_ids {
         if let Ok(mut zorder) = zorders.get_mut(world,child_entity) {
            zorder.last = zorder.value;
            zorder.value = now_index;
         } else {
            world.entity_mut(child_entity).insert(UIZOrder {last:-1,value:now_index });
         }
         log::info!("set zorder{:?}={}",child_entity,now_index);
         now_index += 1;
         _fill_ui_zorders(world,child_entity, now_index, zorders, childrens);
      }
      
   }
}