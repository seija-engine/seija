use bevy_ecs::prelude::Component;
use seija_core::math::Vec2;
use num_enum::FromPrimitive;
use crate::{components::rect2d::Rect2D, types::Thickness};
use super::comps::{Orientation, StackLayout, FlexLayout};

#[derive(PartialEq, Eq, Clone, Copy, Debug,FromPrimitive)]
#[repr(u8)]
pub enum LayoutAlignment {
    #[default]
    Start = 0,
    Center = 1,
    End = 2,
    Stretch = 3,
}
#[derive(Default,Clone, Copy,Debug)]
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

#[derive(Clone, Copy,Debug)]
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

#[derive(Component,Debug)]
#[repr(C)]
pub struct CommonView {
    pub margin: Thickness,
    pub padding: Thickness,
    pub hor: LayoutAlignment,
    pub ver: LayoutAlignment,
    pub use_rect_size: bool,
    pub ui_size:UISize,
}

impl Default for CommonView {
    fn default() -> Self {
        CommonView {
            ui_size:UISize::default(),
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
    pub fn sub_margin_and_padding(&self,size:Vec2) -> Vec2 {
        Vec2::new(size.x - self.margin.horizontal() - self.padding.horizontal(),
                  size.y - self.margin.vertical() - self.padding.vertical())
    }
}

#[repr(C)]
pub enum TypeElement {
    Stack(StackLayout),
    Flex(FlexLayout),
    Free(FreeLayout),
    View,
}

#[derive(Component)]
#[repr(C)]
pub struct FreeLayout {}


#[derive(Component,Default)]
#[repr(C)]
pub struct FreeLayoutItem {
   pub pos:Vec2
}

#[derive(Component)]
#[repr(C)]
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

    pub fn create_flex(flex:FlexLayout) -> LayoutElement {
        LayoutElement {
            common: CommonView::default(),
            typ_elem: TypeElement::Flex(flex)
        }
    }

    pub fn create_free() -> LayoutElement {
        LayoutElement {
            common: CommonView::default(),
            typ_elem: TypeElement::Free(FreeLayout {  })
        }
    }

    pub fn is_invalid_measure(&self, _child: &LayoutElement) -> bool {
        if !self.common.ui_size.has_auto() {
            return false;
        }
        match self.typ_elem {
            TypeElement::Free(_) => { return false } 
            _ => ()
        }
        true
    }
}