use std::borrow::BorrowMut;

use bevy_app::{App, AppBuilder, CoreStage, EventReader};
use bevy_window::{WindowPlugin};
use bevy_winit::WinitPlugin;
use bevy_input::InputPlugin;
use bevy_wgpu::WgpuPlugin;
use bevy_render::RenderPlugin;
use bevy_asset::{AssetEvent, AssetPlugin, AssetServer, Assets, Handle};
use bevy_core::CorePlugin;
use bevy_log::LogPlugin;
use bevy_ecs::prelude::*;
use bevy_transform::TransformPlugin;

use seija_render::{MaterialDesc,SeijaRenderPlugin,MaterialDescTable,Material};

fn main() {
    App::build()
    .add_plugin(LogPlugin::default())
    .add_plugin(TransformPlugin::default())
    .add_plugin(CorePlugin::default())
    .add_plugin(AssetPlugin::default())
    .add_plugin(InputPlugin::default())
    .add_plugin(RenderPlugin::default())
    .add_plugin(WgpuPlugin::default())
    .add_plugin(SeijaRenderPlugin::default())
    .add_plugin(WindowPlugin::default())
    .add_plugin(WinitPlugin::default())
    .add_startup_system(setup.system())
    .add_system_to_stage(CoreStage::Last,update.system())
   .run()
}



fn setup(mut _commands: Commands,assets: Res<AssetServer>,mat_descs:Res<Assets<MaterialDesc>>,mut table:ResMut<MaterialDescTable>) {
    let mat_path = "UISprite.material";
    let mat = table.load_get(mat_path, &assets);
    
}

fn update(mut mesh_events: EventReader<AssetEvent<MaterialDesc>>
    ,assets: Res<AssetServer>
    ,mat_descs:ResMut<Assets<MaterialDesc>>
    ,mut table:ResMut<MaterialDescTable>
    ,mut materials:ResMut<Assets<Material>>) {
   
    for event in mesh_events.iter() {
        match event {
            AssetEvent::Created {handle } => {            
               let mat =  table.load_get("UISprite.material", &assets);    
               let refv = mat_descs.get(mat).unwrap();
               let material = refv.create();
               let mat_handle = materials.add(material);
               dbg!(mat_handle);
            },
            AssetEvent::Modified {handle } => {
                let mat = mat_descs.get(handle);
               
            },
            _ => ()
        }
    }
}