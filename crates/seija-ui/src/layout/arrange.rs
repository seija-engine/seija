use bevy_ecs::prelude::Entity;
use bitflags::bitflags;
use seija_core::math::Vec2;
use seija_transform::hierarchy::Children;

use super::{
    comps::{FlexDirection, FlexJustify, FlexLayout, FlexWrap, Orientation, StackLayout, FlexAlignItems},
    system::LayoutParams,
    types::{LayoutAlignment, LayoutElement, TypeElement},
    RECT2D_ID, VIEW_ID,
};
/*
位置为元素锚点的(0.5,0.5)
parent_origin是父元素的坐标系下当前元素的左上

parent_size是减去元素padding之后的size
parent_origin也是应用过padding之后的位置
*/

bitflags! {
   pub struct ArrangeXY: u32 {
        const NONE = 0b00000000;
        const X    = 0b00000001;
        const Y    = 0b00000010;
        const ALL = ArrangeXY::X.bits | ArrangeXY::Y.bits;
    }
}

pub fn arrange_layout_element(entity: Entity,element: &LayoutElement,parent_origin: Vec2,parent_size: Vec2,axy: ArrangeXY,params: &LayoutParams) {
    let arrange_position = match &element.typ_elem {
        TypeElement::View => {
            arrange_view_element(entity, element, parent_origin, parent_size, axy, params)
        }
        TypeElement::Stack(stack) => arrange_stack_element(entity,stack,element,parent_origin,parent_size,axy,params),
        TypeElement::Flex(flex) => arrange_flex_element(
            entity,
            flex,
            element,
            parent_origin,
            parent_size,
            axy,
            params,
        ),
    };
    if let Ok(mut transform) = unsafe { params.trans.get_unchecked(entity) } {
        transform.local.position.x = arrange_position.x;
        transform.local.position.y = arrange_position.y;
    }
}

pub fn arrange_view_element(
    entity: Entity,
    element: &LayoutElement,
    parent_origin: Vec2,
    parent_size: Vec2,
    axy: ArrangeXY,
    params: &LayoutParams,
) -> Vec2 {
    let mut ret_pos = parent_origin;
    if let Ok(rect2d) = params.rect2ds.get(entity) {
        if (axy & ArrangeXY::X).bits > 0 {
            match element.common.hor {
                LayoutAlignment::Start => {
                    ret_pos.x += rect2d.width * 0.5f32 + element.common.margin.left;
                }
                LayoutAlignment::Center | LayoutAlignment::Stretch => {
                    ret_pos.x += parent_size.x * 0.5f32;
                }
                LayoutAlignment::End => {
                    ret_pos.x +=
                        parent_size.x + -rect2d.width * 0.5f32 - element.common.margin.right;
                }
            }
        }

        if (axy & ArrangeXY::Y).bits > 0 {
            match element.common.ver {
                LayoutAlignment::Start => {
                    ret_pos.y += -rect2d.height * 0.5f32 - element.common.margin.top;
                }
                LayoutAlignment::Center | LayoutAlignment::Stretch => {
                    ret_pos.y += -parent_size.y * 0.5f32;
                }
                LayoutAlignment::End => {
                    ret_pos.y +=
                        -parent_size.y + rect2d.height * 0.5f32 + element.common.margin.bottom;
                }
            }
        }
    }
    ret_pos
}

pub fn arrange_stack_element(
    entity: Entity,
    stack: &StackLayout,
    element: &LayoutElement,
    parent_origin: Vec2,
    parent_size: Vec2,
    axy: ArrangeXY,
    params: &LayoutParams,
) -> Vec2 {
    let this_pos = arrange_view_element(entity, element, parent_origin, parent_size, axy, params);
    if let Ok(rect2d) = params.rect2ds.get(entity) {
        let lt_pos = Vec2::new(
            -rect2d.width * 0.5f32 + element.common.padding.left,
            rect2d.height * 0.5f32 - element.common.padding.top,
        );
        let inner_size = Vec2::new(
            rect2d.width - element.common.padding.horizontal(),
            rect2d.height - element.common.padding.vertical(),
        );
        if let Ok(childs) = params.childrens.get(entity) {
            let mut cur_pos: Vec2 = lt_pos;
            for child_entity in childs.iter() {
                if let Ok(child_size) = params.rect2ds.get(*child_entity) {
                    if let Ok(child_element) = params.elems.get(*child_entity) {
                        match stack.orientation {
                            Orientation::Horizontal => {
                                arrange_layout_element(
                                    *child_entity,
                                    child_element,
                                    Vec2::new(cur_pos.x + child_size.width * 0.5f32, cur_pos.y),
                                    inner_size,
                                    ArrangeXY::Y,
                                    params,
                                );
                                cur_pos.x += stack.spacing + child_size.width;
                            }
                            Orientation::Vertical => {
                                arrange_layout_element(
                                    *child_entity,
                                    child_element,
                                    Vec2::new(cur_pos.x, cur_pos.y - child_size.height * 0.5f32),
                                    inner_size,
                                    ArrangeXY::X,
                                    params,
                                );
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

fn arrange_flex_element(
    entity: Entity,
    flex: &FlexLayout,
    element: &LayoutElement,
    parent_origin: Vec2,
    parent_size: Vec2,
    _: ArrangeXY,
    params: &LayoutParams,
) -> Vec2 {
    match flex.warp {
        FlexWrap::Wrap => {
            Vec2::ZERO
            //arrange_flex_element_wrap(entity, flex, element, parent_origin, parent_size, params)
        }
        FlexWrap::NoWrap => {
            arrange_flex_element_nowrap(entity, flex, element, parent_origin, parent_size, params)
        }
    }
}

pub struct FlexIter<'a> {
    pub children: Option<&'a Children>,
    pub is_reverse: bool,
    pub index: usize,
}

impl<'a> FlexIter<'a> {
    pub fn new(children: Option<&'a Children>, flex_dir: FlexDirection) -> Self {
        match flex_dir {
            FlexDirection::RowReverse | FlexDirection::ColumnReverse => Self {
                children,
                is_reverse: true,
                index: children.map(|c| c.children().len() - 1).unwrap_or_default(),
            },
            _ => Self {
                children,
                is_reverse: false,
                index: 0,
            },
        }
    }

    fn child_count(&self) -> usize {
        self.children.map(|v| v.len()).unwrap_or(0)
    }
}

impl<'a> Iterator for FlexIter<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(children) = self.children {
            let ret = children.children().get(self.index).copied();
            if self.is_reverse {
                if self.index > 0 {
                    self.index -= 1;
                }
            } else {
                if self.index < children.children().len() - 1 {
                    self.index += 1;
                }
            }
            ret
        } else {
            None
        }
    }

   
}

fn arrange_flex_element_nowrap(entity: Entity,flex: &FlexLayout,element: &LayoutElement,parent_origin: Vec2,parent_size: Vec2,params: &LayoutParams) -> Vec2 {
    let this_pos = arrange_view_element(entity,element,parent_origin,parent_size,ArrangeXY::ALL,params);
    let this_size = params.rect2ds.get(entity).unwrap_or(&RECT2D_ID);
    let inner_size = element.common.padding.apply2size(Vec2::new(this_size.width, this_size.height));
    let this_axis_size = flex.get_axis_size(inner_size);
    let lt_pos = Vec2::new(
        -this_size.width * 0.5f32 + element.common.padding.left,
        this_size.height * 0.5f32 - element.common.padding.top,
    );
    let mut main_axis_size = 0f32;
    for child_entity in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
        let child_size = params.rect2ds.get(*child_entity).unwrap_or(&RECT2D_ID);
        let axis_size = flex.get_axis_size(Vec2::new(child_size.width, child_size.height));
        main_axis_size += axis_size.x;
    }

    //没有空余空间，直接根据size进行排列
    if main_axis_size >= this_axis_size.x || flex.justify == FlexJustify::Start {
        arrange_by_start_pos(lt_pos,flex,entity,inner_size,this_axis_size,params);
        return this_pos;
    }

    //有空余空间，根据justify进行排列
    match flex.justify {
        FlexJustify::Center => {
            let start_pos:Vec2 = match flex.direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    Vec2::new(lt_pos.x + (this_axis_size.x - main_axis_size) * 0.5f32,lt_pos.y)
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    Vec2::new(lt_pos.x,lt_pos.y - (this_axis_size.x - main_axis_size) * 0.5f32)
                }
            };
            arrange_by_start_pos(start_pos,flex,entity,inner_size,this_axis_size,params);
        }
        FlexJustify::End => {
            let start_pos = match flex.direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    Vec2::new(lt_pos.x + (this_axis_size.x - main_axis_size),lt_pos.y)
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    Vec2::new(lt_pos.x,lt_pos.y - (this_axis_size.x - main_axis_size))
                }
            };
            arrange_by_start_pos(start_pos,flex,entity,inner_size,this_axis_size,params);
        }

        //分散对齐,两边贴靠
        FlexJustify::SpaceBetween => { arrange_flex_spacebetween(entity,lt_pos,flex,this_axis_size,inner_size,params) }
        //分散对齐，两边是中间的一半
        FlexJustify::SpaceAround => { arrange_flex_spacearound(entity,lt_pos,flex,this_axis_size,inner_size,params) }
        _ => {}
    }

    this_pos
}


fn align_flex_cross(align_item: FlexAlignItems,axis_start:f32,axis_size:f32,self_size:f32,axis:f32) -> f32 {
    match align_item {
        FlexAlignItems::Start => { axis_start + (self_size * 0.5f32 * axis) },
        FlexAlignItems::Center | FlexAlignItems::Stretch => { axis_start + (axis_size * axis * 0.5f32)  },
        FlexAlignItems::End => { axis_start + (axis_size * axis) - (self_size * 0.5f32 * axis) },
    }
}

fn arrange_by_start_pos(start_pos:Vec2,flex: &FlexLayout,entity: Entity,inner_size:Vec2,this_axis_size:Vec2,params: &LayoutParams) {
    let flex_iter = FlexIter::new(params.childrens.get(entity).ok(), flex.direction);
    let mut cur_pos: Vec2 = start_pos;
    for child_entity in flex_iter {
        let child_rect = params.rect2ds.get(child_entity).unwrap_or(&RECT2D_ID);
        let child_elem = params.elems.get(child_entity).unwrap_or(&VIEW_ID);
        match flex.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                cur_pos.y = align_flex_cross(flex.align_items, start_pos.y, this_axis_size.x, child_rect.height, 1f32);
                arrange_layout_element(child_entity,child_elem,
                    Vec2::new(cur_pos.x + child_rect.width * 0.5f32, cur_pos.y),
                    inner_size,
                    ArrangeXY::NONE,
                    params,
                );
                cur_pos.x += child_rect.width;
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                cur_pos.y = align_flex_cross(flex.align_items, start_pos.x, this_axis_size.x, child_rect.width, -1f32);
                arrange_layout_element(child_entity,child_elem,
                    Vec2::new(cur_pos.x, cur_pos.y - child_rect.height * 0.5f32),
                    inner_size,
                    ArrangeXY::NONE,
                    params,
                );
                cur_pos.y -= child_rect.height;
            }
        }
    }
}

fn arrange_flex_spacebetween(entity: Entity,lt_pos:Vec2,flex:&FlexLayout,this_axis_size:Vec2,this_size:Vec2,params: &LayoutParams) {
    let flex_iter = FlexIter::new(params.childrens.get(entity).ok(), flex.direction);
    let child_count = flex_iter.child_count();
    let mut main_size_sum:f32 = 0f32;
    let spacing = this_axis_size.x / (child_count - 1) as f32;
    for (index,child_entity) in flex_iter.enumerate() {
        let child_elem = params.elems.get(child_entity).unwrap_or(&VIEW_ID);
        let child_rect = params.rect2ds.get(child_entity).unwrap_or(&RECT2D_ID);
        let mut pos;
        if index == 0 {
            if flex.is_hor() { 
                pos = Vec2::new(lt_pos.x + child_rect.width * 0.5f32,lt_pos.y); 
                main_size_sum += child_rect.width;
            } else {  
                pos = Vec2::new(lt_pos.x,lt_pos.y - child_rect.height * 0.5f32);
                main_size_sum += child_rect.height;
            }
        } else if child_count == index + 1 {
            if flex.is_hor() { 
                pos = Vec2::new(lt_pos.x + this_axis_size.x - child_rect.width * 0.5f32,lt_pos.y); 
                
            } else {  
                pos = Vec2::new(lt_pos.x,lt_pos.y - this_axis_size.x + child_rect.height * 0.5f32);
            }
        } else {
            if flex.is_hor() { 
                pos = Vec2::new(lt_pos.x + main_size_sum + spacing * index as f32,lt_pos.y); 
                main_size_sum += child_rect.width;
            } else {  
                pos = Vec2::new(lt_pos.x,lt_pos.y - main_size_sum - spacing * index as f32);
                main_size_sum += child_rect.height;
            }
        }
        if flex.is_hor() {
            pos.y = align_flex_cross(flex.align_items, lt_pos.y, this_axis_size.x, child_rect.height, 1f32);
        } else {
            pos.x = align_flex_cross(flex.align_items, lt_pos.x, this_axis_size.x, child_rect.width, -1f32);
        }
        arrange_layout_element(child_entity,child_elem,pos,this_size,ArrangeXY::NONE,params);
    }
}

fn arrange_flex_spacearound(entity: Entity,lt_pos:Vec2,flex:&FlexLayout,this_axis_size:Vec2,this_size:Vec2,params: &LayoutParams) {
    let flex_iter = FlexIter::new(params.childrens.get(entity).ok(), flex.direction);
    let child_count = flex_iter.child_count();
    let mut main_size_sum:f32 = 0f32;
    let spacing = this_axis_size.x / child_count as f32;
    for (index,child_entity) in flex_iter.enumerate() {
        let child_elem = params.elems.get(child_entity).unwrap_or(&VIEW_ID);
        let child_rect = params.rect2ds.get(child_entity).unwrap_or(&RECT2D_ID);
        let mut pos;
        if index == 0 {
            if flex.is_hor() { 
                pos = Vec2::new(lt_pos.x + child_rect.width * 0.5f32 + spacing * 0.5f32,lt_pos.y); 
                main_size_sum += child_rect.width + spacing * 0.5f32;
            } else {  
                pos = Vec2::new(lt_pos.x,lt_pos.y - child_rect.height * 0.5f32 - spacing * 0.5f32);
                main_size_sum += child_rect.height + spacing * 0.5f32;
            }
        } else if child_count == index + 1 {
            if flex.is_hor() { 
                pos = Vec2::new(lt_pos.x + this_axis_size.x - child_rect.width * 0.5f32 - spacing * 0.5f32,lt_pos.y); 
            } else {  
                pos = Vec2::new(lt_pos.x,lt_pos.y - this_axis_size.x + child_rect.height * 0.5f32 + spacing * 0.5f32);
            }
        } else {
            if flex.is_hor() { 
                pos = Vec2::new(lt_pos.x + main_size_sum + spacing * index as f32,lt_pos.y); 
                main_size_sum += child_rect.width;
            } else {  
                pos = Vec2::new(lt_pos.x,lt_pos.y - main_size_sum - spacing * index as f32);
                main_size_sum += child_rect.height;
            }
        }
        if flex.is_hor() {
            pos.y = align_flex_cross(flex.align_items, lt_pos.y, this_axis_size.x, child_rect.height, 1f32);
        } else {
            pos.x = align_flex_cross(flex.align_items, lt_pos.x, this_axis_size.x, child_rect.width, -1f32);
        }
        arrange_layout_element(child_entity,child_elem,pos,this_size,ArrangeXY::NONE,params);
    }
}