use bevy_ecs::prelude::Entity;
use seija_core::{math::{Vec2, Vec3}, log::log};
use super::{types::{LayoutElement, TypeElement}, system::LayoutParams, comps::{StackLayout, Orientation}};

pub fn measure_layout_element(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let measure_size;
    match &element.typ_elem {
        TypeElement::View => { measure_size = measure_view_layout(entity,request_size,element,params); }
        TypeElement::Stack(stack) => { measure_size = measure_stack_layout(entity,stack,request_size,element,params); }
    }
   
    
    if let Ok(mut rect2d) = unsafe { params.rect2ds.get_unchecked(entity) } {
        rect2d.width  = measure_size.x;
        rect2d.height = measure_size.y;
    }
    measure_size
}

//没有强制能撑开，有强制撑不开, request_size就是强制Size
fn measure_view_layout(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let fixed_size = element.common.get_fixed_size(request_size, params.rect2ds.get(entity).ok());
    
    let padding = &element.common.padding;
    let mut inner_size = Vec2::new(fixed_size.x - padding.horizontal(), fixed_size.y - padding.vertical());
    let mut ret_size = fixed_size;
    if let Ok(children) = params.childrens.get(entity) {
        let mut is_dirty = false;
        for child_entity in children.iter() {
            if let Ok(child_elem) = params.elems.get(*child_entity) {
                let child_size = measure_layout_element(*child_entity, inner_size, child_elem, params);
                if fixed_size.x < 0f32 && child_size.x > inner_size.x {
                    inner_size.x = child_size.x;
                    ret_size.x = child_size.x + padding.horizontal();
                    is_dirty = true;
                }
                if fixed_size.y < 0f32 && child_size.y > ret_size.y {
                    inner_size.y = child_size.y;
                    ret_size.y = child_size.y + padding.vertical();
                    is_dirty = true;
                }
            }
        }

        //被子节点撑开了布局，其他子节点的Stretch要重新计算
        if is_dirty {
            for child_entity in children.iter() {
                if let Ok(child_elem) = params.elems.get(*child_entity) {
                    measure_layout_element(*child_entity, inner_size, child_elem, params);
                }
            }
        }
    }
    
    ret_size
}

////Stack System
fn measure_stack_layout(entity:Entity,stack:&StackLayout,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let fixed_size = element.common.get_fixed_size(request_size, params.rect2ds.get(entity).ok());
    let padding = &element.common.padding;
    let mut inner_size = Vec2::new(fixed_size.x - padding.horizontal(), fixed_size.y - padding.vertical());
    let mut ret_size = fixed_size;
    match stack.orientation {
        Orientation::Horizontal => {
            if fixed_size.x < 0f32 {
                ret_size.x = element.common.padding.horizontal();
            }
            inner_size.x = 0f32;
        },
        Orientation::Vertical => { 
            if fixed_size.y < 0f32 {
                ret_size.y = element.common.padding.vertical();
            }
            inner_size.y  = 0f32;
        }
    }
    let childs = params.childrens.get(entity);
    if let Ok(childs) = childs {
        let mut is_dirty = false;
        for child_entity in childs.iter() {
            if let Ok(child_element) = params.elems.get(*child_entity) {
                let child_size = measure_layout_element(*child_entity, inner_size, child_element, params);
                
                match stack.orientation {
                    Orientation::Horizontal => {
                        if fixed_size.x < 0f32 {
                            ret_size.x += child_size.x + stack.spacing;
                        }
                        
                        if fixed_size.y < 0f32 && ret_size.y < child_size.y {
                            ret_size.y = child_size.y;
                            inner_size.y = child_size.y - padding.vertical();
                            is_dirty = true;
                        }
                    },
                    Orientation::Vertical => { 
                        if fixed_size.y < 0f32 {
                            ret_size.y += child_size.y + stack.spacing;
                        }
                        if fixed_size.x < 0f32 && ret_size.x < child_size.x {
                            ret_size.x = child_size.x;
                            inner_size.x = child_size.x - padding.vertical();
                            is_dirty = true;
                        }
                    }
                }
            }
        }

        //被子节点撑开了垂直方向的布局，其他子节点的Stretch要重新计算
        if is_dirty {
            for child_entity in childs.iter() {
                if let Ok(child_element) = params.elems.get(*child_entity) {
                    measure_layout_element(*child_entity, inner_size, child_element, params);
                }
            }
        }
    }
    
    ret_size
}