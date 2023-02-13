use std::collections::HashSet;
use bevy_ecs::{system::{SystemParam, Query, RemovedComponents, Res}, prelude::{Entity, EventReader}, query::Changed};
use seija_core::{math::{Vec2, Vec3}, window::AppWindow};
use seija_transform::{events::HierarchyEvent, hierarchy::{Parent, Children}, Transform};
use crate::components::rect2d::Rect2D;
use super::{types::LayoutElement, measure, arrange::{arrange_layout_element, ArrangeXY}};

#[derive(SystemParam)]
pub struct LayoutParams<'w,'s> {
    pub(crate) update_elems:Query<'w,'s,Entity,Changed<LayoutElement>>,
    pub(crate) events:EventReader<'w,'s,HierarchyEvent>,
    pub(crate) removes:RemovedComponents<'w,LayoutElement>,
    pub(crate) parents:Query<'w,'s,(Entity,&'static Parent)>,
    pub(crate) childrens:Query<'w,'s,&'static Children>,
    pub(crate) elems:Query<'w,'s,&'static LayoutElement>,
    pub(crate) rect2ds:Query<'w,'s,&'static mut Rect2D>,
    pub(crate) trans:Query<'w,'s,&'static mut Transform>,
    pub(crate) window:Res<'w,AppWindow>
}


pub fn ui_layout_system(params:LayoutParams) {
    let dirty_layouts = collect_dirty(&params);
    for elem_entity in dirty_layouts {
       //这里只会修改Element的属性，所以是安全的
       if let Ok(element) = params.elems.get(elem_entity) {
          let x = size_request_x(elem_entity, &params);
          let y = size_request_y(elem_entity, &params);
          let request_size:Vec2 = Vec2::new(x, y);
          measure::measure_layout_element(elem_entity,request_size,&element,&params);
         
          let origin = origin_request(elem_entity, &params);
          let parent_size = if let Ok(size) = params.parents.get(elem_entity).and_then(|p| params.rect2ds.get(p.1.0)) {
             Vec2::new(size.width, size.height)
          } else {
            Vec2::new(params.window.width() as f32, params.window.height() as f32)
          };
          arrange_layout_element(elem_entity, element, origin,parent_size,ArrangeXY::ALL,&params);
          
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

fn origin_request(entity:Entity,params:&LayoutParams) -> Vec2 {
    if let Ok(parent) = params.parents.get(entity) {
        if let Ok(rect) = params.rect2ds.get(parent.1.0) {
            Vec2::new(rect.left(), rect.top())
        } else {
            origin_request(parent.1.0, params)
        }
    } else {
        let w = params.window.width() as f32;
        let h = params.window.height() as f32;
        Vec2::new(-w * 0.5f32,h * 0.5f32)
    }
}