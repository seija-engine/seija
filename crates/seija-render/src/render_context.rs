use std::sync::Arc;

use wgpu::{CommandEncoder, Device};

use crate::{TransformBuffer, camera::system::CameraState, light::LightState, material::{MaterialSystem, PassDef}, resource::RenderResources, uniforms::GPUUniformList, rt_shaders::RuntimeShaderInfo};

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
pub struct RenderContext {
    pub device:Arc<Device>,
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
    pub camera_state:CameraState,
    pub light_state:LightState,
    pub transform_buffer:TransformBuffer,
    pub material_sys:MaterialSystem,
    pub uniforms:GPUUniformList,
    pub shaders:RuntimeShaderInfo
}

impl RenderContext {
    pub fn create_bind_group_layout(&self,pass_def:&PassDef) -> Vec<&wgpu::BindGroupLayout> {
        if let Some(shader_info) = self.shaders.find_shader(&pass_def.shader_info.name) {
            vec![]
        } else {
            log::error!("not found shader:{}",pass_def.shader_info.name);
            vec![]
        }
    }
}

/*
  GPUUniformList
     Camera3D
     Transform3D
     Light3D
*/