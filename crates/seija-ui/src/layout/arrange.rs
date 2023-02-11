use bevy_ecs::prelude::Entity;
use seija_core::math::{Vec3, Vec2};
use super::{types::{LayoutElement, TypeElement}, system::LayoutParams, comps::{StackLayout, Orientation}};

//parent_origin固定是父元素的左上
pub fn arrange_layout_element(entity:Entity,element:&LayoutElement,parent_origin:Vec2,params:&LayoutParams) {
    let mut arrange_position = Vec2::ZERO;
    match &element.typ_elem {
        TypeElement::View => {  arrange_position = arrange_view_element(entity,element,parent_origin,params); }
        TypeElement::Stack(stack) => {  

        }
    }
    if let Ok(mut transform) = unsafe { params.trans.get_unchecked(entity) } {
        transform.local.position.x = arrange_position.x;
        transform.local.position.y = arrange_position.y;
    }
}

pub fn arrange_view_element(entity:Entity,element:&LayoutElement,parent_origin:Vec2,params:&LayoutParams) -> Vec2 {
    if !element.common.anchor_correct { return Vec2::ZERO + parent_origin; }
    if let Ok(rect2d) = params.rect2ds.get(entity) {
       let nx = parent_origin.x + rect2d.anchor.x * rect2d.width;
       let ny = parent_origin.y - rect2d.anchor.y * rect2d.height;
       return Vec2::new(nx, ny);
    }
    parent_origin
}