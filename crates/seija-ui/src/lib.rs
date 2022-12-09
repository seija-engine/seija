use std::collections::HashSet;
use components::{sprite::Sprite, panel::Panel};
use seija_app::{IModule, App};
use seija_core::CoreStage; 
use seija_app::ecs::prelude::*;
use seija_transform::hierarchy::{Parent, Children};
pub mod types;
mod sprite_alloc;
pub mod components;
pub mod mesh2d;

pub struct UIModule;

impl IModule for UIModule {
    fn init(&mut self,app:&mut App) {
        app.add_system(CoreStage::Update, update_render_system);
    }
}

fn update_render_system(mut sprites:Query<(Entity,&mut Sprite)>,
                        mut panels:Query<(Entity,&mut Panel)>,
                        parents: Query<(Entity,&Parent)>,
                        childrens: Query<&Children>) {

    let mut top_panels:HashSet<Entity> = HashSet::default();
    for (entity,sprite) in sprites.iter() {
        if sprite.is_dirty {
            if let Some(top_entity) = calc_top_panel(&entity,&mut panels,&parents) {
                top_panels.insert(top_entity);
            }
        }
    }

    for panel_entity in top_panels.iter() {
        rebuild_mesh(*panel_entity,&mut panels,&childrens,&sprites);
    }
    sprites.iter_mut().for_each(|mut v| v.1.is_dirty = false);
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

fn rebuild_mesh(panel_entity:Entity,
                panels:&mut Query<(Entity,&mut Panel)>,
                childrens: &Query<&Children>,
                sprites:&Query<(Entity,&mut Sprite)>) {
    if let Ok((entity,panel)) = panels.get_mut(panel_entity) {
       if let Ok(children) = childrens.get(entity) {
          for child in children.iter() {
            //sprite
            if let Ok(sprite) = sprites.get(*child) {
                if sprite.1.is_dirty {

                }
            }

          }
       }
    }
} 