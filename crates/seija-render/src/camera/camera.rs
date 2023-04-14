use bevy_ecs::prelude::Component;
use glam::Mat4;
use seija_asset::Handle;
use smol_str::SmolStr;
use crate::resource::Texture;

#[derive(Clone,Copy)]   
#[repr(u8)]
pub enum SortType {
    Distance,
    Z
}

#[derive(Component)]
#[repr(C)]
pub struct Camera {
    pub order:i32,
    pub layer:i32,
    pub cull_type:i32,
    pub is_hdr:bool,
    pub sort_type:SortType,

    pub projection:Projection,
    pub path:SmolStr,
    pub target:Option<Handle<Texture>>,
 }
 
 impl Default for Camera {
    fn default() -> Self {
       Self { 
          projection:Projection::Perspective(Perspective::default()),
          path:"Foward".into(),
          order:0,
          target:None,
          layer:1,
          cull_type:-1,
          is_hdr:false,
          sort_type:SortType::Distance
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

  
 }

 #[derive(Debug,Clone)]
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
#[derive(Debug,Clone)]
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
            near: 0.001,
            far: 100.0,
         }
    }
}

impl Orthographic {
    pub fn new(size:f32) -> Orthographic {
        Orthographic { left: -size, right: size, bottom: -size, top: size, near: 0.001, far: 100f32 }
    }
    pub fn proj_matrix(&self) -> Mat4  {
        /*
            //左手坐标系
             * P =  2/r-l    0         0       - r+l/r-l
             *       0      2/t-b      0       - t+b/t-b
             *       0       0       2/F-N    - F+N/F-N
             *       0       0         0            1
        */
        /* 
        Mat4::from_cols(Vec4::new(2f32 / (self.right - self.left), 0f32, 0f32, 0f32),
                        Vec4::new(0f32, 2f32 / (self.top - self.bottom), 0f32, 0f32),
                        Vec4::new(0f32, 0f32, 2f32 / (self.far - self.near), 0f32),
                        Vec4::new(-(self.right + self.left) / (self.right - self.left), -(self.top + self.bottom) / (self.top - self.bottom), 
                                          -(self.far + self.near) / (self.far - self.near), 1f32))
        */
        //右手坐标系
        Mat4::orthographic_rh(self.left, self.right, self.bottom, self.top, self.near, self.far)
    } 
}

#[derive(Debug,PartialEq, Eq,Clone,Copy)]
#[repr(u8)]
pub enum FovDirection {
    Ver,
    Hor,
}

#[derive(Debug,Clone)]
pub struct  Perspective {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub dir:FovDirection
}

impl Default for Perspective {
    fn default() -> Self {
        Perspective {
            fov: 60f32.to_radians(),
            near: 0.01f32,
            far: 100.0,
            aspect_ratio: 4.0 / 3.0,
            dir:FovDirection::Ver
        }
    }

}

impl Perspective {
    /* 
    fn frustum(left:f32,right:f32,bottom:f32,top:f32,near:f32,far:f32) -> Mat4 {
        /* P =      2N/r-l    0      l+r/l-r        0
             *       0      2N/t-b   b+t/b-t        0
             *       0        0      f+n/f-n        2nf/n-f
             *       0        0        -1           0
             */
        Mat4::from_cols(Vec4::new((2f32 * near) / (right - left), 0f32, 0f32, 0f32), 
                        Vec4::new(0f32, (2f32 * near) / (top - bottom), 0f32, 0f32), 
                        Vec4::new((left + right) / (left - right), (bottom + top) / (bottom - top), (far + near) / (far - near), 1f32), 
                        Vec4::new(0f32, 0f32, (2f32 * near * far) / (near - far), 0f32))
    }*/

    fn proj_matrix(&self) -> Mat4 {
        /* 
        let w;
        let h;
        let s = (self.fov / 2.0f32).tan() * self.near;
        if self.dir == FovDirection::Ver {
            w = s * self.aspect_ratio;
            h = s;
        } else {
            w = s;
            h = s / self.aspect_ratio;
        }
        Self::frustum(-w, w, -h, h, self.near, self.far)*/

        //右手坐标系
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
        //Mat4::perspective_infinite_reverse_rh(self.fov, self.aspect_ratio, self.near)
    }
}
