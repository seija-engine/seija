use std::ops::{BitOr, BitXor};

use bevy_ecs::prelude::Entity;
use seija_core::math::{Vec2};
use bitflags::bitflags;
use super::{types::{LayoutElement, TypeElement, LayoutAlignment}, system::LayoutParams, comps::{StackLayout, Orientation}};
/*
位置为元素锚点的(0.5,0.5)
parent_origin是父元素的坐标系下当前元素的左上

parent_size是减去元素padding之后的size
parent_origin也是应用过padding之后的位置
*/


bitflags! {
   
   pub struct ArrangeXY: u32 {
        const X = 0b00000001;
        const Y = 0b00000010;
        const ALL = ArrangeXY::X.bits | ArrangeXY::Y.bits;
    }
}

pub fn arrange_layout_element(entity:Entity,element:&LayoutElement,parent_origin:Vec2,parent_size:Vec2,axy:ArrangeXY,params:&LayoutParams) {
    let arrange_position = match &element.typ_elem {
        TypeElement::View => {  arrange_view_element(entity,element,parent_origin,parent_size,axy,params) }
        TypeElement::Stack(stack) => { arrange_stack_element(entity, stack, element, parent_origin, parent_size,axy, params) },
        TypeElement::Flex(flex) => {
            //TODO 
            Vec2::ZERO
        },
    };
    if let Ok(mut transform) = unsafe { params.trans.get_unchecked(entity) } {
        transform.local.position.x = arrange_position.x;
        transform.local.position.y = arrange_position.y;
    }
}

pub fn arrange_view_element(entity:Entity,element:&LayoutElement,parent_origin:Vec2,parent_size:Vec2,axy:ArrangeXY,params:&LayoutParams) -> Vec2 {
    let mut ret_pos = parent_origin;
    if let Ok(rect2d) = params.rect2ds.get(entity) {
        if (axy & ArrangeXY::X).bits > 0 {
            match element.common.hor {
                LayoutAlignment::Start  => { ret_pos.x += rect2d.width * 0.5f32 + element.common.margin.left; },
                LayoutAlignment::Center | LayoutAlignment::Stretch => { ret_pos.x += parent_size.x * 0.5f32; },
                LayoutAlignment::End => {  ret_pos.x += parent_size.x + -rect2d.width * 0.5f32 - element.common.margin.right; }
            }
        }
       
        if (axy & ArrangeXY::Y).bits > 0 {
            match element.common.ver {
                LayoutAlignment::Start  => { ret_pos.y += -rect2d.height * 0.5f32 - element.common.margin.top; },
                LayoutAlignment::Center | LayoutAlignment::Stretch => { ret_pos.y += -parent_size.y * 0.5f32; },
                LayoutAlignment::End => {  ret_pos.y += -parent_size.y + rect2d.height * 0.5f32 + element.common.margin.bottom; }
            }
        }
    }
    ret_pos
}

pub fn arrange_stack_element(entity:Entity,stack:&StackLayout,element:&LayoutElement,parent_origin:Vec2,parent_size:Vec2,axy:ArrangeXY,params:&LayoutParams) -> Vec2 {
    let this_pos = arrange_view_element(entity,element,parent_origin,parent_size,axy,params);
    if let Ok(rect2d) = params.rect2ds.get(entity) {
        
        let lt_pos = Vec2::new( -rect2d.width * 0.5f32 + element.common.padding.left,rect2d.height * 0.5f32 - element.common.padding.top);
        let inner_size = Vec2::new(rect2d.width - element.common.padding.horizontal(), rect2d.height - element.common.padding.vertical());
        if let Ok(childs) = params.childrens.get(entity) {
            let mut cur_pos:Vec2 = lt_pos;
            for child_entity in childs.iter() {
                if let Ok(child_size) = params.rect2ds.get(*child_entity) {
                    if let Ok(child_element) = params.elems.get(*child_entity) {
                        
                        match stack.orientation {
                            Orientation::Horizontal => {
                                arrange_layout_element(*child_entity, child_element, 
                                                        Vec2::new(cur_pos.x + child_size.width * 0.5f32,cur_pos.y),
                                                        inner_size,ArrangeXY::Y, params);
                                cur_pos.x += stack.spacing + child_size.width ;
                            },
                            Orientation::Vertical => {
                                arrange_layout_element(*child_entity, child_element,
                                                        Vec2::new(cur_pos.x,cur_pos.y - child_size.height * 0.5f32), 
                                                        inner_size,ArrangeXY::X, params);
                                cur_pos.y -= stack.spacing + child_size.height;
                            }
                        }
                        
                    }
                }
            }
        }
    }
    
    this_pos
}