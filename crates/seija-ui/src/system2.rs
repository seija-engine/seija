use std::collections::{HashSet, HashMap};
use seija_core::log;
use bevy_ecs::system::{Res,SystemParam};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
use seija_transform::events::HierarchyEvent;
use seija_transform::hierarchy::{Parent,PreviousParent, Children};
use crate::SpriteAllocator;
use crate::components::panel::Panel;
use crate::components::rect2d::Rect2D;
use crate::components::sprite::Sprite;
use crate::render_info::PanelInfo;
use crate::types::UIZOrder;

#[derive(Default)]
pub struct UIRenderData {
    entity2panel:HashMap<Entity,Entity>,
    render_panels:HashMap<Entity,PanelInfo>
}

#[derive(SystemParam)]
pub struct SystemParams<'w,'s> {
    sprite_alloc:Res<'w,SpriteAllocator>,
    update_sprites:Query<'w,'s,Entity,Or<(Changed<Sprite>, Changed<Rect2D>, Changed<Transform>)>>,
    update_panels:Query<'w,'s,Entity,Or<(Changed<Panel>, Changed<Rect2D>, Changed<Transform>)>>,
    remove_sprites:RemovedComponents<'w,Sprite>,
    remove_panels:RemovedComponents<'w,Panel>,
    panels:Query<'w,'s,(Entity,&'static Panel)>,
    parents:Query<'w,'s,(Entity,&'static Parent,Option<&'static PreviousParent>)>,
    zorders:Query<'w,'s,&'static mut UIZOrder>,
    childrens:Query<'w,'s,(Entity,&'static Children)>,
    sprites:Query<'w,'s,(Entity,(&'static Sprite, &'static Rect2D, &'static Transform))>,
    events:EventReader<'w,'s,HierarchyEvent>,
    render_data:Local<'s,UIRenderData>,
    commands: Commands<'w,'s>
}

impl<'w,'s> SystemParams<'w,'s> {
    pub fn get_top_panel(&self,entity:Entity) -> Option<Entity> {
        let mut cur_entity: Entity = entity;
        let mut top_panel_entity: Option<Entity> = None;
        while let Ok(parent) = self.parents.get(cur_entity) {
            cur_entity = parent.1.0;
            if self.panels.get(cur_entity).is_ok() {
                top_panel_entity = Some(cur_entity);
            }
        }
        top_panel_entity
    }

    pub fn get_render_parent_panel(&self,entity:Entity) -> Option<Entity> {
        let mut cur_entity: Entity = entity;
        let mut last_panel: Option<Entity> = None;
        while let Ok((_,parent,_)) = self.parents.get(cur_entity) {
            cur_entity = parent.0;
            if let Ok((e, panel)) = self.panels.get(cur_entity) {
                last_panel = Some(e);
                if !panel.is_static {
                    return Some(e);
                }
            }
        }
        if let Some(last) = last_panel.take() {
            return Some(last);
        }
        None
    }

    pub fn visit_mut<F>(&mut self,entity:Entity,f:&mut F) where F:FnMut(Entity,&mut Self) {
        f(entity,self);
        if let Ok(childrens) = self.childrens.get(entity) {
            let children_lst = childrens.1.iter().cloned().collect::<Vec<_>>();
            for child_entity in children_lst {
                self.visit_mut(child_entity, f);
            }
        }
    }
}

//处理Sprite增删改，处理Panel增删改，处理Entity层级变化
pub(crate) fn ui_render_system(mut params:SystemParams) {
    UpdateZOrders::default().run(&mut params);
    let mut dirty_sprites:HashSet<Entity> = HashSet::default();
    let mut dirty_panels:HashSet<Entity> = HashSet::default();
    //Sprite 增 + 改
    for sprite_entity in params.update_sprites.iter() {
        dirty_sprites.insert(sprite_entity);
        if let Some(entity) = params.get_render_parent_panel(sprite_entity) {
            dirty_panels.insert(entity);
        }
    }
    //Sprite 删
    for remove_entity in params.remove_sprites.iter() {
        if let Some(panel_entity) = params.render_data.entity2panel.remove(&remove_entity) {
            if params.render_data.render_panels.contains_key(&panel_entity) {
                dirty_panels.insert(panel_entity);
            }
        }
    }
   
    //Panel 增 + 改
    for update_panel in params.update_panels.iter() {
        if let Ok((_,panel)) = params.panels.get(update_panel) {
            if panel.is_static {
                //静态Panel
                //TODO
            } else { 
                //动态Panel
                //TODO
            }
        }
    }
    //Panel 删
    for remove_panel in params.remove_panels.iter() {
        if let Some(panel_info) = params.render_data.render_panels.remove(&remove_panel) {
            //TODO
        }
    }
    //Entity层级变化
    for event in params.events.iter() {
        match event {
            HierarchyEvent::ParentChange { entity , old_parent, new_parent } => {

            }
        }
    }

    
}


#[derive(Default)]
struct UpdateZOrders {
   pub(crate) dirty_top_panels:HashSet<Entity>,
}

impl UpdateZOrders {
    pub fn run(mut self,params:&mut SystemParams) {
        for entity in params.update_sprites.iter() {
            if let Some(e) = params.get_top_panel(entity) {
                self.dirty_top_panels.insert(e);
            }
        }
        
        for panel_entity in self.dirty_top_panels {
            let mut now_index = 1;
            if let Ok(panel_zorder) = params.zorders.get(panel_entity) {
                now_index = panel_zorder.value;
            }
            params.visit_mut(panel_entity, &mut |entity,fn_params| {
               if let Ok(mut zorder) = fn_params.zorders.get_mut(entity) {
                  zorder.last = zorder.value;
                  zorder.value = now_index;
               } else {
                  fn_params.commands.entity(entity).insert(UIZOrder {
                        last: -1,
                        value: now_index
                  });
               }
               now_index += 1;
            });
        }
    }
}