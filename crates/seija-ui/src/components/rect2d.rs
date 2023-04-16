use bevy_ecs::prelude::Component;
use seija_core::math::{Vec2, Vec3, Vec4};
use seija_transform::{TransformMatrix, Transform};

#[derive(Debug,Component,Clone)]
#[repr(C)]
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


    pub fn test(&self,t:&TransformMatrix,pos:Vec2) -> bool {
        if self.width <= 0f32 || self.height <= 0f32 {
            return false;
        }
        let mat = t.matrix();
        let offset_x = self.width * self.anchor.x;
        let offset_y = self.height * self.anchor.y;
        let z = t.position.z;
        let x = - offset_x;
        let x1 = self.width - offset_x;
        let y =  self.height - offset_y;
        let y1 = - offset_y;
        let p0:Vec4 = mat * Vec4::new(x,y,z,1f32);
        let p1:Vec4 = mat * Vec4::new(x1,y,z,1f32);
        let p2:Vec4 = mat * Vec4::new(x,y1,z,1f32);

        let am = Vec2::new(pos.x - p0.x ,pos.y - p0.y);
        let ab = Vec2::new(p1.x - p0.x,p1.y - p0.y);
        let ad = Vec2::new(p2.x - p0.x,p2.y - p0.y);
        let amdab = am.dot(ab);
        let amdad = am.dot(ad);
        if 0f32 <= am.dot(ab) && amdab  <= ab.dot(ab) && 0f32 <= amdad && amdad <= ad.dot(ad) {
            return true
        }
        false
    }
}

impl Default for Rect2D {
    fn default() -> Self {
        Rect2D { width: 0f32, height: 0f32, anchor:Vec2::new(0.5f32, 0.5f32) }
    }
}