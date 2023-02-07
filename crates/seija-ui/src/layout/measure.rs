use bevy_ecs::prelude::Entity;
use seija_core::math::Vec2;
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

//View Box
fn measure_viewbox_layout(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let mut content_size = element.common.get_content_size(request_size, params.rect2ds.get(entity).ok());
    let inner_size = Vec2::new(content_size.x - element.common.padding.horizontal(),content_size.y - element.common.padding.vertical());
    if let Ok(children) = params.childrens.get(entity) {
        for child_entity in children.iter() {
           if let Ok(child_elem) = params.elems.get(*child_entity) {
              let child_size = measure_layout_element(*child_entity, inner_size, child_elem, params);
              if child_size.x > content_size.x {
                content_size.x = child_size.x;
              }
              if child_size.y > content_size.y {
                content_size.y = child_size.y;
              }
           }
        }

        for child_entity in children.iter() {
            if let Ok(child_elem) = params.elems.get(*child_entity) {
                measure_layout_element(*child_entity, content_size, child_elem, params);
            }
        }
    }
    Vec2::ZERO
}

////Stack System
fn measure_stack_layout(entity:Entity,stack:&StackLayout,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let mut ret_size = request_size;
    //let content_size:Vec2 = element.common.get_content_size(request_size, params.rect2ds.get(entity).ok());
    //let inner_size:Vec2 = Vec2::new(content_size.x - element.common.padding.horizontal(),
    //                               content_size.y - element.common.padding.vertical());
    
    match stack.orientation {
           Orientation::Horizontal => ret_size.x = element.common.padding.left,
           Orientation::Vertical => ret_size.y = element.common.padding.top
    }

    if let Ok(children) = params.childrens.get(entity) {
        for child_entity in children.iter() {

        }
    }
    Vec2::ZERO
}