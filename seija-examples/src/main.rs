mod lib;
mod sample_gltf;
mod cube_map;
mod async_asset;

use async_asset::AsyncAsset;
use bevy_ecs::prelude::{IntoSystem};

use cube_map::CubeMapTest;
use sample_gltf::SampleGltf;
use seija_app::App;
use seija_asset::{AssetModule};
use seija_core::{CoreModule, CoreStage, StartupStage};

use seija_examples::{IExamples, pre_start};

use seija_render::{RenderModule};
use seija_winit::WinitModule;
use seija_transform::{TransformModule};

const TEST_NAME:&'static str = "cube_map";

fn main() {
    env_logger::init();
   

    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(WinitModule::default());
    app.add_module(TransformModule);
    app.add_module(AssetModule);
    app.add_module(RenderModule);
    
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, pre_start.system());
    match TEST_NAME {
        "sample_gltf" => {
            SampleGltf::run(&mut app);
        },
        "cube_map" => {
            CubeMapTest::run(&mut app)
        }
        "async_asset" => {
            AsyncAsset::run(&mut app);
        }
        _ => { unimplemented!() }
    };

    
    app.run();
}