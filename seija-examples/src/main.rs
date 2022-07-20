mod lib;
use std::sync::Arc; 
use bevy_ecs::prelude::{IntoSystem, Commands, ResMut, Res};
use glam::{Vec3, Quat, Vec4};
use lib::load_material;
use seija_app::App;
use seija_asset::{AssetModule, Assets};
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_deferred::{create_deferred_plugin, DeferredRenderModule};
use seija_examples::{pre_start};
use seija_pbr::{create_pbr_plugin, lights::PBRLight};
use seija_render::{RenderModule, RenderConfig, GraphSetting, resource::{shape::{Cube, Sphere}, Mesh, Texture}, material::MaterialStorage, shadow::Shadow};
use seija_skeleton3d::{Skeleton3dModule, create_skeleton_plugin};
use seija_winit::WinitModule;
use seija_transform::{TransformModule, Transform};


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
        plugins:vec![create_pbr_plugin()],
        render_lib_paths:vec!["../crates/seija-pbr/res".into(),"../crates/seija-render/res".into()],
    };
    render_config.set_config_path(".render");
    app.add_module(RenderModule(Arc::new(render_config)));
    //app.add_module(DeferredRenderModule {mat_path:"res/materials/light_pass.mat.clj".into() });
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, pre_start);
    app.add_system2(CoreStage::Startup, StartupStage::Startup, start);
    app.start();

    

   
    app.run();
}

fn start(mut commands:Commands,
         mut meshs: ResMut<Assets<Mesh>>,
         mut textures: ResMut<Assets<Texture>>,
         materials: Res<MaterialStorage>) {
    {
        load_material("res/materials/pbrColor.mat.clj", &materials);
        {
            let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 12000f32);
            let mut t = Transform::default();
            let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32.to_radians(),  0f32.to_radians(), 0f32.to_radians());
            t.local.rotation = r;
            let mut l = commands.spawn();
            l.insert(light);
            l.insert(t);
        }
        
        let mesh =  Sphere::new(1f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("pbrColor", |mat| {
            mat.props.set_f32("metallic",  0.5f32, 0);
            mat.props.set_f32("roughness", 0.5f32, 0);
            mat.props.set_float4("color", Vec4::new(0f32, 0f32, 1f32, 1f32), 0)
        }).unwrap();

        let mut t = Transform::default();
        t.local.position = Vec3::new(0f32, 0f32, 3f32);
        let shadow = Shadow {cast_shadow:true,receive_shadow:true };
        commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
    };
   
}
