mod lib;
mod sample_gltf;
mod cube_map;
mod light_test;
mod pbr;

use std::sync::Arc;

use light_test::LightTest;
use bevy_ecs::prelude::{IntoSystem};

use cube_map::CubeMapTest;
use sample_gltf::SampleGltf;
use seija_app::App;
use seija_asset::{AssetModule};
use seija_core::{CoreModule, CoreStage, StartupStage};

use seija_examples::{IExamples, pre_start};

use seija_render::{RenderModule, RenderConfig, GraphSetting,};
use seija_winit::WinitModule;
use seija_transform::{TransformModule};

const TEST_NAME:&'static str = "light_test";

fn main() {
     env_logger::Builder::new().filter_level(log::LevelFilter::Info).try_init().unwrap();
    
    
    let mut app = App::new();
    app.add_module(CoreModule);
    let mut win = WinitModule::default();
    //win.0.width = 480f32;
    //win.0.height = 320f32;
    app.add_module(win);
    app.add_module(TransformModule);
    app.add_module(AssetModule);
    let mut render_config = RenderConfig {
        config_path:".render".into(),
        setting:Arc::new(GraphSetting {
            msaa_samples:4
        }), 
    };
    render_config.set_config_path(".render");
    app.add_module(RenderModule(Arc::new(render_config)));
    
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, pre_start.system());
    
    match TEST_NAME {
        "sample_gltf" => {
            SampleGltf::run(&mut app);
        },
        "cube_map" => {
            CubeMapTest::run(&mut app)
        }
        
        "light_test" => {
            LightTest::run(&mut app);
        },
        "pbr_test" => {
            pbr::PbrTest::run(&mut app);
        }
        _ => { unimplemented!() }
    }; /**/

    
    app.run();
}