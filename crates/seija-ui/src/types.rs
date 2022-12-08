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