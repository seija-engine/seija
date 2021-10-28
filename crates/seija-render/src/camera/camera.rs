use glam::{EulerRot, Mat4, Quat};
use seija_transform::Transform;
use super::view_list::ViewList;

pub struct Camera {
    pub projection:Projection,
    pub view_list:ViewList,
 }
 
 impl Camera {
     pub fn from_2d(ortho:Orthographic) -> Camera {
         Camera { projection:Projection::Ortho(ortho),view_list:ViewList::default() }
     }

     pub fn from_3d(perspective:Perspective) -> Camera {
        Camera { projection:Projection::Perspective(perspective),view_list:ViewList::default() }
     }
 }

pub enum Projection {
    Ortho(Orthographic),
    Perspective(Perspective)
}

impl Projection {
    pub fn matrix(&self) -> Mat4 {
        match self {
            Projection::Ortho(o) => o.proj_matrix(),
            Projection::Perspective(p) => p.proj_matrix()
        }
    }
}

pub struct Orthographic {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self { 
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
         }
    }
}

impl Orthographic {
    pub fn proj_matrix(&self) -> Mat4  {
        Mat4::orthographic_rh(self.left,self.right,self.bottom,self.top,self.near,self.far)
    } 
}




pub struct Perspective {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Perspective {
    fn default() -> Self {
        Perspective {
            fov: std::f32::consts::PI / 4.0,
            near: 1.0,
            far: 1000.0,
            aspect_ratio: 1.0,
        }
    }
}

impl Perspective {
    fn proj_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }
}