use std::path::{PathBuf, Path};

use camera::system::CameraState;
use camera::{view_list::view_list_system};
use graph::nodes::SwapchainNode;
use light::LightState;
use material::MaterialSystem;
use pipeline::{PipelineCache, update_pipeline_cache};
use render::{AppRender, Config , RenderGraphContext};
use resource::{RenderResources};
use rt_shaders::RuntimeShaderInfo;
use script::{RenderScriptContext, builtin_node_creators};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_core::{CoreStage};
use crate::graph::nodes::{PassNode, WindowTextureNode};
#[macro_use]
extern crate serde_derive;

pub use wgpu;
mod script;
pub mod material;
pub mod camera;
pub mod graph;
pub mod resource;
pub mod pipeline;
pub mod light;

mod uniforms;
mod rt_shaders;
mod mesh_render;
mod render_context;
mod render;
mod memory;
mod transform_buffer;

pub use transform_buffer::TransformBuffer;
pub use render_context::{RenderContext};
pub use uniforms::{UBOInfoSet,UBOInfo};

const MATRIX_SIZE: u64 = std::mem::size_of::<[[f32; 4]; 4]>() as u64;

#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel )]
pub enum RenderStage {
    AfterRender,
    Render,
    PostRender,
}

#[derive(Default)]
pub struct RenderConfig {
    config_path:PathBuf
}

impl RenderConfig {
    pub fn set_config_path<P:AsRef<Path>>(&mut self,path:P) {
        self.config_path = path.as_ref().into();
    }
}

#[derive(Default)]
pub struct RenderModule(pub RenderConfig);

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        
        resource::init_resource(app);
        material::init_material(app);
        light::init_light(app);

        let render_system = self.get_render_system(&mut app.world);
        app.schedule.add_stage_after(CoreStage::PostUpdate, RenderStage::AfterRender, SystemStage::parallel());
        app.schedule.add_stage_before(RenderStage::AfterRender, RenderStage::Render, SystemStage::single(render_system.exclusive_system()));
        app.schedule.add_stage_before(RenderStage::Render, RenderStage::PostRender, SystemStage::parallel());

        

        app.add_system(RenderStage::AfterRender, update_pipeline_cache.system());
        app.add_system(CoreStage::PostUpdate, view_list_system.system());
    }
}


impl RenderModule {
    fn get_render_system(&self,w:&mut World) -> impl FnMut(&mut World) {
        let mut app_render = AppRender::new_sync(Config::default());
        let render_ctx = RenderContext::new(app_render.device.clone(),&self.0.config_path);
        self.init_render(w,render_ctx,&mut app_render); 
        move |_w| {
            _w.resource_scope(|world:&mut World,mut ctx:Mut<RenderContext>| {
                app_render.update(world,&mut ctx); 
            }); 
        }
    }

    fn init_render(&self,w:&mut World,mut ctx:RenderContext,app_render:&mut AppRender) {
        let mut rsc = RenderScriptContext::new();
        let creator_set = builtin_node_creators();
        rsc.add_node_creator_set(&creator_set);
        let script_path = self.0.config_path.join("render.clj");
        match std::fs::read_to_string(script_path) {
            Ok(code_string) => {
                rsc.run(code_string.as_str(), &mut ctx.ubos.info,&mut app_render.graph);
            },
            Err(err) => {
                log::error!("load render.clj error:{:?}",err);
            }
        }
        
        w.insert_resource(PipelineCache::default());
        w.insert_resource(ctx);
        add_base_nodes(&mut app_render.graph);
    }
}


fn add_base_nodes(graph_ctx:&mut RenderGraphContext) {
    let pass_node = PassNode::new();
    let pass_id = graph_ctx.graph.add_node("pass", pass_node);

    let swap_chain_node = SwapchainNode::new();
    let swap_id = graph_ctx.graph.add_node("swapchain", swap_chain_node);

    let depth_node = WindowTextureNode::new(wgpu::TextureDescriptor { 
        label: None,
        size: wgpu::Extent3d::default(),
        mip_level_count: 1,
        sample_count: 1, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Depth32Float, 
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT 
    });
    let depth_node_id = graph_ctx.graph.add_node("depth", depth_node);

    graph_ctx.graph.add_link( swap_id ,pass_id ,    vec![0], vec![0]).unwrap();
    graph_ctx.graph.add_link( depth_node_id ,pass_id , vec![0], vec![1]).unwrap();


    graph_ctx.build();
}