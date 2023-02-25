use bevy_ecs::prelude::Component;


#[derive(Clone, Copy,Hash,PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct StackLayout {
    pub spacing:f32,
    pub orientation:Orientation
}

pub struct FlexLayout {
    pub direction:FlexDirection,
    pub warp:FlexWrap,
    pub justify:FlexJustify,
    pub align_items:FlexAlignItems,
    pub align_content:FlexAlignContent   
}

impl FlexLayout {
    pub fn is_hor(&self) -> bool {
        match self.direction {
            FlexDirection::Column | FlexDirection::ColumnReverse => false,
            _ => true
        }
    } 
}

#[derive(Component)]
pub struct FlexItem {
    pub order:i32,
    pub grow:u32,
    pub shrink:f32,
    pub basis:FlexBasis,
    pub align_self:FlexAlignSelf
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