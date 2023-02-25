use bevy_ecs::prelude::Entity;
use seija_core::math::Vec2;
use crate::layout::{types::LayoutElement, system::LayoutParams};
use super::{types::{TypeElement, LayoutAlignment, UISize, SizeValue}, comps::{StackLayout, FlexLayout, Orientation, FlexWrap}};
use lazy_static::lazy_static;

lazy_static! { static ref VIEW_ID:LayoutElement = LayoutElement::create_view(); }

pub fn measure_layout_element(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&mut LayoutParams) -> Vec2 {
   match element.typ_elem {
       TypeElement::View => measure_view_element(entity,request_size,element,params),
       _ => Vec2::ZERO
  };
   Vec2::ZERO
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
      FlexWrap::Wrap => Vec2::ZERO
   }
}

fn calc_desired_flex_nowrap_size(entity:Entity,flex:&FlexLayout,cur_size:UISize,elem:&LayoutElement,params:&LayoutParams) -> Vec2 {
   let mut ret_size:Vec2 = Vec2::ZERO;
   if let Ok(childs_comp) = params.childrens.get(entity) {
      for child_entity in childs_comp.iter() {
         let child_size = calc_desired_size(*child_entity, cur_size, params);
         if flex.is_hor() {
            
         } else {

         }
      }
   }
   Vec2::ZERO
}

fn measure_view_element(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&mut LayoutParams) -> Vec2 {
   if element.common.ui_size.has_auto() {
      if element.common.hor == LayoutAlignment::Stretch {

      }
   }
   Vec2::ZERO
}