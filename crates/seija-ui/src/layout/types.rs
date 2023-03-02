use bevy_ecs::prelude::Component;
use seija_core::math::Vec2;

use crate::{components::rect2d::Rect2D, types::Thickness};

use super::comps::{Orientation, StackLayout, FlexLayout};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LayoutAlignment {
    Start = 0,
    Center = 1,
    End = 2,
    Stretch = 3,
}
#[derive(Default,Clone, Copy)]
pub struct UISize {
   pub width:SizeValue,
   pub height:SizeValue
}

impl UISize {

    pub fn from_number(vec2:Vec2) -> UISize {
        UISize { width: SizeValue::Pixel(vec2.x), height: SizeValue::Pixel(vec2.y) }
    }

    pub fn has_auto(&self) -> bool {
        self.width.is_auto() || self.height.is_auto()
    }

    pub fn get_number_size(&self,rect2d: Option<&Rect2D>) -> Vec2 {
        let w = match self.width {
            SizeValue::Auto => 0f32,
            SizeValue::Pixel(v) => v,
            SizeValue::PixelFromRect => rect2d.map(|v| v.width).unwrap_or(0f32)
        };
        let h = match self.height {
            SizeValue::Auto => 0f32,
            SizeValue::Pixel(v) => v,
            SizeValue::PixelFromRect => rect2d.map(|v| v.height).unwrap_or(0f32)
        };
        Vec2::new(w, h)
    }
}

#[derive(Clone, Copy)]
pub enum SizeValue {
    Auto,
    Pixel(f32),
    PixelFromRect
}

impl SizeValue {
    pub fn is_auto(&self) -> bool {
        match self {
            Self::Auto => true,
            _ => false
        }
    }

    pub fn get_pixel(&self) -> f32 {
        match self {
            Self::Pixel(v) => *v,
            _ => 0f32
        }
    }
}

impl Default for SizeValue {
    fn default() -> Self { Self::Auto }
}

#[derive(Component)]
pub struct CommonView {
    pub ui_size:UISize,
    pub size: Vec2,
    pub margin: Thickness,
    pub padding: Thickness,
    pub hor: LayoutAlignment,
    pub ver: LayoutAlignment,
    pub use_rect_size: bool,
    //pub anchor_correct:bool
}

impl Default for CommonView {
    fn default() -> Self {
        CommonView {
            ui_size:UISize::default(),
            size: Vec2::new(-1f32, -1f32),
            margin: Thickness::default(),
            padding: Thickness::default(),
            hor: LayoutAlignment::Stretch,
            ver: LayoutAlignment::Stretch,
            use_rect_size: false,
            //anchor_correct:false
        }
    }
}

impl CommonView {
    pub fn get_size(&self, rect2d: Option<&Rect2D>) -> Vec2 {
        if self.use_rect_size {
            rect2d
                .map(|rect| Vec2::new(rect.width, rect.height))
                .unwrap_or(Vec2::ZERO)
        } else {
            self.size
        }
    }

    pub fn get_fixed_size(&self, request_size: Vec2, rect2d: Option<&Rect2D>) -> Vec2 {
        let mut fixed_size = self.get_size(rect2d);
        if fixed_size.x < 0f32 {
            if self.hor == LayoutAlignment::Stretch {
                fixed_size.x = request_size.x - self.margin.horizontal();
            }
        }
        if fixed_size.y < 0f32 {
            if self.ver == LayoutAlignment::Stretch {
                fixed_size.y = request_size.y - self.margin.vertical();
            }
        }
        fixed_size
    }

    
}

pub enum TypeElement {
    Stack(StackLayout),
    Flex(FlexLayout),
    View,
}

#[derive(Component)]
pub struct LayoutElement {
    pub common: CommonView,
    pub typ_elem: TypeElement,
    
    
}

impl LayoutElement {
    pub fn create_view() -> LayoutElement {
        LayoutElement {
            common:CommonView::default(),
            typ_elem:TypeElement::View
        }
    }
    
    pub fn create_stack(spacing: f32, orientation: Orientation) -> LayoutElement {
        LayoutElement {
            common: CommonView::default(),
            typ_elem: TypeElement::Stack(StackLayout {
                spacing,
                orientation,
            })
        }
    }

    pub fn is_invalid_measure(&self, child: &LayoutElement) -> bool {
        //TODO 判断布局是否失效
        true
    }
}
