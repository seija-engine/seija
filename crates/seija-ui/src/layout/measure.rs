use bevy_ecs::prelude::Entity;
use seija_core::{math::{Vec2}};
use super::{types::{LayoutElement, TypeElement}, system::LayoutParams, comps::{StackLayout, Orientation, FlexLayout, FlexDirection, FlexWrap}};

pub fn measure_layout_element(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let measure_size = match &element.typ_elem {
        TypeElement::View => {measure_view_layout(entity,request_size,element,params) }
        TypeElement::Stack(stack) => { measure_stack_layout(entity,stack,request_size,element,params) },
        TypeElement::Flex(flex) => { measure_flex_layout(entity,flex,request_size,element,params) }
    };
   
    
    if let Ok(mut rect2d) = unsafe { params.rect2ds.get_unchecked(entity) } {
        rect2d.width  = measure_size.x;
        rect2d.height = measure_size.y;
    }
    measure_size
}

fn measure_empty_element(entity:Entity,request_size:Vec2,params:LayoutParams) -> Vec2 {
    if let Ok(childs) = params.childrens.get(entity) {
        for child_entity in childs.iter() {
           
        }
    }
    Vec2::ZERO
}

//没有强制能撑开，有强制撑不开, request_size就是强制Size
fn measure_view_layout(entity:Entity,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let fixed_size = element.common.get_fixed_size(request_size, params.rect2ds.get(entity).ok());
    
    let padding = &element.common.padding;
    let mut inner_size = Vec2::new(fixed_size.x - padding.horizontal(), fixed_size.y - padding.vertical());
    let mut ret_size = fixed_size;
    if let Ok(children) = params.childrens.get(entity) {
        let mut is_dirty = false;
        for child_entity in children.iter() {
            if let Ok(child_elem) = params.elems.get(*child_entity) {
                let child_size = measure_layout_element(*child_entity, inner_size, child_elem, params);
                if fixed_size.x < 0f32 && child_size.x > inner_size.x {
                    inner_size.x = child_size.x;
                    ret_size.x = child_size.x + padding.horizontal();
                    is_dirty = true;
                }
                if fixed_size.y < 0f32 && child_size.y > ret_size.y {
                    inner_size.y = child_size.y;
                    ret_size.y = child_size.y + padding.vertical();
                    is_dirty = true;
                }
            }
        }

        //被子节点撑开了布局，其他子节点的Stretch要重新计算
        if is_dirty {
            for child_entity in children.iter() {
                if let Ok(child_elem) = params.elems.get(*child_entity) {
                    measure_layout_element(*child_entity, inner_size, child_elem, params);
                }
            }
        }
    }
    
    ret_size
}

////Stack System
fn measure_stack_layout(entity:Entity,stack:&StackLayout,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let fixed_size = element.common.get_fixed_size(request_size, params.rect2ds.get(entity).ok());
    let padding = &element.common.padding;
    let mut inner_size = Vec2::new(fixed_size.x - padding.horizontal(), fixed_size.y - padding.vertical());
    let mut ret_size = fixed_size;
    match stack.orientation {
        Orientation::Horizontal => {
            if fixed_size.x < 0f32 {
                ret_size.x = element.common.padding.horizontal();
            }
            inner_size.x = 0f32;
        },
        Orientation::Vertical => { 
            if fixed_size.y < 0f32 {
                ret_size.y = element.common.padding.vertical();
            }
            inner_size.y  = 0f32;
        }
    }
    let childs = params.childrens.get(entity);
    if let Ok(childs) = childs {
        let mut is_dirty = false;
        for child_entity in childs.iter() {
            if let Ok(child_element) = params.elems.get(*child_entity) {
                let child_size = measure_layout_element(*child_entity, inner_size, child_element, params);
                
                match stack.orientation {
                    Orientation::Horizontal => {
                        if fixed_size.x < 0f32 {
                            ret_size.x += child_size.x + stack.spacing;
                        }
                        
                        if fixed_size.y < 0f32 && ret_size.y < child_size.y {
                            ret_size.y = child_size.y;
                            inner_size.y = child_size.y - padding.vertical();
                            is_dirty = true;
                        }
                    },
                    Orientation::Vertical => { 
                        if fixed_size.y < 0f32 {
                            ret_size.y += child_size.y + stack.spacing;
                        }
                        if fixed_size.x < 0f32 && ret_size.x < child_size.x {
                            ret_size.x = child_size.x;
                            inner_size.x = child_size.x - padding.vertical();
                            is_dirty = true;
                        }
                    }
                }
            }
        }

        //被子节点撑开了垂直方向的布局，其他子节点的Stretch要重新计算
        if is_dirty {
            for child_entity in childs.iter() {
                if let Ok(child_element) = params.elems.get(*child_entity) {
                    measure_layout_element(*child_entity, inner_size, child_element, params);
                }
            }
        }
    }
    
    ret_size
}


fn measure_flex_layout(entity:Entity,flex:&FlexLayout,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    match flex.warp {
        FlexWrap::NoWrap => measure_flex_no_wrap_layout(entity,flex,request_size,element,params),
        FlexWrap::Wrap => measure_flex_no_wrap_layout(entity,flex,request_size,element,params)
    }
   
    
}

fn measure_flex_no_wrap_layout(entity:Entity,flex:&FlexLayout,request_size:Vec2,element:&LayoutElement,params:&LayoutParams) -> Vec2 {
    let fixed_size = element.common.get_fixed_size(request_size, params.rect2ds.get(entity).ok());
    let (main_axis_fixed_size,cross_axis_fixed_size) = match flex.direction {
        FlexDirection::Column | FlexDirection::ColumnReverse => { (fixed_size.y,fixed_size.x) },
        FlexDirection::Row | FlexDirection::RowReverse => { (fixed_size.x,fixed_size.y) }
    };
    //子元素占据主轴尺寸
    let mut main_axis_all_child_size:f32 = 0f32;
    //交叉轴被撑开尺寸
    let mut cross_axis_max_size:f32 = 0f32;
    for_each_child(entity, params, |child_entity,params| {
        if let Ok(rect) = params.rect2ds.get(child_entity) {
            match flex.direction {
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    main_axis_all_child_size += rect.height;
                    cross_axis_max_size = cross_axis_max_size.max(rect.width);
                },
                FlexDirection::Row | FlexDirection::RowReverse => {
                    main_axis_all_child_size += rect.width;
                    cross_axis_max_size = cross_axis_max_size.max(rect.height);
                }
            }
       }
    });
    
    let cross_real_size = if cross_axis_fixed_size > 0f32 { cross_axis_fixed_size } else { cross_axis_max_size  };

    //主轴固定尺寸
    if main_axis_fixed_size > 0f32 {
        //子元素占据主轴尺寸 > 主轴固定尺寸
        if main_axis_all_child_size > main_axis_fixed_size {
            //挤压重排
            let shrink1_size = calc_shrink1_size(entity,flex.direction,main_axis_fixed_size,params);
            for_each_child(entity, params, |child_entity,params| {
                if let Ok(flex_item) = params.flexitems.get(child_entity) {
                    //可挤压元素
                    if flex_item.shrink > 0f32 {
                       let main_axis_size = shrink1_size * flex_item.shrink;
                       let child_size = match flex.direction {
                           FlexDirection::Column | FlexDirection::ColumnReverse => {
                              Vec2::new(cross_real_size, main_axis_size)
                           },
                           FlexDirection::Row | FlexDirection::RowReverse => {
                              Vec2::new(main_axis_size,cross_real_size)
                           }
                       };
                      
                    } else {

                    }

                }
            })
        }
    }


    /*
    let 子元素占据主轴尺寸 = 取主轴方向所有Child尺寸();
       let 交叉轴被撑开尺寸 =  取交叉轴方向最大Child尺寸();
       let 交叉轴实际尺寸 =  if(固定交叉轴) { 固定交叉轴 } else { 交叉轴被撑开最大尺寸 }
 
       if(主轴固定尺寸) {
            if(子元素占据主轴尺寸 > 主轴固定尺寸) {
                //挤压重排
                let 可挤压单位一尺寸 = 计算可挤压尺寸单位1();
                for(子元素) {
                    if(可挤压元素) {
                        更新元素大小(可挤压尺寸,交叉轴实际尺寸);
                    } else if(交叉轴被撑开了) {
                        更新元素大小(当前尺寸,交叉轴实际尺寸);
                    }
                }
            } elseif(子元素有grow属性) {
                //放大子元素 重排
                let 可放大单位一尺寸 = 计算可放大尺寸单位1();
                ...逻辑同上
            }
       } else { //主轴自由尺寸
            if(子元素占据主轴尺寸 > 申请尺寸) {
                //挤压重排 同上
            } else if(交叉轴被撑开了){
               更新元素大小(当前尺寸,交叉轴实际尺寸);
            }
       }
    
    */
    Vec2::ZERO
}

//计算挤压的单位一尺寸
fn calc_shrink1_size(entity:Entity,dir:FlexDirection,max_axis_size:f32,params:&LayoutParams) -> f32 {
    let mut all_free_size:f32 = max_axis_size;
    let mut all_shrink = 0f32;
    for_each_child(entity,params,|child_entity:Entity,params: &LayoutParams| {
        if let Ok(flex_item) = params.flexitems.get(child_entity) {
            if flex_item.shrink <= 0f32 {
                if let Ok(rect2d) = params.rect2ds.get(child_entity) {
                    match dir {
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            all_free_size -= rect2d.height;
                        },
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            all_free_size -= rect2d.width;
                        }
                    }
                }
            } else {
                all_shrink += flex_item.shrink;
            }
        } else {
            //没有默认为1    
            all_shrink += 1f32;
        }
    }); 

    if all_shrink > 0f32 { all_free_size / all_shrink } else { 0f32 }
}

fn for_each_child<F>(entity:Entity,params:&LayoutParams,mut f:F) where F:FnMut(Entity,&LayoutParams) {
    if let Ok(childs) = params.childrens.get(entity) {
        for child_entity in childs.iter() {
            f(*child_entity,params);
        }
    }
}