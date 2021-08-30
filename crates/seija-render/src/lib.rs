use std::borrow::{Borrow, BorrowMut};
use render::{AppRender, Config, RenderGraphContext};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_core::{CoreStage, StartupStage};
use seija_core::window::AppWindow;



pub mod graph;
pub mod resource;
mod render;


pub struct RenderModule;

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        let app_render = AppRender::new_sync(Config::default());
        app.add_resource(app_render);
        app.add_system2(CoreStage::Startup, StartupStage::Startup,on_start_up.exclusive_system());

        let render_system = get_render_system(&mut app.world);
        app.add_system(CoreStage::Update, render_system.exclusive_system());
    }
}



fn on_start_up(mut render:ResMut<AppRender>,window:Res<AppWindow>) {
    let render_mut = render.borrow_mut();
    let window_ref:&AppWindow = window.borrow();

    let surface = unsafe { render_mut.instance.create_surface(window_ref) };
    render_mut.resources.set_main_surface(surface);
    render_mut.resources.create_swap_chain(window.width(), window.height(), window.vsync());
}

fn get_render_system(world:&mut World) -> impl FnMut(&mut World) {
    let mut app_render = AppRender::new_sync(Config::default());
    let mut graph_ctx = RenderGraphContext::default();
    add_base_nodes(&mut graph_ctx);
    world.insert_resource(graph_ctx);
    move |world| { app_render.update(world); }
}

fn add_base_nodes(graph_ctx:&mut RenderGraphContext) {
    use crate::graph::nodes::LogNode;
  
    let node_a = graph_ctx.graph.add_node("node_a",  LogNode("a".into()));
    let node_b = graph_ctx.graph.add_node("node_b", LogNode("b".into()));
    let node_c = graph_ctx.graph.add_node("node_c", LogNode("c".into()));
    let node_d = graph_ctx.graph.add_node("node_d", LogNode("d".into()));
    graph_ctx.graph.add_link(node_a, node_b).unwrap();
    graph_ctx.graph.add_link(node_b, node_c).unwrap();
    graph_ctx.graph.add_link(node_d, node_b).unwrap();

    graph_ctx.build_iter();
}