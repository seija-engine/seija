use std::borrow::BorrowMut;

use bevy_app::{App, AppBuilder, CoreStage, EventReader};
use bevy_window::{WindowPlugin};
use bevy_winit::WinitPlugin;
use bevy_input::InputPlugin;
use bevy_wgpu::WgpuPlugin;
use bevy_render::{RenderPlugin, mesh::{Mesh, shape}, prelude::{OrthographicCameraBundle, Texture}};
use bevy_asset::{Asset, AssetEvent, AssetPlugin, AssetServer, Assets, Handle};
use bevy_core::CorePlugin;
use bevy_log::LogPlugin;
use bevy_ecs::prelude::*;
use bevy_transform::TransformPlugin;
use glam::Vec2;
use seija_2d::{Seija2DPlugin,elems::image::Image};

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
    .add_plugin(TransformPlugin::default())
    .add_plugin(SeijaRenderPlugin::default())
    .add_plugin(Seija2DPlugin::default())
    .add_plugin(WindowPlugin::default())
    .add_plugin(WinitPlugin::default())
    .add_startup_system(setup.system())
    .add_system_to_stage(CoreStage::Last,update.system())
   .run()
}



fn setup(mut commands: Commands,assets: Res<AssetServer>,mat_descs:Res<Assets<MaterialDesc>>,mut table:ResMut<MaterialDescTable>) {
    let mat_path = "UISprite.material";
    table.load_get(mat_path, &assets);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn update(mut commands: Commands
    ,mut meshs:ResMut<Assets<Mesh>>
    ,mut mesh_events: EventReader<AssetEvent<MaterialDesc>>
    ,assets: Res<AssetServer>
    ,mat_descs:ResMut<Assets<MaterialDesc>>
    ,mut table:ResMut<MaterialDescTable>
    ,mut materials:ResMut<Assets<Material>>) {
   
    for event in mesh_events.iter() {
        match event {
            AssetEvent::Created {handle } => {
                let mat_desc = mat_descs.get(handle).unwrap();
                let mut mat = mat_desc.create();
                let tex:Handle<Texture> = assets.load("icon.png");
                mat.set_textute("Image", tex);
                let vec2 = glam::Vec2::new(3f32, 3f32);
                let quad_mesh:Mesh = shape::Quad::new(vec2).into();
                let mesh_handle = meshs.add(quad_mesh);
                let mut img = Image::new(mat,mesh_handle);
                
                
                

                commands.spawn().insert_bundle(img);      
            },
            _ => ()
        }
    }
}