use std::path::Path;

use glsl_pack_rtbase::rt_shaders::{RuntimeShaders, RTShaderInfo};

#[derive(Default)]
pub struct RuntimeShaderInfo {
    rt_shaders:RuntimeShaders
}

impl RuntimeShaderInfo {
    pub fn load<P:AsRef<Path>>(&mut self,p:P) {
        let path = &p.as_ref().join("shaders/rt.json");
        if let Some(shader) = RuntimeShaders::read_from(path) {
            self.rt_shaders = shader;
        } else {
            log::error!("load runtime shaders error {:?}",path);
        }  
    }

    pub fn find_shader(&self,key:&str) -> Option<&RTShaderInfo> {
        
        self.rt_shaders.shaders.get(key)
    }
}