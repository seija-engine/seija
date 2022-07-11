mod lib;
use std::sync::Arc; 
use bevy_ecs::prelude::{IntoSystem};
use seija_app::App;
use seija_asset::{AssetModule};
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_deferred::{create_deferred_plugin, DeferredRenderModule};
use seija_examples::{pre_start};
use seija_pbr::create_pbr_plugin;
use seija_render::{RenderModule, RenderConfig, GraphSetting};
use seija_skeleton3d::{Skeleton3dModule, create_skeleton_plugin};
use seija_winit::WinitModule;
use seija_transform::{TransformModule};


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
    app.add_module(Skeleton3dModule);
    let mut render_config = RenderConfig {
        config_path:".render".into(),
        setting:Arc::new(GraphSetting {
            msaa_samples:1
        }),
        plugins:vec![create_pbr_plugin(),
                     create_skeleton_plugin(),
                     create_deferred_plugin()],
        render_lib_paths:vec!["../crates/seija-pbr/res".into(),"../crates/seija-render/res".into()],
    };
    render_config.set_config_path(".render");
    app.add_module(RenderModule(Arc::new(render_config)));
    app.add_module(DeferredRenderModule {mat_path:"res/materials/light_pass.mat.clj".into() });
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, pre_start);
    app.start();

    

   
    app.run();
}