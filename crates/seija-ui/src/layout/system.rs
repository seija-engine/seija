use std::collections::HashSet;
use bevy_ecs::{system::{SystemParam, Query, RemovedComponents, Res}, prelude::{Entity, EventReader}, query::Changed};
use seija_core::{math::Vec2, window::AppWindow};
use seija_transform::{events::HierarchyEvent, hierarchy::{Parent, Children}};
use crate::components::rect2d::Rect2D;
use super::{types::LayoutElement, measure};

#[derive(SystemParam)]
pub struct LayoutParams<'w,'s> {
    pub(crate) update_elems:Query<'w,'s,Entity,Changed<LayoutElement>>,
    pub(crate) events:EventReader<'w,'s,HierarchyEvent>,
    pub(crate) removes:RemovedComponents<'w,LayoutElement>,
    pub(crate) parents:Query<'w,'s,(Entity,&'static Parent)>,
    pub(crate) childrens:Query<'w,'s,&'static Children>,
    pub(crate) elems:Query<'w,'s,&'static LayoutElement>,
    pub(crate) rect2ds:Query<'w,'s,&'static mut Rect2D>,
    pub(crate) window:Res<'w,AppWindow>
}


pub fn ui_layout_system(params:LayoutParams) {
    let dirty_layouts = collect_dirty(&params);
    for elem_entity in dirty_layouts {
       //这里只会修改Element的属性，所以是安全的
       if let Ok(mut element) = params.elems.get(elem_entity) {
          let x = size_request_x(elem_entity, &params);
          let y = size_request_y(elem_entity, &params);
          let request_size:Vec2 = Vec2::new(x, y);
          measure::measure_layout_element(elem_entity,request_size,&mut element,&params);
       }
    }
}



/////计算需要重布局的Element
fn collect_dirty(params:&LayoutParams) -> HashSet<Entity> {
    let mut layouts = HashSet::default();
    for entity in params.update_elems.iter() {
        let dirty_entity = get_top_elem_dirty(entity, params);
        layouts.insert(dirty_entity);
    }
    layouts
}

fn get_top_elem_dirty(entity:Entity,params:&LayoutParams) -> Entity {
    let mut cur_entity = entity;
    let mut last_elem_entity = cur_entity;
    while let Ok(parent) = params.parents.get(cur_entity) {
        let parent_entity = parent.1.0;
        if let Ok(element) = params.elems.get(parent_entity) {
            last_elem_entity = parent_entity;
            if let Ok(cur_element) = params.elems.get(cur_entity) {
               if !element.is_invalid_measure(cur_element) {
                  return cur_entity;
               }
            }
        }
        cur_entity = parent_entity;   
    }
    last_elem_entity
}


fn size_request_x(entity:Entity,params:&LayoutParams) -> f32 {
    if let Ok(elem) = params.elems.get(entity) {
        if elem.common.size.x >= 0f32 {
            return elem.common.size.x - elem.common.padding.horizontal();
        }
        if let Ok(parent) = params.parents.get(entity) {
           return size_request_x(parent.1.0, params);
        } else {
            return params.window.width() as f32;
        }
    } else {
        if let Ok(parent) = params.parents.get(entity) {
            return size_request_x(parent.1.0, params);
        } else {
            return params.window.width() as f32;
        }
    }
    
}

fn size_request_y(entity:Entity,params:&LayoutParams) -> f32 {
    if let Ok(elem) = params.elems.get(entity) {
        if elem.common.size.y >= 0f32 {
            return elem.common.size.y - elem.common.padding.vertical();
        }
        if let Ok(parent) = params.parents.get(entity) {
           return size_request_y(parent.1.0, params);
        } else {
            return params.window.height() as f32;
        }
    } else {
        if let Ok(parent) = params.parents.get(entity) {
            return size_request_y(parent.1.0, params);
        } else {
            return params.window.height() as f32;
        }
    }
}

