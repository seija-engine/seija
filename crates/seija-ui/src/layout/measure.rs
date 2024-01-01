use bevy_ecs::{prelude::Entity, system::Query};
use seija_core::{math::Vec2, info::EStateInfo};
use crate::layout::{types::LayoutElement, system::LayoutParams};
use super::{types::{TypeElement, LayoutAlignment, UISize, SizeValue, FreeLayout}, 
            comps::{StackLayout, FlexLayout, Orientation, FlexWrap, FlexDirection, FlexAlignItems, FlexAlignContent}, VIEW_ID};

pub(crate) struct MeasureScope<'p,'i,'aa,'w,'s> {
   pub params:&'p LayoutParams<'w,'s>,
   pub infos:&'i Query<'w,'s,&'aa EStateInfo>
}

impl<'p,'i,'w,'s,'aa> MeasureScope<'p,'i,'w,'s,'aa> {

   //parent_size是给子元素的实际可用空间，去除了margin和padding
   pub fn measure_layout_element(&self,entity:Entity,parent_size:Vec2) {
      let is_active = self.infos.get(entity).map(|v| v.is_active_global()).unwrap_or(true);
      if !is_active { return; } 
      let element = self.params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
      let size = match &element.typ_elem {
         TypeElement::View => self.measure_view_element(entity,parent_size,None,element),
         TypeElement::Stack(stack) => self.measure_stack_element(entity, stack, parent_size,None, element),
         TypeElement::Flex(flex) => self.measure_flex_element(entity, flex, parent_size,None, element),
         TypeElement::Free(free) => self.measure_free_element(entity,parent_size,None,element,free),
      };

      if let Ok(mut rect2d) = unsafe { self.params.rect2ds.get_unchecked(entity) } {
         rect2d.width  = size.x - element.common.margin.horizontal();
         rect2d.height = size.y - element.common.margin.vertical();
      }
   }

   fn measure_self_size(&self,entity:Entity,parent_size:Vec2,element:&LayoutElement) -> Vec2 {
      let number_size:Vec2;
      if !element.common.ui_size.has_auto() {
         number_size = element.common.margin.add2size(element.common.ui_size.get_number_size(self.params.rect2ds.get(entity).ok()))
      } else {
         number_size = self.calc_desired_size(entity, UISize::from_number(parent_size));
      }
      number_size
   }

   fn calc_desired_size(&self,entity:Entity,psize:UISize) -> Vec2 {
      let element = self.params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
      let cur_size = fill_desired_ui_size(entity,psize,&element,self.params);
      if !cur_size.has_auto() {
         return cur_size.get_number_size(self.params.rect2ds.get(entity).ok());
      };
      match &element.typ_elem {
         TypeElement::View => self.calc_desired_view_size(entity,cur_size,&element),
         TypeElement::Stack(stack) => self.calc_desired_stack_size(entity,&stack,cur_size,&element),
         TypeElement::Flex(flex) => self.calc_desired_flex_size(entity,&flex,cur_size,&element),
         TypeElement::Free(_) => cur_size.get_number_size(self.params.rect2ds.get(entity).ok())
      }
   }

   fn calc_desired_flex_size(&self,entity:Entity,flex:&FlexLayout,cur_size:UISize,elem:&LayoutElement) -> Vec2 {
      match flex.warp {
         FlexWrap::NoWrap => self.calc_desired_flex_nowrap_size(entity,flex,cur_size,elem),
         FlexWrap::Wrap => self.calc_desired_flex_wrap_size(entity,flex,cur_size,elem), 
      }
   }

   fn calc_desired_flex_wrap_size(&self,entity:Entity,flex:&FlexLayout,cur_size:UISize,_elem:&LayoutElement) -> Vec2 {
      //warp的情况下，主轴不能是auto
      let mut ret_size:Vec2 = uisize2size(cur_size, Vec2::ZERO);
      let mut line_max_size = 0f32;
      let mut added_main_size = 0f32;
      if let Ok(childs_comp) = self.params.childrens.get(entity) {
         for child_entity in childs_comp.iter() {
            let is_active = self.is_active_global(*child_entity);
            if !is_active { continue; }
            let child_size = self.calc_desired_size(*child_entity, cur_size);
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

   fn calc_desired_flex_nowrap_size(&self,entity:Entity,flex:&FlexLayout,cur_size:UISize,_:&LayoutElement) -> Vec2 {
      let mut ret_size:Vec2 = uisize2size(cur_size, Vec2::ZERO);
   
      if let Ok(childs_comp) = self.params.childrens.get(entity) {
         for child_entity in childs_comp.iter() {
            let is_active = self.is_active_global(*child_entity);
            if !is_active { continue; }
            let child_size = self.calc_desired_size(*child_entity, cur_size);
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

   fn calc_desired_view_size(&self,entity:Entity,cur_size:UISize,elem:&LayoutElement) -> Vec2 {
      let max_child_size = self.calc_desired_max_child_size(entity,cur_size);
      elem.common.margin.add2size(uisize2size(cur_size, max_child_size))
   }

   fn calc_desired_max_child_size(&self,entity:Entity,cur_size:UISize) -> Vec2 {
      let mut max_child_size = Vec2::new(0f32, 0f32);
      if let Ok(childs_comp) = self.params.childrens.get(entity) {
         for child_entity in childs_comp.iter() {
            let is_active = self.is_active_global(*child_entity);
            if is_active {
               let child_size = self.calc_desired_size(*child_entity,cur_size);
               if child_size.x > max_child_size.x {
                  max_child_size.x = child_size.x;
               }
               if child_size.y > max_child_size.y {
                  max_child_size.y = child_size.y;
               }
            }
         }
      }
      max_child_size
   }

   fn calc_desired_stack_size(&self,entity:Entity,stack:&StackLayout,cur_size:UISize,elem:&LayoutElement) -> Vec2 {
      let is_main_axis_auto = match stack.orientation {
         Orientation::Horizontal => cur_size.width.is_auto(),
         Orientation::Vertical => cur_size.height.is_auto()
      };
      if is_main_axis_auto {
         let mut ret_size:Vec2 = cur_size.get_number_size(self.params.rect2ds.get(entity).ok());
         if let Ok(childs_comp) = self.params.childrens.get(entity) {
            for child_entity in childs_comp.iter() {
               let is_active = self.is_active_global(*child_entity);
               if !is_active { continue; }
               let child_size = self.calc_desired_size(*child_entity, cur_size);
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
         self.calc_desired_view_size(entity,cur_size,elem)
      }
   }

   fn measure_view_element(&self,entity:Entity,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement) -> Vec2 {
      let self_size:Vec2 = force_size.unwrap_or_else(|| self.measure_self_size(entity, parent_size, element));
      if let Ok(childs_comp) = self.params.childrens.get(entity) {
         for child_entity in childs_comp.iter() {
            self.measure_layout_element(*child_entity, element.common.sub_margin_and_padding(self_size));
         }
      }
      self_size
   }

   fn measure_stack_element(&self,entity:Entity,_:&StackLayout,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement) -> Vec2 {
      let self_size:Vec2 = force_size.unwrap_or_else(|| self.measure_self_size(entity, parent_size, element));
      if let Ok(childs_comp) = self.params.childrens.get(entity) {
         for child_entity in childs_comp.iter() {
            self.measure_layout_element(*child_entity,element.common.sub_margin_and_padding(self_size));
         }
      }
      self_size
   }

   fn measure_flex_element(&self,entity:Entity,flex:&FlexLayout,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement) -> Vec2 {
      match flex.warp {
         FlexWrap::NoWrap => self.measure_flex_nowrap_element(entity, flex,parent_size,force_size, element),
         FlexWrap::Wrap => self.measure_flex_wrap_element(entity, flex, parent_size,force_size, element),
      }
   }

   
   fn measure_free_element(&self,entity:Entity,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement,free:&FreeLayout) -> Vec2 {
      let self_size:Vec2 = force_size.unwrap_or_else(|| measure_self_free_size(entity,parent_size, element,free,self.params));
      if let Ok(childs_comp) = self.params.childrens.get(entity) {
         for child_entity in childs_comp.iter() {
            self.measure_layout_element(*child_entity, element.common.sub_margin_and_padding(self_size));
         }
      }
      self_size
   }

   fn measure_flex_nowrap_element(&self,entity:Entity,flex:&FlexLayout,parent_size:Vec2,force_size:Option<Vec2>
                               ,element:&LayoutElement) -> Vec2 {
      let self_size:Vec2 = force_size.unwrap_or_else(|| self.measure_self_size(entity, parent_size, element));
      let content_size = element.common.sub_margin_and_padding(self_size);
      //log::error!("measure_flex_nowrap_element:{:?}",content_size);
      let main_axis_number = match flex.direction {
         FlexDirection::Column | FlexDirection::ColumnReverse => { content_size.y },
         FlexDirection::Row | FlexDirection::RowReverse => { content_size.x }
      };
      //log::error!("main_axis_number:{:?}",main_axis_number);
      let mut child_size_lst:Vec<Vec2> = vec![];
      let mut all_shrink_total = 0f32;
      let mut all_grow_total = 0f32;
      let mut all_main_child_size = 0f32;
      let mut all_remain_shrink = main_axis_number;
      let mut all_remain_grow = main_axis_number;
      let mut has_grow = false;
      for child_entity in self.params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
         let child_size = self.calc_desired_size(*child_entity, UISize::from_number(content_size));
         child_size_lst.push(child_size);
         //log::error!("child_size:{:?}",child_size);
         let (flex_item_shrink, flex_item_grow) = self.params.flexitems.get(*child_entity)
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
            //log::error!("all_grow_total+=:{:?} {:?}",child_main_size,flex_item_grow);
         } else {
            all_remain_grow -= child_main_size;
         }
         all_main_child_size += child_main_size;
      }

      if all_main_child_size > main_axis_number {
         self.shrink_no_warp(entity, child_size_lst, flex, all_shrink_total, all_remain_shrink)
      } else if has_grow {
         //log::error!("grow_no_warp:total={:?} {:?} remain={:?}",all_grow_total,child_size_lst,all_remain_grow);
         self.grow_no_warp(entity, child_size_lst, flex, all_grow_total, all_remain_grow);
      } else {
         for (index,child_entity) in self.params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
            let child_size = child_size_lst[index];
            let (child_main_size , child_cross_size) = calc_axis_sizes(flex.direction, child_size);
            self.measure_flex_item_element(*child_entity,axis2vec2(flex.direction, child_main_size, child_cross_size));
         }
      }
      self_size
   }

   fn measure_flex_wrap_element(&self,entity:Entity,flex:&FlexLayout,parent_size:Vec2,force_size:Option<Vec2>,element:&LayoutElement) -> Vec2 {
      let self_size:Vec2 = force_size.unwrap_or_else(|| self.measure_self_size(entity, parent_size, element));
      let content_size = element.common.sub_margin_and_padding(self_size);
      let (main_axis_number, cross_axis_number) = calc_axis_sizes(flex.direction, content_size);
      if flex.align_content != FlexAlignContent::Stretch && flex.align_items != FlexAlignItems::Stretch {
         for child_entity in self.params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
            let child_size = self.calc_desired_size(*child_entity, UISize::from_number(content_size));
            self.measure_flex_item_element(*child_entity,child_size);
         }
         return self_size;
      }
      let mut line_count = 1;
      let mut main_axis_total_size = 0f32;
      let mut all_child_sizes = vec![];
      //计算Flex的行数
      for child_entity in self.params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()) {
         let child_size = self.calc_desired_size(*child_entity, UISize::from_number(content_size));
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
      for (index,child_entity) in self.params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
         let item = self.params.elems.get(*child_entity).ok().unwrap_or(&VIEW_ID);
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
         self.measure_flex_item_element(*child_entity, all_child_sizes[index]);
      }
      self_size
   }


   pub fn measure_flex_item_element(&self,entity:Entity,cur_size:Vec2) {
      let element = self.params.elems.get(entity).ok().unwrap_or(&VIEW_ID);
      if let Ok(mut rect2d) = unsafe { self.params.rect2ds.get_unchecked(entity) } {
         rect2d.width  = cur_size.x;
         rect2d.height = cur_size.y;
      }
      match &element.typ_elem {
         TypeElement::View => self.measure_view_element(entity,cur_size,Some(cur_size),element),
         TypeElement::Stack(stack) => self.measure_stack_element(entity, stack,cur_size,Some(cur_size), element),
         TypeElement::Flex(flex) => self.measure_flex_element(entity, flex,cur_size, Some(cur_size), element),
         TypeElement::Free(free) => self.measure_free_element(entity, cur_size, Some(cur_size),element,free),
      };
   }

   fn shrink_no_warp(&self,entity:Entity,child_size_lst:Vec<Vec2>,flex:&FlexLayout,all_shrink_total:f32,all_remain_shrink:f32) {
      //挤压重排
      for (index,child_entity) in self.params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
         let flex_item_shrink = self.params.flexitems.get(*child_entity).map(|v| v.shrink).unwrap_or(1f32);
         let child_size = child_size_lst[index];
         let (child_main_size , child_cross_size) = calc_axis_sizes(flex.direction, child_size);
         //可挤压元素
         if flex_item_shrink > 0f32 {
            let shrink_rate = all_shrink_total /  (child_main_size / flex_item_shrink);
            let new_main_size = all_remain_shrink * shrink_rate;
            self.measure_flex_item_element(*child_entity,axis2vec2(flex.direction, new_main_size, child_cross_size));
         } else {
            self.measure_flex_item_element(*child_entity,axis2vec2(flex.direction, child_main_size, child_cross_size));
         }
      }
   }

   fn grow_no_warp(&self,entity:Entity,child_size_lst:Vec<Vec2>,flex:&FlexLayout,all_grow_total:f32,all_remain_grow:f32) {
      //放大重排
      for (index,child_entity) in self.params.childrens.get(entity).map(|v| v.iter()).unwrap_or_else(|_| [].iter()).enumerate() {
         let flex_item_grow = self.params.flexitems.get(*child_entity).map(|v| v.grow).unwrap_or(0f32);
         let child_size = child_size_lst[index];
         let (child_main_size , child_cross_size) = calc_axis_sizes(flex.direction, child_size);
         let grow_rate =   (child_main_size * flex_item_grow) / all_grow_total;
         let new_main_size = all_remain_grow * grow_rate;
         if flex_item_grow > 0f32 {
            self.measure_flex_item_element(*child_entity,axis2vec2(flex.direction, new_main_size, child_cross_size));
         } else {

            self.measure_flex_item_element(*child_entity,axis2vec2(flex.direction, child_main_size, child_cross_size));
         } 
       }
   }

   pub fn is_active_global(&self,entity:Entity) -> bool {
      self.infos.get(entity).map(|v| v._is_active_global).unwrap_or(true)
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


fn measure_self_free_size(entity:Entity,parent_size:Vec2,element:&LayoutElement,_:&FreeLayout,params:&LayoutParams) -> Vec2 {
   let number_size:Vec2;
   if !element.common.ui_size.has_auto() {
      number_size = element.common.margin.add2size(element.common.ui_size.get_number_size(params.rect2ds.get(entity).ok()))
   } else {
      let mut desired_size = element.common.ui_size.get_number_size(params.rect2ds.get(entity).ok());
      if !element.common.ui_size.width.is_auto() {
         desired_size.x += element.common.margin.horizontal();
      } else if element.common.hor == LayoutAlignment::Stretch {
         desired_size.x = parent_size.x;
      }

      if !element.common.ui_size.height.is_auto() {
         desired_size.y += element.common.margin.vertical();
      } else if element.common.ver == LayoutAlignment::Stretch {
         desired_size.y = parent_size.y;
      }
      number_size = desired_size;
   }
   
   number_size
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

