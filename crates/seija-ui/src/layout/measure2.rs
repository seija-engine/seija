use bevy_ecs::prelude::Entity;
use seija_core::math::Vec2;
use crate::layout::{types::LayoutElement, system::LayoutParams};
use super::{types::{TypeElement, LayoutAlignment, UISize, SizeValue}, comps::{StackLayout, FlexLayout, Orientation, FlexWrap, FlexDirection}};
use lazy_static::lazy_static;



lazy_static! { static ref VIEW_ID:LayoutElement = LayoutElement::create_view(); }

pub fn measure_layout_element(entity:Entity,parent_size:Vec2,params:&LayoutParams) {
   let element = params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
   let size = match &element.typ_elem {
       TypeElement::View => measure_view_element(entity,parent_size,element,params),
       TypeElement::Stack(stack) => measure_stack_element(entity, stack, parent_size, element, params),
       TypeElement::Flex(flex) => measure_flex_element(entity, flex, parent_size, element, params),
   };

  if let Ok(mut rect2d) = unsafe { params.rect2ds.get_unchecked(entity) } {
    rect2d.width  = size.x;
    rect2d.height = size.y;
  }
}


fn fill_desired_ui_size(entity:Entity,psize:UISize,elem:&LayoutElement,params:&LayoutParams) -> UISize {
   
   let desired_size = elem.common.ui_size.get_number_size(params.rect2ds.get(entity).ok());
   let width = if !elem.common.ui_size.width.is_auto() {
     SizeValue::Pixel(desired_size.x) 
   } else if elem.common.hor == LayoutAlignment::Stretch {
         psize.width
   } else { SizeValue::Auto };

   let height = if !elem.common.ui_size.height.is_auto() {
      SizeValue::Pixel(desired_size.y) 
   }  else if elem.common.hor == LayoutAlignment::Stretch {
      SizeValue::Pixel(desired_size.y) 
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
      return element.common.margin.apply2size(element.common.ui_size.get_number_size(params.rect2ds.get(entity).ok()));
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
   elem.common.margin.apply2size(uisize2size(cur_size, max_child_size))
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
      elem.common.margin.apply2size(ret_size)
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

fn calc_desired_flex_nowrap_size(entity:Entity,flex:&FlexLayout,cur_size:UISize,elem:&LayoutElement,params:&LayoutParams) -> Vec2 {
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
      number_size = element.common.ui_size.get_number_size(params.rect2ds.get(entity).ok())
   } else {
      number_size = calc_desired_size(entity, UISize::from_number(parent_size), params);
   }
   element.common.margin.apply2size(number_size)
}

fn measure_view_element(entity:Entity,parent_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let self_size:Vec2 = measure_self_size(entity, parent_size, element, params);
   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         measure_layout_element(*child_entity, element.common.padding.apply2size(self_size),params);
      }
   }
   self_size
}

fn measure_stack_element(entity:Entity,stack:&StackLayout,parent_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let self_size:Vec2 = measure_self_size(entity, parent_size, element, params);
   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         measure_layout_element(*child_entity, element.common.padding.apply2size(self_size),params);
      }
   }
   self_size
}
fn measure_flex_element(entity:Entity,flex:&FlexLayout,parent_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   match flex.warp {
      FlexWrap::NoWrap => measure_flex_nowrap_element(entity, flex,parent_size, element, params),
      FlexWrap::Wrap => measure_flex_wrap_element(entity, flex, parent_size, element, params),
   }
}


fn calc_axis_sizes(direction: FlexDirection, size: Vec2) -> (f32, f32) {
   match direction {
       FlexDirection::Row | FlexDirection::RowReverse => (size.x, size.y),
       FlexDirection::Column | FlexDirection::ColumnReverse => (size.y, size.x)
   }
}



fn measure_flex_nowrap_element(entity:Entity,flex:&FlexLayout,parent_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let self_size:Vec2 = measure_self_size(entity, parent_size, element, params);
   let content_size = element.common.padding.apply2size(self_size);

   let mut all_child_size:Vec<Vec2> = vec![];
   //计算flex的压缩和放大比例:总的放大比例和总的压缩比例
   let mut flex_grow_total = 0f32;
   let mut flex_shrink_total = 0f32;

   let mut main_axis_size_total   = 0f32;
   for child_entity in params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
      let child_size = calc_desired_size(*child_entity, UISize::from_number(content_size), params);
      all_child_size.push(child_size);

      let (flex_item_shrink, flex_item_grow) = params.flexitems.get(*child_entity).ok()
                                                     .and_then(|flex_item| Some((flex_item.shrink, flex_item.grow)))
                                                     .unwrap_or((1.0, 0.0));
      let (main_axis_size , _) = calc_axis_sizes(flex.direction, child_size);
      main_axis_size_total   += main_axis_size;
      flex_grow_total        += flex_item_grow;
      flex_shrink_total      += flex_item_shrink;
      
   }


  
   self_size
   
}

fn measure_flex_wrap_element(entity:Entity,flex:&FlexLayout,parent_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {

   Vec2::ZERO
}

pub fn measure_flex_item_element(entity:Entity,cur_size:Vec2,params:&LayoutParams) {
   let element = params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
   if let Ok(mut rect2d) = unsafe { params.rect2ds.get_unchecked(entity) } {
      rect2d.width  = cur_size.x;
      rect2d.height = cur_size.y;
   }
   match &element.typ_elem {
      TypeElement::View => measure_view_element(entity,cur_size,element,params),
      TypeElement::Stack(stack) => measure_stack_element(entity, stack, cur_size, element, params),
      TypeElement::Flex(flex) => measure_flex_element(entity, flex, cur_size, element, params),
   };
}