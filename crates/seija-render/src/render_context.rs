use std::{sync::Arc, path::Path};

use seija_asset::AssetServer;
use wgpu::{CommandEncoder, Device};

use crate::{ material::{MaterialSystem, PassDef}, 
resource::RenderResources,  rt_shaders::RuntimeShaderInfo, uniforms::{UniformContext}, graph_setting::GraphSetting};

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
pub struct RenderContext {
    pub device:Arc<Device>,
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
    pub mat_system:MaterialSystem,
    pub shaders:RuntimeShaderInfo,
    pub ubo_ctx:UniformContext,
    pub setting:Arc<GraphSetting>,
    pub frame_draw_pass:u32,
}

impl RenderContext {
    pub fn create_bind_group_layouts(&self,pass_def:&PassDef) -> Option<Vec<&wgpu::BindGroupLayout>>  {
        let mut ret = vec![];
        let rt_shader = self.shaders.find_shader(&pass_def.shader_info.name)?;
        let ubos = self.ubo_ctx.info.get_ubos_by_backends(&rt_shader.get_backends(&pass_def.shader_info.features));
        for (ubo_name,_) in ubos.iter() {
           let layout = self.ubo_ctx.get_layout(ubo_name)?;
           ret.push(layout);
        }
        Some(ret)
    }

    pub fn new<P:AsRef<Path>>(device:Arc<Device>,config_path:P,setting:Arc<GraphSetting>,assets:&AssetServer) -> Self {
        let mut shaders = RuntimeShaderInfo::default();
        shaders.load(config_path);
        let ctx = RenderContext {
            device:device.clone(),
            command_encoder:None,
            resources:RenderResources::new(device.clone(),assets),
            mat_system:MaterialSystem::new(&device),
            shaders,
            ubo_ctx:UniformContext::default(),
            setting,
            frame_draw_pass:0
        };
       
         
        ctx
    }
}