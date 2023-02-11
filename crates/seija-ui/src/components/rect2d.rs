use bevy_ecs::prelude::Component;
use seija_core::math::Vec2;

#[derive(Debug,Component,Clone)]
pub struct Rect2D {
   pub width:f32,
   pub height:f32,
   pub anchor:Vec2
}

impl Rect2D {
    pub fn new(width:f32,height:f32) -> Rect2D {
        Rect2D { width, height, anchor: Vec2::new(0.5f32, 0.5f32) }
    }

    pub fn left(&self) -> f32 {
        -self.width * self.anchor[0]
    }
    pub fn right(&self) -> f32 {
        self.width * (1f32 - self.anchor[0])
    }
    pub fn top(&self) -> f32 {
        self.height * (1f32 - self.anchor[0])
    }
    pub fn bottom(&self) -> f32 {
        -self.height * self.anchor[0]
    }
}

impl Default for Rect2D {
    fn default() -> Self {
        Rect2D { width: 0f32, height: 0f32, anchor:Vec2::new(0.5f32, 0.5f32) }
    }
}