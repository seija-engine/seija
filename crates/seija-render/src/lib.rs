use camera::{view_list::view_list_system,camera::CamerasBuffer};
use render::{AppRender, Config,RenderContext, RenderGraphContext};
use resource::RenderResources;
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_core::{CoreStage};
use wgpu::CommandEncoderDescriptor;
use crate::graph::nodes::PassNode;

pub mod material;
pub mod camera;
pub mod graph;
pub mod resource;
mod render;

const MATRIX_SIZE: u64 = std::mem::size_of::<[[f32; 4]; 4]>() as u64;

pub struct RenderModule;

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        app.add_resource(CamerasBuffer::default());
        let render_system = get_render_system();
        app.add_system(CoreStage::PostUpdate, render_system.exclusive_system().at_end());
        app.add_system(CoreStage::PostUpdate, view_list_system.system());
        
        material::init_material(app);
    }
}

fn get_render_system() -> impl FnMut(&mut World) {
    let mut app_render = AppRender::new_sync(Config::default());
    let command_encoder = app_render.device.create_command_encoder(&CommandEncoderDescriptor::default());
    let mut graph_ctx = RenderGraphContext::default();
    let mut render_ctx = RenderContext {
        command_encoder:Some(command_encoder),
        resources:RenderResources::new(app_render.device.clone())
    };
    add_base_nodes(&mut graph_ctx);

    move |world| { app_render.update(world,&mut graph_ctx,&mut render_ctx); }
}

fn add_base_nodes(graph_ctx:&mut RenderGraphContext) {
    let pass_node = PassNode;
    graph_ctx.graph.add_node("pass", pass_node);
    graph_ctx.build_iter();
}