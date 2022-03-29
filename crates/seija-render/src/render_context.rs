use std::{sync::Arc, path::Path};

use wgpu::{CommandEncoder, Device};

use crate::{ material::{MaterialSystem, PassDef}, 
resource::RenderResources,  rt_shaders::RuntimeShaderInfo, uniforms::UBOContext};

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
pub struct RenderContext {
    pub device:Arc<Device>,
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
    pub material_sys:MaterialSystem,
    pub shaders:RuntimeShaderInfo,
    pub ubo_ctx:UBOContext
}

impl RenderContext {
    pub fn create_bind_group_layouts(&self,pass_def:&PassDef) -> Option<Vec<&wgpu::BindGroupLayout>>  {
        let mut ret = vec![];
        let rt_shader = self.shaders.find_shader(&pass_def.shader_info.name)?;
        let ubos = self.ubo_ctx.info.get_ubos_by_backends(&rt_shader.backends);
        for (ubo_name,_) in ubos.iter() {
           let layout = self.ubo_ctx.info_layouts.get(ubo_name)?;
           ret.push(layout);
        }
        Some(ret)
    }

    pub fn new<P:AsRef<Path>>(device:Arc<Device>,config_path:P) -> Self {
        let mut shaders = RuntimeShaderInfo::default();
        shaders.load(config_path);
        let ctx = RenderContext {
            device:device.clone(),
            command_encoder:None,
            resources:RenderResources::new(device.clone()),
            material_sys:MaterialSystem::new(&device),
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