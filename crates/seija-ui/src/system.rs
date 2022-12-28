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
   Sprite2            ZOrder(4)
   Panel2             ZOrder(5)
     Sprite3          ZOrder(6)

Panel10
  Sprite11
*/

/*
   let dirty_lst = scan_dirty;
   let dirty_panels = calc_top_panels(dirty_lst);
   refill_transz(dirty_panels);
   
   let dirty_render_panels = calc_render_panels(dirty_lst);
   gen_panel_mesh(dirty_render_panels);
*/

pub(crate) fn update_render_system(world:&mut World) {
   let mut changed_sprites = world.query_filtered::<(Entity,&Sprite),Or<(Changed<Sprite>,Changed<Rect2D>,Changed<Transform>)>>();
   let mut panels = world.query::<(Entity,&Panel)>();
   let mut parents = world.query::<(Entity,&Parent)>();
   let mut dirty_top_panels:HashSet<Entity> = HashSet::default();
   //收集dirty sprite
   for (entity,_) in changed_sprites.iter(world) {
      log::error!("update changed:{:?}",entity);
      if let Some(v) = calc_top_panel(&world,&entity, &mut panels, &mut parents) {
         dirty_top_panels.insert(v);
      }
   }
   drop(changed_sprites);
   let mut trans = world.query::<&mut Transform>();
   let mut childrens = world.query::<&Children>();
   for top_panel_entity in dirty_top_panels.iter() {
     log::error!("top panel:{:?}",top_panel_entity);
     fill_z_order(world,*top_panel_entity,&mut childrens, &mut trans,0f32);
   }
   

}

const Z_OFFSET:f32 = 0.1f32;

fn fill_z_order(world:&World,entity:Entity,childrens:&mut QueryState<&Children>,trans:&mut QueryState<&mut Transform>,z:f32) {
    if let Ok(childs) = childrens.get(world,entity) {
        for (index,child) in childs.iter().enumerate() {
            if let Ok(mut t) = unsafe { trans.get_unchecked(world,*child) }  {
                log::error!("set {}.z = {}",child.id(),index);
                t.local.position.z = index as f32 * Z_OFFSET;
            }
            fill_z_order(world,*child,childrens,trans,z);
        }
    }
}

fn calc_top_panel(world:&World,entity:&Entity,panels:&mut QueryState<(Entity,&Panel)>,parents:&mut QueryState<(Entity,&Parent)>) -> Option<Entity> {
    let mut cur_entity:Entity = *entity;
    let mut top_panel_entity:Option<Entity> = None;
    while let Ok((_,parent)) = parents.get(world,cur_entity) {
        cur_entity = parent.0;
        if panels.get(world,cur_entity).is_ok() {
            top_panel_entity = Some(cur_entity);
        }
    }
    top_panel_entity
}


/*
fn rebuild_panel_mesh(panel_entity:Entity,
                panels:&mut Query<(Entity,&mut Panel)>,
                childrens: &Query<&Children>,
                sprites:&mut Query<(Entity,&mut Sprite)>,
                infos:&Query<(Entity,&Transform,&Rect2D)>,
                sprite_alloc:&SpriteAllocator) -> Option<Mesh2D> {
    let mut mesh_dic:HashMap<u32,Mesh2D> = HashMap::default();
    if let Ok(children) = childrens.get(panel_entity) {
          for child in children.iter() {
             if let Some(mesh2d) = rebuild_sprite_mesh(sprites, *child, infos, sprite_alloc) {
                mesh_dic.insert(child.id(), mesh2d);
             } else if panels.contains(*child) {
               if let Some(mesh2d) = rebuild_panel_mesh(*child,panels,childrens,sprites,infos,sprite_alloc) {
                  mesh_dic.insert(child.id(), mesh2d);
               }
             }
          }
    }
    if let Ok((_,mut panel)) = panels.get_mut(panel_entity) {
        panel.child_meshs = mesh_dic;
        if let Ok((_,t,rect2d)) = infos.get(panel_entity) {
            panel.is_dirty = false;
            return panel.build_mesh(t,rect2d);
        }
        
    }
    None
}

fn rebuild_sprite_mesh(sprites:&mut Query<(Entity,&mut Sprite)>,
                       child:Entity,infos:&Query<(Entity,&Transform,&Rect2D)>,
                       sprite_alloc:&SpriteAllocator) -> Option<Mesh2D> {
    if let Ok((_,mut sprite)) = sprites.get_mut(child) { 
        if sprite.is_dirty {
            if let Ok((_,t,rect2d)) = infos.get(child) {
                if let Some(sprite_index) = sprite.sprite_index {
                    if let Some(info) = sprite_alloc.get_sprite_info(sprite_index) {
                        let mat = t.global().matrix();
                        let uv = info.uv.clone();
                        
                        let mesh2d = sprite.build(rect2d, uv, &mat,&info.rect);
                        sprite.is_dirty = false;
                        return Some(mesh2d);
                    }
                }
            }
        }
    }
    None
}

 /*
    //calc dirty top top_panels
    let mut dirty_top_panels:HashSet<Entity> = HashSet::default();
    for (entity,sprite) in sprites.iter() {
        if sprite.is_dirty {
            if let Some(top_entity) = calc_top_panel(&entity,&mut panels,&parents) {
                dirty_top_panels.insert(top_entity);
            }
        }
    }
   
    for panel_entity in dirty_top_panels.iter() {
        if let Some(top_mesh) = rebuild_panel_mesh(*panel_entity,&mut panels,&childrens,&mut sprites,&infos,&sprite_alloc) {
            if ui_datas.renders.contains_key(&panel_entity) {
                //update 
            } else {
                //add
                let mesh:Mesh = top_mesh.into();
                let h_mesh = meshs.add(mesh);
               
                let material_def = ui_datas.baseui.as_ref().unwrap();
                let material = Material::from_def(material_def.clone(), &server).unwrap();
                let h_mat = materials.add(material);
                let render_entity = commands.spawn().insert(Transform::default()).insert(h_mesh).insert(h_mat).id();
                ui_datas.renders.insert(*panel_entity, RootRender {
                    panel_entity:*panel_entity,
                    render_entity
                });
            }
        }
    }

    //remove
    for render_id in ui_datas.renders.keys() {
        if !entities.contains(*render_id) {

        }
    }*/
*/
