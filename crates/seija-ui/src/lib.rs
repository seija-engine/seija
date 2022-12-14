use std::{collections::{HashSet, HashMap}, sync::Arc};
use bevy_ecs::{entity::Entities, schedule::RunOnce};
use components::{sprite::Sprite, panel::Panel, rect2d::Rect2D};
use mesh2d::Mesh2D;
use root_render::RootRender;
use seija_app::{IModule, App};
use seija_asset::{Assets, AssetServer, Handle};
use seija_core::{CoreStage, StartupStage}; 
use seija_app::ecs::prelude::*;
use seija_render::{resource::Mesh, material::{MaterialDefineAsset, MaterialDef, Material}};
use seija_transform::{hierarchy::{Parent, Children}, Transform, TransformLabel};
pub mod types;
mod sprite_alloc;
pub mod components;
pub mod mesh2d;
mod root_render;
use crate::components::IBuildMesh2D;
pub use sprite_alloc::system::update_sprite_alloc_render;
pub use sprite_alloc::alloc::SpriteAllocator;

#[derive(Clone, Copy,Hash,Debug,PartialEq, Eq,StageLabel)]
pub enum UIStageLabel {
    AfterStartup
}

pub struct UIModule;

impl IModule for UIModule {
    fn init(&mut self,app:&mut App) {
        app.world.insert_resource(SpriteAllocator::new());
        app.init_resource::<UIRootDatas>();
        app.schedule.add_stage_after(CoreStage::Startup, UIStageLabel::AfterStartup, 
                                     SystemStage::single(on_after_start.exclusive_system()).with_run_criteria(RunOnce::default()));
        app.add_system(CoreStage::PostUpdate, update_render_system.after(TransformLabel::Propagate));
    }
}

#[derive(Default)]
struct UIRootDatas {
   baseui:Option<Arc<MaterialDef>>,
   pub(crate) renders:HashMap<Entity,RootRender>
}


fn on_after_start(world:&mut World) {
   let server = world.get_resource::<AssetServer>().unwrap().clone();
   
   let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
   h_baseui.forget(); //常驻

   let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
   let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
   if let Some(mut ui_data) = world.get_resource_mut::<UIRootDatas>() {
      ui_data.baseui = Some(arc_mat_define);
   }
}

fn update_render_system(mut sprites:Query<(Entity,&mut Sprite)>,
                        mut panels:Query<(Entity,&mut Panel)>,
                        parents: Query<(Entity,&Parent)>,
                        childrens: Query<&Children>,
                        infos:Query<(Entity,&Transform,&Rect2D)>,
                        sprite_alloc:Res<SpriteAllocator>,
                        mut ui_datas:ResMut<UIRootDatas>,
                        entities:&Entities,
                        mut commands:Commands,
                        mut meshs:ResMut<Assets<Mesh>>,
                        mut materials:ResMut<Assets<Material>>,
                        server:Res<AssetServer>) {
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
    }
}


fn calc_top_panel(entity:&Entity,panels:&mut Query<(Entity,&mut Panel)>,parents:&Query<(Entity,&Parent)>) -> Option<Entity> {
    let mut cur_entity:Entity = *entity;
    let mut top_panel_entity:Option<Entity> = None;
    while let Ok((_,parent)) = parents.get(cur_entity) {
        cur_entity = parent.0;
        if let Ok(mut panel) = panels.get_mut(cur_entity)  {
            top_panel_entity = Some(cur_entity);
            panel.1.is_dirty = true;
        }
    }
    top_panel_entity
}

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
                        let mesh2d = sprite.build(rect2d, uv, &mat);
                        sprite.is_dirty = false;
                        return Some(mesh2d);
                    }
                }
            }
        }
    }
    None
}



