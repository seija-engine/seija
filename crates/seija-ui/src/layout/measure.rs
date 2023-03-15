use bevy_ecs::prelude::Entity;
use seija_core::math::Vec2;
use crate::layout::{types::LayoutElement, system::LayoutParams};
use super::{types::{TypeElement, LayoutAlignment, UISize, SizeValue}, comps::{StackLayout, FlexLayout, Orientation, FlexWrap, FlexDirection, FlexAlignItems, FlexAlignContent}, VIEW_ID};


pub fn measure_layout_element(entity:Entity,parent_size:Vec2,params:&LayoutParams) {
   let element = params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
   let size = match &element.typ_elem {
       TypeElement::View => measure_view_element(entity,parent_size,None,element,params),
       TypeElement::Stack(stack) => measure_stack_element(entity, stack, parent_size,None, element, params),
       TypeElement::Flex(flex) => measure_flex_element(entity, flex, parent_size,None, element, params),
   };

  if let Ok(mut rect2d) = unsafe { params.rect2ds.get_unchecked(entity) } {
    rect2d.width  = size.x - element.common.margin.horizontal();
    rect2d.height = size.y - element.common.margin.vertical();
  }
}


fn fill_desired_ui_size(entity:Entity,psize:UISize,elem:&LayoutElement,params:&LayoutParams) -> UISize {
   
   let desired_size = elem.common.ui_size.get_number_size(params.rect2ds.get(entity).ok());
   let width = if !elem.common.ui_size.width.is_auto() {
     SizeValue::Pixel(desired_size.x + elem.common.margin.horizontal()) 
   } else if elem.common.hor == LayoutAlignment::Stretch {
         psize.width
   } else { SizeValue::Auto };

   let height = if !elem.common.ui_size.height.is_auto() {
      SizeValue::Pixel(desired_size.y + elem.common.margin.vertical()) 
   }  else if elem.common.ver == LayoutAlignment::Stretch {
      psize.height
   } else { SizeValue::Auto };
   UISize { width, height }
}


fn uisize2size(cur_size:UISize,size:Vec2) -> Vec2 {
   Vec2::new(if cur_size.width.is_auto()  { size.x } else { cur_size.width.get_pixel() }, 
             if cur_size.height.is_auto() { size.y } else { cur_size.height.get_pixel() })
}

fn calc_desired_size(entity:Entity,psize:UISize,params:&LayoutParams) -> Vec2 {
   let element = params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
   let cur_size = fill_desired_ui_size(entity,psize,&element,params);
   if !cur_size.has_auto() {
      return cur_size.get_number_size(params.rect2ds.get(entity).ok());
   };
   match &element.typ_elem {
      TypeElement::View => calc_desired_view_size(entity,cur_size,&element,params),
      TypeElement::Stack(stack) =>  calc_desired_stack_size(entity,&stack,cur_size,&element,params),
      TypeElement::Flex(flex) => calc_desired_flex_size(entity,&flex,cur_size,&element,params),
   }
}

fn calc_desired_max_child_size(entity:Entity,cur_size:UISize,params:&LayoutParams) -> Vec2 {
   let mut max_child_size = Vec2::new(0f32, 0f32);
   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         let child_size = calc_desired_size(*child_entity,cur_size,params);
         if child_size.x > max_child_size.x {
            max_child_size.x = child_size.x;
        }
        if child_size.y > max_child_size.y {
           max_child_size.y = child_size.y;
        }
      }
   }
   max_child_size
}

fn calc_desired_view_size(entity:Entity,cur_size:UISize,elem:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let max_child_size = calc_desired_max_child_size(entity,cur_size,params);
   elem.common.margin.add2size(uisize2size(cur_size, max_child_size))
}

fn calc_desired_stack_size(entity:Entity,stack:&StackLayout,cur_size:UISize,elem:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let is_main_axis_auto = match stack.orientation {
      Orientation::Horizontal => cur_size.width.is_auto(),
      Orientation::Vertical => cur_size.height.is_auto()
   };
   if is_main_axis_auto {
      let mut ret_size:Vec2 = Vec2::ZERO;
      if let Ok(childs_comp) = params.childrens.get(entity) {
         for child_entity in childs_comp.iter() {
            let child_size = calc_desired_size(*child_entity, cur_size, params);
            match stack.orientation {
               Orientation::Horizontal => {
                  ret_size.x += child_size.x + stack.spacing;
                  if child_size.y > ret_size.y {
                     ret_size.y = child_size.y;
                  }
               },
               Orientation::Vertical => {
                  ret_size.y += child_size.y + stack.spacing;
                  if child_size.x > ret_size.x {
                     ret_size.x = child_size.x;
                  }
               }
            }
         }
      }
      elem.common.margin.add2size(ret_size)
   } else {
      calc_desired_view_size(entity,cur_size,elem,params)
   }
}

fn calc_desired_flex_size(entity:Entity,flex:&FlexLayout,cur_size:UISize,elem:&LayoutElement,params:&LayoutParams) -> Vec2 {
   match flex.warp {
      FlexWrap::NoWrap => calc_desired_flex_nowrap_size(entity,flex,cur_size,elem,params),
      FlexWrap::Wrap => calc_desired_flex_wrap_size(entity,flex,cur_size,elem,params), 
   }
}

fn calc_desired_flex_nowrap_size(entity:Entity,flex:&FlexLayout,cur_size:UISize,_:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let mut ret_size:Vec2 = uisize2size(cur_size, Vec2::ZERO);

   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         let child_size = calc_desired_size(*child_entity, cur_size, params);
         match flex.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
               if cur_size.width.is_auto() {
                  ret_size.x += child_size.x;
               }
               if cur_size.height.is_auto() {
                  if child_size.y > ret_size.y {
                     ret_size.y = child_size.y;
                  }
               }
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
               if cur_size.height.is_auto() {
                  ret_size.y += child_size.y;
               }
               if cur_size.width.is_auto() {
                  if child_size.x > ret_size.x {
                     ret_size.x = child_size.x;
                  }
               }
            }
         }
      }
   }
   ret_size
}

fn calc_desired_flex_wrap_size(entity:Entity,flex:&FlexLayout,cur_size:UISize,elem:&LayoutElement,params:&LayoutParams) -> Vec2 {
   //warp的情况下，主轴不能是auto
   let mut ret_size:Vec2 = uisize2size(cur_size, Vec2::ZERO);
   let mut line_max_size = 0f32;
   let mut added_main_size = 0f32;
   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         let child_size = calc_desired_size(*child_entity, cur_size, params);
         match flex.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
               if child_size.y > line_max_size {
                  line_max_size = child_size.y;
               }
               if (added_main_size + child_size.x) > ret_size.x {
                  added_main_size = 0f32;
                  ret_size.y += line_max_size;
               } else {
                  added_main_size += child_size.x;
               }
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
               if child_size.x > line_max_size {
                  line_max_size = child_size.x;
               }
               if (added_main_size + child_size.y) > ret_size.y {
                  added_main_size = 0f32;
                  ret_size.x += line_max_size;
               } else {
                  added_main_size += child_size.x;
               }
            }
         }
      }
   }
   ret_size
}

fn measure_self_size(entity:Entity,parent_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let number_size:Vec2;
   if !element.common.ui_size.has_auto() {
      number_size = element.common.margin.add2size(element.common.ui_size.get_number_size(params.rect2ds.get(entity).ok()))
   } else {
      number_size = calc_desired_size(entity, UISize::from_number(parent_size), params);
   }
   number_size
}

fn measure_view_element(entity:Entity,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let self_size:Vec2 = force_size.unwrap_or_else(|| measure_self_size(entity, parent_size, element, params));
   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         measure_layout_element(*child_entity, element.common.padding.sub2size(self_size),params);
      }
   }
   self_size
}

fn measure_stack_element(entity:Entity,stack:&StackLayout,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let self_size:Vec2 = force_size.unwrap_or_else(|| measure_self_size(entity, parent_size, element, params));
   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         measure_layout_element(*child_entity, element.common.padding.sub2size(self_size),params);
      }
   }
   self_size
}


fn measure_flex_element(entity:Entity,flex:&FlexLayout,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   match flex.warp {
      FlexWrap::NoWrap => measure_flex_nowrap_element(entity, flex,parent_size,force_size, element, params),
      FlexWrap::Wrap => measure_flex_wrap_element(entity, flex, parent_size,force_size, element, params),
   }
}

fn calc_axis_sizes(direction: FlexDirection, size: Vec2) -> (f32, f32) {
   match direction {
       FlexDirection::Row | FlexDirection::RowReverse => (size.x, size.y),
       FlexDirection::Column | FlexDirection::ColumnReverse => (size.y, size.x)
   }
}

fn axis2vec2(direction: FlexDirection,main:f32,cross:f32) -> Vec2 {
   match direction {
      FlexDirection::Row | FlexDirection::RowReverse => Vec2::new(main, cross),
      FlexDirection::Column | FlexDirection::ColumnReverse => Vec2::new(cross, main)
  }
}

fn measure_flex_nowrap_element(entity:Entity,flex:&FlexLayout,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let self_size:Vec2 = force_size.unwrap_or_else(|| measure_self_size(entity, parent_size, element, params));
   let content_size = element.common.padding.sub2size(self_size);
   let main_axis_number = match flex.direction {
      FlexDirection::Column | FlexDirection::ColumnReverse => { self_size.y },
      FlexDirection::Row | FlexDirection::RowReverse => { self_size.x }
   };

   let mut child_size_lst:Vec<Vec2> = vec![];
   let mut all_shrink_total = 0f32;
   let mut all_grow_total = 0f32;
   let mut all_main_child_size = 0f32;
   let mut all_remain_shrink = main_axis_number;
   let mut all_remain_grow = main_axis_number;
   let mut has_grow = false;
   for child_entity in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
      let child_size = calc_desired_size(*child_entity, UISize::from_number(content_size), params);
      child_size_lst.push(child_size);

      let (flex_item_shrink, flex_item_grow) = params.flexitems.get(*child_entity)
                                                     .ok()
                                                     .and_then(|flex_item| Some((flex_item.shrink, flex_item.grow)))
                                                     .unwrap_or((1.0, 0.0));
      if flex_item_grow > 0f32 { has_grow = true; }
      let (child_main_size , _) = calc_axis_sizes(flex.direction, child_size);
      if flex_item_shrink > 0f32 {
         all_shrink_total += child_main_size / flex_item_shrink;
      } else {
         all_remain_shrink -= child_main_size;
      }
      if flex_item_grow > 0f32 {
         all_grow_total += child_main_size * flex_item_grow;
      } else {
         all_remain_grow -= child_main_size;
      }
      all_main_child_size += child_main_size;
   }

   if all_main_child_size > main_axis_number {
      shrink_no_warp(entity, child_size_lst, flex, all_shrink_total, all_remain_shrink, params)
   } else if has_grow {
      grow_no_warp(entity, child_size_lst, flex, all_grow_total, all_remain_grow, params);
   } else {
      for (index,child_entity) in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
         let child_size = child_size_lst[index];
         let (child_main_size , child_cross_size) = calc_axis_sizes(flex.direction, child_size);
         measure_flex_item_element(*child_entity,axis2vec2(flex.direction, child_main_size, child_cross_size),params);
      }
   }
   self_size
}

fn shrink_no_warp(entity:Entity,child_size_lst:Vec<Vec2>,flex:&FlexLayout,all_shrink_total:f32,all_remain_shrink:f32,params:&LayoutParams) {
   //挤压重排
   for (index,child_entity) in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
      let flex_item_shrink = params.flexitems.get(*child_entity).map(|v| v.shrink).unwrap_or(1f32);
      let child_size = child_size_lst[index];
      let (child_main_size , child_cross_size) = calc_axis_sizes(flex.direction, child_size);
      //可挤压元素
      if flex_item_shrink > 0f32 {
         let shrink_rate = all_shrink_total /  (child_main_size / flex_item_shrink);
         let new_main_size = all_remain_shrink * shrink_rate;
         measure_flex_item_element(*child_entity,axis2vec2(flex.direction, new_main_size, child_cross_size),params);
      } else {
         measure_flex_item_element(*child_entity,axis2vec2(flex.direction, child_main_size, child_cross_size),params);
      }
   }
}

fn grow_no_warp(entity:Entity,child_size_lst:Vec<Vec2>,flex:&FlexLayout,all_grow_total:f32,all_remain_grow:f32,params:&LayoutParams) {
   //放大重排
   for (index,child_entity) in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
      let flex_item_grow = params.flexitems.get(*child_entity).map(|v| v.grow).unwrap_or(0f32);
      let child_size = child_size_lst[index];
      let (child_main_size , child_cross_size) = calc_axis_sizes(flex.direction, child_size);
      let grow_rate = all_grow_total /  (child_main_size / flex_item_grow);
      let new_main_size = all_remain_grow * grow_rate;
      if flex_item_grow > 0f32 {
        measure_flex_item_element(*child_entity,axis2vec2(flex.direction, new_main_size, child_cross_size),params);
      } else {
        measure_flex_item_element(*child_entity,axis2vec2(flex.direction, child_main_size, child_cross_size),params);
      } 
    }
}

fn measure_flex_wrap_element(entity:Entity,flex:&FlexLayout,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let self_size:Vec2 = force_size.unwrap_or_else(|| measure_self_size(entity, parent_size, element, params));
   let content_size = element.common.padding.sub2size(self_size);
   let (main_axis_number, cross_axis_number) = calc_axis_sizes(flex.direction, content_size);
   if flex.align_content != FlexAlignContent::Stretch && flex.align_items != FlexAlignItems::Stretch {
      for child_entity in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
         let child_size = calc_desired_size(*child_entity, UISize::from_number(content_size), params);
         measure_flex_item_element(*child_entity,child_size, params);
      }
      return self_size;
   }
   let mut line_count = 1;
   let mut main_axis_total_size = 0f32;
   let mut all_child_sizes = vec![];
   //计算Flex的行数
   for child_entity in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
      let child_size = calc_desired_size(*child_entity, UISize::from_number(content_size), params);
      all_child_sizes.push(child_size);
      let (main_axis_size, _) = calc_axis_sizes(flex.direction, child_size);
      if main_axis_total_size + main_axis_size > main_axis_number {
         line_count += 1;
         main_axis_total_size = 0f32;
      } else {
         main_axis_total_size += main_axis_size;
      }
   }
   let cross_stretch_size = cross_axis_number / line_count as f32;
   for (index,child_entity) in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
      let item = params.elems.get(*child_entity).ok().unwrap_or(&VIEW_ID);
      match flex.direction {
         FlexDirection::Row | FlexDirection::RowReverse => {
            if item.common.ui_size.height.is_auto() {
               all_child_sizes[index].y = cross_stretch_size;
            } 
         }
         FlexDirection::Column | FlexDirection::ColumnReverse => {
            if item.common.ui_size.width.is_auto() {
               all_child_sizes[index].y = cross_stretch_size;
            }
         }
      }
      measure_flex_item_element(*child_entity, all_child_sizes[index], params);
   }
   self_size
}

pub fn measure_flex_item_element(entity:Entity,cur_size:Vec2,params:&LayoutParams) {
   let element = params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
   if let Ok(mut rect2d) = unsafe { params.rect2ds.get_unchecked(entity) } {
      rect2d.width  = cur_size.x;
      rect2d.height = cur_size.y;
   }
   match &element.typ_elem {
      TypeElement::View => measure_view_element(entity,cur_size,Some(cur_size),element,params),
      TypeElement::Stack(stack) => measure_stack_element(entity, stack,cur_size,Some(cur_size), element, params),
      TypeElement::Flex(flex) => measure_flex_element(entity, flex,cur_size, Some(cur_size), element, params),
   };
}

