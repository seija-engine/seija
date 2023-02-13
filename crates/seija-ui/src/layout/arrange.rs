use bevy_ecs::prelude::Entity;
use seija_core::math::{Vec3, Vec2};
use super::{types::{LayoutElement, TypeElement, LayoutAlignment}, system::LayoutParams, comps::{StackLayout, Orientation}};
/*
 默认把位置设置到子元素锚点的(0.5,0.5)
 parent_origin是父元素的左上
*/

pub fn arrange_layout_element(entity:Entity,element:&LayoutElement,parent_origin:Vec2,parent_size:Vec2,params:&LayoutParams) {
    let mut arrange_position = Vec2::ZERO;
    match &element.typ_elem {
        TypeElement::View => {  arrange_position = arrange_view_element(entity,element,parent_origin,parent_size,params); }
        TypeElement::Stack(stack) => {

        }
    }
    if let Ok(mut transform) = unsafe { params.trans.get_unchecked(entity) } {
        transform.local.position.x = arrange_position.x;
        transform.local.position.y = arrange_position.y;
    }
}

pub fn arrange_view_element(entity:Entity,element:&LayoutElement,parent_origin:Vec2,parent_size:Vec2,params:&LayoutParams) -> Vec2 {
    let mut ret_pos = Vec2::ZERO;
    if let Ok(rect2d) = params.rect2ds.get(entity) {
        match element.common.hor {
            LayoutAlignment::Start  => { ret_pos.x = rect2d.width * 0.5f32; },
            LayoutAlignment::Center | LayoutAlignment::Stretch => { ret_pos.x = parent_size.x * 0.5f32; },
            LayoutAlignment::End => {  ret_pos.x = parent_size.x + -rect2d.width * 0.5f32; }
        }

        match element.common.ver {
            LayoutAlignment::Start  => { ret_pos.y = -rect2d.height * 0.5f32; },
            LayoutAlignment::Center | LayoutAlignment::Stretch => { ret_pos.y = -parent_size.y * 0.5f32; },
            LayoutAlignment::End => {  ret_pos.y = -parent_size.y + rect2d.height * 0.5f32; }
        }
    }
    seija_core::log::error!("{:?} + {:?}",parent_origin,ret_pos);
    parent_origin + ret_pos
}