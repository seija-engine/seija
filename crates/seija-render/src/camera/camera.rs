use glam::Mat4;

pub enum Projection {
    Ortho(Orthographic)
}

impl Projection {
    pub fn matrix(&self) -> Mat4 {
        match self {
            Projection::Ortho(o) => o.proj_matrix(),
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


pub struct Camera {
   pub projection:Projection
}

impl Camera {
    pub fn from_2d(ortho:Orthographic) -> Camera {
        Camera { projection:Projection::Ortho(ortho) }
    }
}