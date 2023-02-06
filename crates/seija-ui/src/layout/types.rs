use bevy_ecs::{prelude::Component};
use seija_core::math::Vec2;

use crate::{types::Thickness, components::rect2d::Rect2D};

use super::comps::{StackLayout, Orientation};

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
pub enum LayoutAlignment {
    Start = 0,
	Center = 1,
	End = 2,
	Stretch = 3
}

#[derive(Component)]
pub struct CommonView {
    pub size:Vec2,
    pub margin: Thickness,
    pub padding: Thickness,
    pub hor: LayoutAlignment,
    pub ver: LayoutAlignment,
    pub use_rect_size:bool,
}

impl Default for CommonView {
    fn default() -> Self {
        CommonView {
            size:Vec2::new(-1f32, -1f32),
            margin:Thickness::default(),
            padding:Thickness::default(),
            hor:LayoutAlignment::Stretch,
            ver:LayoutAlignment::Stretch,
            use_rect_size:false
        }
    }
}

impl CommonView {
    pub fn get_content_size<'w,'s>(&self,request_size:Vec2,rect2d:Option<&Rect2D>) -> Vec2 {
        let mut ret_size = self.get_size(rect2d);
        if ret_size.x <= 0f32 && self.hor == LayoutAlignment::Stretch {
            ret_size.x = request_size.x - self.margin.horizontal();
        }
        if ret_size.y <= 0f32 && self.ver == LayoutAlignment::Stretch {
            ret_size.y = request_size.y - self.margin.vertical();
        }
        ret_size
    }

    pub fn get_size<'w,'s>(&self,rect2d:Option<&Rect2D>) -> Vec2 {
        if self.use_rect_size {
            rect2d.map(|rect| Vec2::new(rect.width,rect.height)).unwrap_or(Vec2::ZERO)
        } else {
            self.size
        }
    }
}

pub enum TypeElement {
    Stack(StackLayout),
    ViewBox
}

#[derive(Component)]
pub struct LayoutElement {
    pub common:CommonView,
    pub typ_elem:TypeElement,
}


impl LayoutElement {
    pub fn create_stack(spacing:f32,orientation:Orientation) -> LayoutElement {
        LayoutElement {
            common:CommonView::default(),
            typ_elem:TypeElement::Stack(StackLayout {spacing,orientation})
        }
    }
    
    pub fn is_invalid_measure(&self,child:&LayoutElement) -> bool {
        //TODO 判断布局是否失效
        true
    }

    
}