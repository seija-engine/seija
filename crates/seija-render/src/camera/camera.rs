use bevy_ecs::prelude::Entity;
use glam::Mat4;
use seija_asset::Handle;

use crate::resource::Texture;

use super::view_list::{ViewList};
pub struct Camera {
    pub projection:Projection,
    pub view_list:ViewList,
    pub order:i32,
    pub path:String,
    pub target:Option<Handle<Texture>>
 }
 
 impl Default for Camera {
    fn default() -> Self {
       Self { 
          projection:Projection::Perspective(Perspective::default()),
          view_list:ViewList::default(),
          path:String::from("Foward"),
          order:0,
          target:None 
       }
    }
}

 impl Camera {
     pub fn from_2d(ortho:Orthographic) -> Camera {
         let mut camera = Camera::default();
         camera.projection = Projection::Ortho(ortho);
         camera
     }

     pub fn from_3d(perspective:Perspective) -> Camera {
        let mut camera = Camera::default();
        camera.projection = Projection::Perspective(perspective);
        camera
     }

     pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.view_list.values.iter().map(|v| v.value.iter()).flatten().map(|v| &v.entity)
     }
 }

 #[derive(Debug)]
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
#[derive(Debug)]
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



#[derive(Debug)]
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
            near: 0.001f32,
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