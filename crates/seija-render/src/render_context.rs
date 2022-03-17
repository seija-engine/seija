use std::{sync::Arc, path::Path};

use wgpu::{CommandEncoder, Device};

use crate::{light::LightState, material::{MaterialSystem, PassDef}, 
resource::RenderResources,  rt_shaders::RuntimeShaderInfo, uniforms::UBOContext};

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
pub struct RenderContext {
    pub device:Arc<Device>,
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
    pub light_state:LightState,
    pub material_sys:MaterialSystem,
    pub shaders:RuntimeShaderInfo,
    pub ubo_ctx:UBOContext
}

impl RenderContext {
    //TODO
    pub fn create_bind_group_layouts(&self,pass_def:&PassDef) -> Vec<&wgpu::BindGroupLayout> {
        if let Some(shader_info) = self.shaders.find_shader(&pass_def.shader_info.name) {
            vec![]
        } else {
            log::error!("not found shader:{}",pass_def.shader_info.name);
            vec![]
        }
    }

    pub fn new<P:AsRef<Path>>(device:Arc<Device>,config_path:P) -> Self {
        let mut shaders = RuntimeShaderInfo::default();
        shaders.load(config_path);
        let ctx = RenderContext {
            device:device.clone(),
            command_encoder:None,
            resources:RenderResources::new(device.clone()),
            material_sys:MaterialSystem::new(&device),
            light_state:LightState::new(&device),
            shaders,
            ubo_ctx:UBOContext::default()
        };
       
         
        ctx
    }
}

/*
  GPUUniformList
     Camera3D
     Transform3D
     Light3D
*/