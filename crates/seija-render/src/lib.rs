use std::borrow::{Borrow, BorrowMut};
use camera::view_list::view_list_system;
use render::{AppRender, Config,RenderContext, RenderGraphContext};
use resource::RenderResources;
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_core::{CoreStage, StartupStage};
use seija_core::window::AppWindow;
use wgpu::CommandEncoderDescriptor;

use crate::graph::nodes::PassNode;

pub mod material;
pub mod camera;
pub mod graph;
pub mod resource;
mod render;


pub struct RenderModule;

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        let render_system = get_render_system();
        app.add_system(CoreStage::Update, render_system.exclusive_system());
        app.add_system(CoreStage::PostUpdate, view_list_system.system());
    }
}

fn get_render_system() -> impl FnMut(&mut World) {
    let mut app_render = AppRender::new_sync(Config::default());
    let command_buffer = app_render.device.create_command_encoder(&CommandEncoderDescriptor::default());
    let mut graph_ctx = RenderGraphContext::default();
    let mut render_ctx = RenderContext {
        command_buffer,
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