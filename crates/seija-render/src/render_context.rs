use std::{sync::Arc};
use crate::pipeline::{PipelineKey,PipelineCache};
use bevy_ecs::system::Resource;
use fnv::FnvHasher;
use seija_asset::AssetServer;
use wgpu::{CommandEncoder, Device, TextureFormat};
use std::hash::{Hash,Hasher};
use crate::{ material::{MaterialSystem, PassDef, MaterialDef}, 
resource::{RenderResources, Mesh},  rt_shaders::RuntimeShaderInfo, uniforms::{UniformContext}, graph_setting::GraphSetting, RenderConfig};

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}

#[derive(Resource)]
pub struct RenderContext {
    pub device:Arc<Device>,
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
    pub material_system:MaterialSystem,
    pub shaders:RuntimeShaderInfo,
    pub ubo_ctx:UniformContext,
    pub setting:Arc<GraphSetting>,
    pub pipeline_cache:PipelineCache,
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

    pub fn new(device:Arc<Device>,config:Arc<RenderConfig>,assets:&AssetServer) -> Self {
        let mut shaders = RuntimeShaderInfo::default();
        shaders.load(&config.config_path);
        let ctx = RenderContext {
            device:device.clone(),
            command_encoder:None,
            resources:RenderResources::new(device.clone(),assets),
            material_system:MaterialSystem::new(&device),
            shaders,
            ubo_ctx:UniformContext::default(),
            setting:config.setting.clone(),
            frame_draw_pass:0,
            pipeline_cache:PipelineCache::new(config)
        };
       
         
        ctx
    }

    pub fn build_pipeine(&mut self,mat_def:&MaterialDef,mesh:&Mesh,formats:&Vec<TextureFormat>,depth_format:Option<wgpu::TextureFormat>,pass_index:usize) {
        let mut hasher = FnvHasher::default();
        PipelineKey(mat_def.name.as_str(),mesh.layout_hash_u64(),formats,depth_format,pass_index).hash(&mut hasher);
        let key = hasher.finish();
        
        if !self.pipeline_cache.cache_pipelines.contains_key(&key) {
            //log::error!("in key:{}",&key);
            match self.pipeline_cache.compile_pipeline(mesh,&mat_def.pass_list[pass_index],self,mat_def,formats,depth_format) {
                Ok(None) => {
                    log::info!("wait create {}",mat_def.name.as_str());
                },
                Ok(Some(pipe)) => {
                    self.pipeline_cache.cache_pipelines.insert(key, pipe);
                }
                Err(err) => {
                    log::error!("create pipeline fail {} {:?}",mat_def.name,err);
                },
            }
        }
    }
}