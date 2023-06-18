use bevy_ecs::prelude::{Component};
use seija_core::math::Vec2;
use num_enum::{TryFromPrimitive, IntoPrimitive};


#[derive(PartialEq,Clone,Default,Debug)]
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

    pub fn add2size(&self,size:Vec2) -> Vec2 {
        Vec2::new(size.x + self.horizontal(),size.y + self.vertical())
    }

    pub fn sub2size(&self,size:Vec2) -> Vec2 {
        Vec2::new(size.x - self.horizontal(),size.y - self.vertical())
    }
}

#[derive(Component)]
pub struct UIZOrder {
    pub last:i32,
    pub value:i32,
}

#[derive(Copy,Clone,Eq, Debug,PartialEq,TryFromPrimitive,IntoPrimitive)]
#[repr(u8)]
pub enum AnchorAlign {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight
}

#[derive(Debug,Default,Clone,Copy)]
pub struct Box2D {
  pub lt:Vec2,
  pub rb:Vec2
}

impl Box2D {
    pub fn new(l:f32,t:f32,r:f32,b:f32) -> Box2D {
        Box2D {
            lt:Vec2::new(l, t),
            rb:Vec2::new(r,b)
        }
    }

    pub fn max() -> Box2D {
        let fmax = f32::MAX;
        Box2D::new(-fmax, fmax, fmax, -fmax)
    }

    pub fn zero() -> Box2D {
        Box2D::new(0f32, 0f32, 0f32, 0f32)
    }

    pub fn intersection(&self,other:&Box2D) -> Box2D {
       if self.is_cross(other) {
          let lx = self.lt.x.max(other.lt.x);
          let rx = self.rb.x.min(other.rb.x);
          let ly = self.lt.y.min(other.lt.y);
          let ry = self.rb.y.max(other.rb.y);
          return Box2D::new(lx, ly, rx, ry);
       }
       Box2D::zero()
    }

    pub fn is_cross(&self,other:&Box2D) -> bool {
        return    self.lt.x.max(other.lt.x) <= self.rb.x.min(other.rb.x) 
               && self.lt.y.min(other.lt.y) >= self.rb.y.max(other.rb.y)
    }
}