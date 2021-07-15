use bevy_app::{App,AppBuilder};
use bevy_window::{WindowPlugin};
use bevy_winit::WinitPlugin;
use bevy_input::InputPlugin;
use bevy_wgpu::WgpuPlugin;
use bevy_render::RenderPlugin;
use bevy_asset::{ AssetPlugin, AssetServer, Handle};
use bevy_core::CorePlugin;
use bevy_log::LogPlugin;
use bevy_ecs::prelude::*;
use seija_render::{MaterialDesc,SeijaRenderPlugin};

fn main() {
    App::build()
    .add_plugin(LogPlugin::default())
    .add_plugin(CorePlugin::default())
    .add_plugin(AssetPlugin::default())
    .add_plugin(InputPlugin::default())
    .add_plugin(RenderPlugin::default())
    .add_plugin(WgpuPlugin::default())
    .add_plugin(SeijaRenderPlugin::default())
    .add_plugin(WindowPlugin::default())
    .add_plugin(WinitPlugin::default())
    .add_startup_system(setup.system())
   .run()
}

fn setup(mut _commands: Commands,assets: Res<AssetServer>) {
    let mat:Handle<MaterialDesc> = assets.load("UISprite.material");
    
}