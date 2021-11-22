use glam::Vec3;
use seija_core::bytes::{ Byteable};

unsafe impl Byteable for LightEnvInner {}

pub struct LightEnv {
    pub is_dirty:bool,
    pub inner:LightEnvInner
}

impl Default for LightEnv {
    fn default() -> Self {
        Self { 
            is_dirty: true, 
            inner: Default::default() 
        }
    }
}

impl LightEnv {
    pub fn inner(&self) -> &LightEnvInner {
        &self.inner
    }

    pub fn set_directional(&mut self,dir:Vec3) {
        self.inner.directional_dir[0] = dir.x;
        self.inner.directional_dir[1] = dir.y;
        self.inner.directional_dir[2] = dir.z;
        self.is_dirty = true;
    }   
}

#[repr(C)]
pub struct LightEnvInner {
    ambient_color:[f32;4],
    pub directional_dir:[f32;4],
    directional_color:[f32;4]
}

impl Default for LightEnvInner {
    fn default() -> Self {
       
        Self { 
            ambient_color: [0.05f32, 0.05f32, 0.05f32, 1f32],
            directional_dir:[0.5f32, 0.5f32, 0f32,1f32],
            directional_color:[1f32,1f32,1f32,1f32]
        }
    }
}