use bevy_ecs::prelude::{Component, Entity};
use seija_core::math::Vec2;

#[derive(Debug,Clone,Default)]
pub struct Rect<T:Default> {
    pub x:T,
    pub y:T,
    pub width:T,
    pub height:T
}


#[derive(PartialEq,Clone,Default)]
pub struct Thickness {
   pub left:f32,
   pub top:f32,
   pub right:f32,
   pub bottom:f32
}


impl Thickness {
    pub fn new1(num:f32) -> Self {
        Thickness { left: num, top: num, right: num, bottom: num }
    }

    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }

    pub fn apply2size(&self,size:Vec2) -> Vec2 {
        Vec2::new(size.x + self.horizontal(),size.y + self.vertical())
    }
}

#[derive(Component)]
pub struct UIZOrder {
    pub last:i32,
    pub value:i32,
}