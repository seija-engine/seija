use seija_core::bytes::{ Byteable};

unsafe impl Byteable for LightEnvInner {}

pub struct LightEnv {
    pub is_dirty:bool,
    inner:LightEnvInner
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
}

#[repr(C)]
pub struct LightEnvInner {
    ambient_color:[f32;4],
    directional_dir:[f32;4],
    directional_color:[f32;4]
}

impl Default for LightEnvInner {
    fn default() -> Self {
        Self { 
            ambient_color: [0.01f32, 0.01f32, 0.01f32, 1f32],
            directional_dir:[0.5f32, 0.5f32, 0f32,0f32],
            directional_color:[1f32,1f32,1f32,1f32]
        }
    }
}