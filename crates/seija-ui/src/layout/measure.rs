use bevy_ecs::prelude::Entity;
use seija_core::math::{Vec2, Vec3};
use super::{types::{LayoutElement, TypeElement}, system::LayoutParams, comps::{StackLayout, Orientation}};

pub fn measure_layout_element(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let measure_size;
    match &element.typ_elem {
        TypeElement::View => { measure_size = element.common.get_content_size(request_size, params.rect2ds.get(entity).ok()); }
        TypeElement::ViewBox => { measure_size = measure_viewbox_layout(entity,request_size,element,params); }
        TypeElement::Stack(stack) => { measure_size = measure_stack_layout(entity,stack,request_size,element,params); }
    }
   

    if let Ok(mut rect2d) = unsafe { params.rect2ds.get_unchecked(entity) } {
        rect2d.width  = measure_size.x;
        rect2d.height = measure_size.y;
    }

    measure_size
}

//View Box 自适配模式下会被最大的子节点撑开
fn measure_viewbox_layout(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let (fixed_size,free_inner_size) = element.common.get_size_info(request_size, params.rect2ds.get(entity).ok());
    let mut ret_size = fixed_size;
    if let Ok(children) = params.childrens.get(entity) {
        let mut is_dirty = false;
        for child_entity in children.iter() {
            if let Ok(child_elem) = params.elems.get(*child_entity) {
                let child_size = measure_layout_element(*child_entity, free_inner_size, child_elem, params);
                if fixed_size.x < 0f32 && child_size.x > ret_size.x {
                    ret_size.x = child_size.x;
                    is_dirty = true;
                }
                if fixed_size.y < 0f32 && child_size.y > ret_size.y {
                    ret_size.y = child_size.y;
                    is_dirty = true;
                }
            }
        }
        if is_dirty {
            for child_entity in children.iter() {
                if let Ok(child_elem) = params.elems.get(*child_entity) {
                    measure_layout_element(*child_entity, ret_size, child_elem, params);
                }
            }
        }
    }

    ret_size
}

////Stack System
fn measure_stack_layout(entity:Entity,stack:&StackLayout,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let (fixed_size,free_inner_size) = element.common.get_size_info(request_size, params.rect2ds.get(entity).ok());
    let mut ret_size = fixed_size;
    match stack.orientation {
        Orientation::Horizontal => {
            ret_size.x = element.common.padding.horizontal();
        },
        Orientation::Vertical => { 
            ret_size.y = element.common.padding.vertical();
        }
    }

    let childs = params.childrens.get(entity);
    if let Ok(childs) = childs {
        for child_entity in childs.iter() {
            if let Ok(child_element) = params.elems.get(entity) {
                let child_size = measure_layout_element(*child_entity, free_inner_size, child_element, params);
                match stack.orientation {
                    Orientation::Horizontal => {
                        ret_size.x += child_size.x;
                    },
                    Orientation::Vertical => { 
                        ret_size.y += child_size.y;
                    }
                }
            }
        }
    }

    let free_x = request_size.x - element.common.margin.horizontal();
    if ret_size.x > free_x {
        ret_size.x = free_x;
    }
    let free_y = request_size.y - element.common.margin.vertical();
    if ret_size.y > free_y {
        ret_size.y = free_y;
    }
   
    ret_size
}