use std::collections::HashSet;
use bevy_ecs::{system::{SystemParam, Query, Res}, prelude::{Entity, EventReader}, query::{Changed, ChangeTrackers}};
use seija_core::{math::Vec2, window::AppWindow, FrameDirty, time::Time, info::EStateInfo};
use seija_transform::{events::HierarchyEvent, hierarchy::{Parent, Children}, Transform};
use seija_winit::event::WindowResized;
use crate::{components::{rect2d::Rect2D, ui_canvas::UICanvas}, ffi::PostLayoutProcess};
use super::{types::{LayoutElement, FreeLayoutItem}, measure, arrange::{arrange_layout_element, 
    ArrangeXY,arrange_freeitem_and_set_pos,arrange_freeitem_layout}, comps::FlexItem};

#[derive(SystemParam)]
pub struct LayoutParams<'w,'s> {
    pub(crate) update_elems:Query<'w,'s,Entity,Changed<LayoutElement>>,
    pub(crate) update_freeitems:Query<'w,'s,Entity,Changed<FreeLayoutItem>>,
    pub(crate) events:EventReader<'w,'s,HierarchyEvent>,
    pub(crate) parents:Query<'w,'s,(Entity,&'static Parent)>,
    pub(crate) childrens:Query<'w,'s,&'static Children>,
    pub(crate) elems:Query<'w,'s,&'static LayoutElement>,
    pub(crate) rect2ds:Query<'w,'s,&'static mut Rect2D>,
    pub(crate) trans:Query<'w,'s,&'static mut Transform>,
    pub(crate) flexitems:Query<'w,'s,&'static FlexItem>,
    pub(crate) freeitems:Query<'w,'s,&'static FreeLayoutItem>,
    pub(crate) window:Res<'w,AppWindow>,
    pub(crate) resize_events:EventReader<'w,'s,WindowResized>,
    pub(crate) ui_canvas:Query<'w,'s,(Entity,&'static UICanvas)>,
    pub(crate) frame_dirty:Query<'w,'s,&'static mut FrameDirty>,
    pub(crate) time:Res<'w,Time>,
    pub(crate) post_process:Option<Res<'w,PostLayoutProcess>>,
}

pub fn ui_layout_system(mut params:LayoutParams,_info_states:Query<&EStateInfo>,
                        changed_states:Query<(Entity,ChangeTrackers<EStateInfo>)>) {
    let mut dirty_layouts = collect_dirty(&mut params);
    for (dirty_entity,tracker) in changed_states.iter() {
        if !tracker.is_added() && tracker.is_changed() {
            dirty_layouts.insert(dirty_entity);
        }
    }
    let mut changed_entity_lst:Vec<Entity> = Vec::new();
    for elem_entity in dirty_layouts {
        process_entity_layout(elem_entity,&mut params,&mut changed_entity_lst)
    }
    arrange_freeitem_layout(&mut params);

    if changed_entity_lst.len() > 0 {
       let cur_frame = params.time.frame();
       let mut iter = params.frame_dirty.iter_many_mut(&changed_entity_lst);
       while let Some(mut fd) = iter.fetch_next() { 
          fd.frame = cur_frame;
          fd.index = 0;
       }
    }

    if let Some(post_layout_fn) = params.post_process.as_ref().map(|v| v.0) {
        let mut dirty_entitys:Vec<u64> = vec![];
        let mut step:i32 = 0;
        let lst_ptr = std::ptr::addr_of_mut!(dirty_entitys);
        while step < 10 {
            post_layout_fn(step,lst_ptr);
            if dirty_entitys.is_empty() { break; }
            changed_entity_lst.clear();
            for dirty_id in dirty_entitys.iter() {
                let cur_entity = Entity::from_bits(*dirty_id);
                process_entity_layout(cur_entity,&mut params,&mut changed_entity_lst);

                arrange_freeitem_and_set_pos(cur_entity,&mut params);
            }
            step += 1;
            dirty_entitys.clear();

            let mut iter = params.frame_dirty.iter_many_mut(&changed_entity_lst);
            while let Some(mut fd) = iter.fetch_next() { 
                fd.index = step;
            }
        }
    }
}

fn process_entity_layout(elem_entity:Entity,params:&mut LayoutParams,changed_entity_lst:&mut Vec<Entity>) {
    //这里只会修改Element的属性，所以是安全的
    if let Ok(element) = params.elems.get(elem_entity) {
        let x = size_request_x(elem_entity, &params);
        let y = size_request_y(elem_entity, &params);
        let request_size:Vec2 = Vec2::new(x, y);
        measure::measure_layout_element(elem_entity,request_size,&params);
       
        let origin = origin_request(elem_entity, &params);
        let parent_size = if let Ok(size) = params.parents.get(elem_entity).and_then(|p| params.rect2ds.get(p.1.0)) {
           Vec2::new(size.width, size.height)
        } else {
          Vec2::new(params.window.width() as f32, params.window.height() as f32)
        };
        arrange_layout_element(elem_entity, element, origin,parent_size,ArrangeXY::ALL,&params,changed_entity_lst);
     }
}

/////计算需要重布局的Element
fn collect_dirty(params:&mut LayoutParams) -> HashSet<Entity> {
    let mut layouts = HashSet::default();
    if !params.resize_events.is_empty() {
        for (entity,_) in params.ui_canvas.iter() {
           if let Ok(canvas_childs) = params.childrens.get(entity) {
                for child in canvas_childs.iter() {
                     layouts.insert(*child);
                }
           }
        }
        return layouts;
    }
    
    for entity in params.update_elems.iter() {
        let dirty_entity = get_top_elem_dirty(entity, params);
        layouts.insert(dirty_entity);
    }
    let mut remove_parents:Vec<Entity> = vec![];
    for event in params.events.iter() {
        match event {
            HierarchyEvent::Delete(_,_,parent) => {
                if let Some(parent) = parent {
                    remove_parents.push(*parent);
                }
            }
            _ => {}
        }
    }

    for entity in remove_parents.drain(..) {
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
        if let Ok(parent_element) = params.elems.get(parent_entity) {
            last_elem_entity = parent_entity;
            if let Ok(cur_element) = params.elems.get(cur_entity) {
               if !parent_element.is_invalid_measure(cur_element) {
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
        if !elem.common.ui_size.width.is_auto() {
            return elem.common.ui_size.width.get_pixel() - elem.common.padding.horizontal();
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
        if !elem.common.ui_size.height.is_auto() {
            return elem.common.ui_size.height.get_pixel() - elem.common.padding.vertical();
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