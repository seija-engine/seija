use seija_core::math::Vec2;

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
}

#[derive(Debug)]
pub struct Rect2D {
   pub width:f32,
   pub height:f32,
   pub anchor:Vec2
}

impl Default for Rect2D {
    fn default() -> Self {
        Rect2D { width: 0f32, height: 0f32, anchor:Vec2::new(0.5f32, 0.5f32) }
    }
}