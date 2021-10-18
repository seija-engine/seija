use camera::{view_list::view_list_system,camera::CamerasBuffer};
use graph::nodes::SwapchainNode;
use pipeline::{PipelineCache, update_pipeline_cache};
use render::{AppRender, Config,RenderContext, RenderGraphContext};
use resource::{Mesh, RenderResources};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_core::{CoreStage};
use crate::graph::nodes::PassNode;
use seija_asset::{AddAsset};

pub mod material;
pub mod camera;
pub mod graph;
pub mod resource;
pub mod pipeline;
mod render;
mod memory;

const MATRIX_SIZE: u64 = std::mem::size_of::<[[f32; 4]; 4]>() as u64;

#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel )]
pub enum RenderStage {
    AfterRender,
    Render,
    PostRender,
}

pub struct RenderModule;

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        let render_system = get_render_system(&mut app.world);

        app.schedule.add_stage_after(CoreStage::PostUpdate, RenderStage::AfterRender, SystemStage::parallel());
        app.schedule.add_stage_before(RenderStage::AfterRender, RenderStage::Render, SystemStage::single(render_system.exclusive_system()));
        app.schedule.add_stage_before(RenderStage::Render, RenderStage::PostRender, SystemStage::parallel());

        app.add_resource(CamerasBuffer::default());
        material::init_material(app);
        resource::init_resource(app);
        app.add_system(RenderStage::AfterRender, update_pipeline_cache.system());
        app.add_system(CoreStage::PostUpdate, view_list_system.system());
    }
}

fn get_render_system(w:&mut World) -> impl FnMut(&mut World) {
    let mut app_render = AppRender::new_sync(Config::default());
    let mut graph_ctx = RenderGraphContext::default();
    let render_ctx = RenderContext {
        device:app_render.device.clone(),
        command_encoder:None,
        resources:RenderResources::new(app_render.device.clone()),
    };
    w.insert_resource(PipelineCache::default());
    w.insert_resource(render_ctx);
    add_base_nodes(&mut graph_ctx);

    move |_w| {
        _w.resource_scope(|world:&mut World,mut ctx:Mut<RenderContext>| {
            app_render.update(world,&mut graph_ctx,&mut ctx); 
        }); 
    }
}

fn add_base_nodes(graph_ctx:&mut RenderGraphContext) {
    let pass_node = PassNode;
    let pass_id = graph_ctx.graph.add_node("pass", pass_node);

    let swap_chain_node = SwapchainNode::new();
    let swap_id = graph_ctx.graph.add_node("swapchain", swap_chain_node);


    graph_ctx.graph.add_link( swap_id ,pass_id , vec![0], vec![0]).unwrap();

    graph_ctx.build_iter();
}