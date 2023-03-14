use std::ops::Range;

use bevy_ecs::prelude::Entity;
use bitflags::bitflags;
use seija_core::math::Vec2;
use seija_transform::hierarchy::Children;

use super::{
    comps::{FlexDirection, FlexJustify, FlexLayout, FlexWrap, Orientation, StackLayout, FlexAlignItems, FlexAlignContent},
    system::LayoutParams,
    types::{LayoutAlignment, LayoutElement, TypeElement},
    RECT2D_ID, VIEW_ID,
};
/*
位置为元素锚点的(0.5,0.5)
parent_origin是父元素的坐标系下的左上角位置

parent_size是减去元素padding之后的size
parent_origin也是应用过padding之后的位置

rect2d里的size是元素的显示size，不包含margin
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
                    ret_pos.x +=  element.common.margin.left + rect2d.width * 0.5f32;
                }
                LayoutAlignment::Center | LayoutAlignment::Stretch => {
                    let offset = rect2d.width * 0.5f32 + element.common.margin.left;
                    ret_pos.x += offset;
                }
                LayoutAlignment::End => {
                    ret_pos.x += parent_size.x + -rect2d.width * 0.5f32 - element.common.margin.right;
                }
            }
        }

        if (axy & ArrangeXY::Y).bits > 0 {
            match element.common.ver {
                LayoutAlignment::Start => {
                    ret_pos.y = ret_pos.y - element.common.margin.top - rect2d.height * 0.5f32;
                }
                LayoutAlignment::Center | LayoutAlignment::Stretch => {
                    ret_pos.y += -parent_size.y * 0.5f32;
                }
                LayoutAlignment::End => {
                    ret_pos.y += -parent_size.y + rect2d.height * 0.5f32 + element.common.margin.bottom;
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
        FlexWrap::Wrap => {  arrange_flex_element_wrap(entity, flex, element, parent_origin, parent_size, params) }
        FlexWrap::NoWrap => { arrange_flex_element_nowrap(entity, flex, element, parent_origin, parent_size, params) }
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
                if self.index < children.children().len() {
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
    let inner_size = element.common.padding.sub2size(Vec2::new(this_size.width, this_size.height));
    let this_axis_size = flex.get_axis_size(inner_size);
    let child_count = params.childrens.get(entity).map(|v| v.children().len()).unwrap_or(0);
    if child_count == 0 { return this_pos; }
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
    if main_axis_size >= this_axis_size.x  {
        arrange_by_start_pos(lt_pos,flex,entity,inner_size,0f32,params);
        return this_pos;
    }

    //有空余空间，根据justify进行排列
    match flex.justify {
        FlexJustify::Start => { arrange_by_start_pos(lt_pos,flex,entity,inner_size,0f32,params); },
        FlexJustify::Center => {
            let start_pos:Vec2 = match flex.direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    Vec2::new(lt_pos.x + (this_axis_size.x - main_axis_size) * 0.5f32,lt_pos.y)
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    Vec2::new(lt_pos.x,lt_pos.y - (this_axis_size.x - main_axis_size) * 0.5f32)
                }
            };
            arrange_by_start_pos(start_pos,flex,entity,inner_size,0f32,params);
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
            arrange_by_start_pos(start_pos,flex,entity,inner_size,0f32,params);
        }

        //分散对齐,两边贴靠
        FlexJustify::SpaceBetween => {
            let spacing;
            if flex.is_hor() {
                spacing = (inner_size.x - main_axis_size) / (child_count - 1) as f32;
            } else {
                spacing = (inner_size.y - main_axis_size) / (child_count - 1) as f32;
            }
            arrange_by_start_pos(lt_pos,flex,entity,inner_size,spacing,params);
        }
        //分散对齐，两边是中间的一半
        FlexJustify::SpaceAround => { 
            let spacing;
            let start_pos;
            if flex.is_hor() {
                spacing = (inner_size.x - main_axis_size) / child_count as f32;
                start_pos = Vec2::new(lt_pos.x + spacing * 0.5f32, lt_pos.y);
            } else {
                spacing = (inner_size.y - main_axis_size) / child_count as f32;
                start_pos = Vec2::new(lt_pos.x, lt_pos.y - spacing * 0.5f32);
            }
            arrange_by_start_pos(start_pos,flex,entity,inner_size,spacing,params);
        }
    }

    this_pos
}


//计算换行情况下的排列
fn arrange_flex_element_wrap(entity:Entity,flex:&FlexLayout,elem:&LayoutElement,parent_origin:Vec2,parent_size:Vec2,params:&LayoutParams) -> Vec2 {
    let this_pos = arrange_view_element(entity,elem,parent_origin,parent_size,ArrangeXY::ALL,params);
    let this_size = params.rect2ds.get(entity).unwrap_or(&RECT2D_ID);
    let inner_size = elem.common.padding.sub2size(Vec2::new(this_size.width, this_size.height));
    let this_axis_size = flex.get_axis_size(inner_size);
    let lt_pos = Vec2::new(-this_size.width * 0.5f32 + elem.common.padding.left,this_size.height * 0.5f32 - elem.common.padding.top);
    let (axis_pos,cross_pos) = match flex.direction {
        FlexDirection::Row | FlexDirection::RowReverse => (lt_pos.x,lt_pos.y),
        FlexDirection::Column | FlexDirection::ColumnReverse => (lt_pos.y,lt_pos.x) ,
    };
    let flex_iter = FlexIter::new(params.childrens.get(entity).ok(),flex.direction);
    let child_count = flex_iter.child_count();
    let mut child_pos_lst:Vec<Vec2> = vec![];
    let mut chid_size_lst:Vec<Vec2> = vec![];
    let mut line_total = 0f32;
    let mut last_line_start = 0;
    let mut line_idx_range:Vec<Range<usize>> = vec![];
    for (index,child_entity) in flex_iter.enumerate() {
        let child_size = params.rect2ds.get(child_entity).unwrap_or(&RECT2D_ID);
        let axis_size = flex.get_axis_size(Vec2::new(child_size.width, child_size.height));
        chid_size_lst.push(axis_size);
        if line_total + axis_size.x > this_axis_size.x {
            calc_align_jusitfy(flex.justify,this_axis_size.x, axis_pos,&mut chid_size_lst[last_line_start..index],flex.direction,&mut child_pos_lst);
            line_total = axis_size.x;
            line_idx_range.push(last_line_start..index);
            last_line_start = index;
        } else {
            line_total += axis_size.x;
        }
    }
    if last_line_start < child_count {
        seija_core::log::error!("{} .. {}",last_line_start,child_count);
        calc_align_jusitfy(flex.justify,this_axis_size.x, axis_pos,&mut chid_size_lst[last_line_start..child_count],flex.direction,&mut child_pos_lst);
        line_idx_range.push(last_line_start..child_count);
    }

    calc_align_content(flex, this_axis_size.y, line_idx_range, cross_pos,&mut chid_size_lst,&mut child_pos_lst);
    let flex_iter = FlexIter::new(params.childrens.get(entity).ok(),flex.direction);
    for (index,child_entity) in flex_iter.enumerate() {
        let cur_pos = child_pos_lst[index];
        arrange_layout_element(child_entity, params.elems.get(child_entity).unwrap_or(&VIEW_ID), cur_pos, parent_size, ArrangeXY::NONE, params)
    }
    this_pos
}

fn calc_align_jusitfy(justify:FlexJustify,axis_size:f32,start:f32,size_lst:&mut [Vec2],dir:FlexDirection,pos_lst:&mut Vec<Vec2>) {
    match justify {
        FlexJustify::Start => {
            let mut cur_axis = start;
            match dir {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    cur_axis += size_lst[0].x * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(cur_axis,0f32));
                        cur_axis += size.x;
                    }
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    cur_axis -= size_lst[0].y * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(0f32,cur_axis));
                        cur_axis -= size.y;
                    }
                }
            }
        },
        FlexJustify::Center => {
            let total_size = size_lst.iter().fold(0f32,|acc,size| acc + size.x);
            let mut cur_axis = start + (axis_size - total_size) * 0.5f32;
            match dir {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    cur_axis += size_lst[0].x * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(cur_axis,0f32));
                        cur_axis += size.x;
                    }
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    cur_axis -= size_lst[0].y * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(0f32,cur_axis));
                        cur_axis -= size.y;
                    }
                }
            }
        }
        FlexJustify::End => {
            let total_size = size_lst.iter().fold(0f32,|acc,size| acc + size.x);
            let mut cur_axis = start + (axis_size - total_size);
            match dir {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    cur_axis += size_lst[0].x * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(cur_axis,0f32));
                        cur_axis += size.x;
                    }
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    cur_axis -= size_lst[0].y * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(0f32,cur_axis));
                        cur_axis -= size.y;
                    }
                }
            }
        }
        FlexJustify::SpaceBetween => {
            let total_size = size_lst.iter().fold(0f32,|acc,size| acc + size.x);
            let mut cur_axis = start;
            let space = (axis_size - total_size) / (size_lst.len() as f32 - 1f32);
            match dir {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    cur_axis += size_lst[0].x * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(cur_axis,0f32));
                        cur_axis += size.x + space;
                    }
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    cur_axis -= size_lst[0].y * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(0f32,cur_axis));
                        cur_axis -= size.y + space;
                    }
                }
            }
        }
        FlexJustify::SpaceAround => {
            let total_size = size_lst.iter().fold(0f32,|acc,size| acc + size.x);
            let mut cur_axis = start;
            let space = (axis_size - total_size) / (size_lst.len() as f32);
            match dir {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    cur_axis += size_lst[0].x * 0.5f32 + space * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(cur_axis,0f32));
                        cur_axis += size.x + space;
                    }
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    cur_axis -= size_lst[0].y * 0.5f32 - space * 0.5f32;
                    for size in size_lst {
                        pos_lst.push(Vec2::new(0f32,cur_axis));
                        cur_axis -= size.y + space;
                    }
                }
            }
        }
    }
}

fn calc_align_content(flex:&FlexLayout,cross_size:f32,line_ranges:Vec<Range<usize>>,start:f32,size_lst:&mut Vec<Vec2>,pos_lst:&mut Vec<Vec2>) {
    let calc_child_size = || {
        let mut all_child_size = 0f32;
        for idxs in line_ranges.iter() {
            let is_hor = flex.is_hor();
            all_child_size += size_lst[idxs.clone()].iter().fold(0f32,|acc,size| if is_hor { acc.max(size.y) } else { acc.max(size.x) });
        }
        all_child_size
    };
    let fst_max_size = size_lst[line_ranges[0].clone()].iter().fold(Vec2::ZERO, |acc,size| Vec2::new(acc.x.max(acc.x), acc.y.max(size.y) ) ); 
    let mut start_pos = start;
    let mut space = 0f32;
    match flex.align_content {
        FlexAlignContent::Start | FlexAlignContent::Stretch => {
            start_pos +=  if flex.is_hor() {-fst_max_size.y * 0.5f32 } else { fst_max_size.x * 0.5f32 };
        },
        FlexAlignContent::Center => {
            let all_child_size = calc_child_size();
            if flex.is_hor() {
                start_pos -= fst_max_size.y * 0.5f32;
                start_pos = start_pos - (cross_size - all_child_size) * 0.5f32;
            } else { 
                start_pos += fst_max_size.x * 0.5f32;
                start_pos += (cross_size - all_child_size) * 0.5f32;
            };
            
        },
        FlexAlignContent::End => {
            let all_child_size = calc_child_size();
            if flex.is_hor() {
                start_pos -= (cross_size - all_child_size) + fst_max_size.y * 0.5f32;
            } else {
                start_pos += (cross_size - all_child_size) + fst_max_size.x * 0.5f32;
            }
        },
        FlexAlignContent::SpaceBetween => {
            let all_child_size = calc_child_size();
            space = (cross_size - all_child_size) / (line_ranges.len() - 1) as f32;
            if flex.is_hor() {
                start_pos -= fst_max_size.y * 0.5f32;
            } else { 
                start_pos += fst_max_size.x * 0.5f32;
            };
        },
        FlexAlignContent::SpaceAround => {
            let all_child_size = calc_child_size();
            space = (cross_size - all_child_size) / line_ranges.len() as f32;
            if flex.is_hor() {
                start_pos -= fst_max_size.y * 0.5f32 + space * 0.5f32;
            } else { 
                start_pos += fst_max_size.x * 0.5f32 + space * 0.5f32;
            };
        }
    }

    let mut cur_cross = start_pos;
    for idxs in line_ranges {
        let max_size = size_lst[idxs.clone()].iter().fold(Vec2::ZERO, |acc,size| Vec2::new(acc.x.max(acc.x), acc.y.max(size.y) ) ); 
        for idx in idxs {
            if flex.is_hor() {
                pos_lst[idx].y = cur_cross;
            } else {
                pos_lst[idx].x = cur_cross;
            }
        }
        if flex.is_hor() {
            cur_cross -= max_size.y + space;
        } else {
            cur_cross += max_size.y + space;
        }
    }
}


fn align_flex_cross(align_item: FlexAlignItems,axis_start:f32,axis_size:f32,self_size:f32,axis:f32) -> f32 {
    match align_item {
        FlexAlignItems::Start => { axis_start + (self_size * 0.5f32 * axis) },
        FlexAlignItems::Center | FlexAlignItems::Stretch => { axis_start + (axis_size * axis * 0.5f32)  },
        FlexAlignItems::End => { axis_start + (axis_size * axis) - (self_size * 0.5f32 * axis) },
    }
}

fn arrange_by_start_pos(start_pos:Vec2,flex: &FlexLayout,entity: Entity,inner_size:Vec2,spacing:f32,params: &LayoutParams) {
    let flex_iter = FlexIter::new(params.childrens.get(entity).ok(), flex.direction);
    let mut cur_pos: Vec2 = start_pos;
    for child_entity in flex_iter {
        let child_rect = params.rect2ds.get(child_entity).unwrap_or(&RECT2D_ID);
        let child_elem = params.elems.get(child_entity).unwrap_or(&VIEW_ID);
        match flex.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                cur_pos.y = align_flex_cross(flex.align_items, start_pos.y, inner_size.y, child_rect.height, -1f32);
                arrange_layout_element(child_entity,child_elem,
                    Vec2::new(cur_pos.x + child_rect.width * 0.5f32, cur_pos.y),
                    inner_size,
                    ArrangeXY::NONE,
                    params,
                );
                cur_pos.x += child_rect.width + spacing;
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                cur_pos.y = align_flex_cross(flex.align_items, start_pos.x, inner_size.x, child_rect.width, 1f32);
                arrange_layout_element(child_entity,child_elem,
                    Vec2::new(cur_pos.x, cur_pos.y - child_rect.height * 0.5f32),
                    inner_size,
                    ArrangeXY::NONE,
                    params,
                );
                cur_pos.y -= child_rect.height + spacing;
            }
        }
    }
}
