use bevy_ecs::{prelude::{Component}};
use seija_core::math::Vec2;
use num_enum::FromPrimitive;

#[derive(Clone,Debug,Copy,Hash,PartialEq, Eq,FromPrimitive)]
#[repr(u8)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}
#[repr(C)]
pub struct StackLayout {
    pub spacing:f32,
    pub orientation:Orientation
}



pub struct FlexLayout {
    pub direction:FlexDirection,
    pub warp:FlexWrap,
    pub justify:FlexJustify,
    //交叉轴每行的对齐方式
    pub align_items:FlexAlignItems,
    //交叉轴多行整体的对齐方式
    pub align_content:FlexAlignContent
}

impl FlexLayout {
    pub fn is_hor(&self) -> bool {
        match self.direction {
            FlexDirection::Column | FlexDirection::ColumnReverse => false,
            _ => true
        }
    }

    pub fn get_axis_size(&self, size:Vec2) -> Vec2 {
        if self.is_hor() {
            Vec2::new(size.x, size.y)
        } else {
            Vec2::new(size.y, size.x)
        }
    }
}

impl Default for FlexLayout {
    fn default() -> Self {
        FlexLayout {
            direction: FlexDirection::Row,
            warp: FlexWrap::NoWrap,
            justify: FlexJustify::Start,
            align_items: FlexAlignItems::Stretch,
            align_content: FlexAlignContent::Stretch
        }
    }
}

#[derive(Component)]
pub struct FlexItem {
    pub order:i32,
    pub grow:f32,
    pub shrink:f32,
    pub basis:FlexBasis,
    pub align_self:FlexAlignSelf
}

impl Default for FlexItem {
    fn default() -> Self {
        FlexItem {
            order: 0,
            grow: 0f32,
            shrink: 1f32,
            basis: FlexBasis { length: 0f32, is_relative: false },
            align_self: FlexAlignSelf::Auto
        }
    }
}


#[derive(Clone, Copy,Hash,PartialEq, Eq)]
pub enum FlexDirection {
    //行
    Row,
    RowReverse,
    //列
    Column,
    ColumnReverse,
    
}


#[derive(Clone, Copy,Hash,PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap
}

#[derive(Clone, Copy,Hash,PartialEq, Eq)]
pub enum FlexJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround
}

#[derive(Clone, Copy,Hash,PartialEq, Eq)]
pub enum FlexAlignItems {
    Stretch,
    Center,
    Start,
    End
}

#[derive(Clone, Copy,Hash,PartialEq, Eq)]
pub enum  FlexAlignContent {
    Stretch,
    Center,
    Start,
    End,
    SpaceBetween,
    SpaceAround
}

#[derive(Clone, Copy,Hash,PartialEq, Eq)]
pub enum FlexAlignSelf {
    Auto,
    Stretch,
    Center,
    Start,
    End
}

pub struct FlexBasis {
    pub length:f32,
    pub is_relative:bool
}
