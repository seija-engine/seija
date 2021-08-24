use std::borrow::{Borrow, BorrowMut};

use render::{AppRender, Config};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_core::{CoreStage, StartupStage};
use seija_core::window::AppWindow;

pub mod resource;
mod render;


pub struct RenderModule;

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        let app_render = AppRender::new_sync(Config::default());
        app.add_resource(app_render);
        app.add_system2(CoreStage::Startup, StartupStage::Startup,on_start_up.exclusive_system());
        app.add_system(CoreStage::Update, on_update.exclusive_system());
    }
}


fn on_start_up(mut render:ResMut<AppRender>,window:Res<AppWindow>) {
    let render_mut = render.borrow_mut();
    let window_ref:&AppWindow = window.borrow();

    let surface = unsafe { render_mut.instance.create_surface(window_ref) };
    render_mut.resources.set_main_surface(surface);
    render_mut.resources.create_swap_chain(window.width(), window.height(), window.vsync());
}

fn on_update(mut render:ResMut<AppRender>) {
    render.update()
}