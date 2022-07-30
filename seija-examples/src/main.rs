mod lib;
use std::sync::Arc; 
use bevy_ecs::prelude::{Commands, ResMut, Res, Query, Entity};
use glam::{Vec3, Quat, Vec4};
use lib::load_material;
use seija_app::App;
use seija_asset::{AssetModule, Assets, Handle};
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_examples::{pre_start};
use seija_pbr::{create_pbr_plugin, lights::PBRLight};
use seija_render::{RenderModule, RenderConfig, GraphSetting, resource::{shape::{Cube, Sphere, Plane}, Mesh, Texture}, material::MaterialStorage, shadow::{Shadow, ShadowLight}};
use seija_skeleton3d::{Skeleton3dModule};
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
    app.add_system(CoreStage::Update, on_update);
    app.start();

    

   
    app.run();
}

fn start(mut commands:Commands,
         mut meshs: ResMut<Assets<Mesh>>,
         //mut textures: ResMut<Assets<Texture>>,
         materials: Res<MaterialStorage>) {
    {
        load_material("res/materials/pbrColor.mat.clj", &materials);
        load_material("res/materials/pbrColorShadow.mat.clj", &materials);
        //light
        {
            let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
            let mut t = Transform::default();
            let r = Quat::from_euler(glam::EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
            t.local.rotation = r;
            let mut l = commands.spawn();
            l.insert(light);
            l.insert(t);
            l.insert(ShadowLight::default());
        }
        //sphere
        {
            let mesh =  Sphere::new(0.5f32);
            let hmesh = meshs.add(mesh.into());
            let hmat = materials.create_material_with("pbrColorShadow", |mat| {
                mat.props.set_f32("metallic",  0.3f32, 0);
                mat.props.set_f32("roughness", 0.7f32, 0);
                //mat.props.set_float4("color", Vec4::new(0f32, 0f32, 1f32, 1f32), 0)
            }).unwrap();
    
            let mut t = Transform::default();
            t.local.position = Vec3::new(-2f32, 0.5f32, 0f32);
            t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, -40f32.to_radians(), 0f32);
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        
        //Cube
        {
            let mesh =  Cube::new(1f32);
            let hmesh = meshs.add(mesh.into());
            let hmat = materials.create_material_with("pbrColorShadow", |mat| {
                mat.props.set_f32("metallic",  0.5f32, 0);
                mat.props.set_f32("roughness", 0.5f32, 0);
                //mat.props.set_float4("color", Vec4::new(0f32, 0f32, 1f32, 1f32), 0)
            }).unwrap();
    
            let mut t = Transform::default();
            t.local.scale = Vec3::new(1f32, 1f32, 1f32);
            t.local.position = Vec3::new(1f32, 0.5f32, 0f32);
            //t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, -40f32.to_radians(), 0f32);
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        //plane
        {
            
            let mesh =  Plane::new(100f32,10).into();
            let hmesh = meshs.add(mesh);
            let hmat = materials.create_material_with("pbrColorShadow", |mat| {
                mat.props.set_f32("metallic",  0.5f32, 0);
                mat.props.set_f32("roughness", 0.5f32, 0);
                mat.props.set_float4("color", Vec4::new(1f32, 1f32, 1f32, 1f32), 0)
            }).unwrap();
            let t = Transform::default();
            
           
            //let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32.to_radians(),  0f32.to_radians(), 0f32.to_radians());
            //t.local.rotation = r;
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        
    };
   
}


fn on_update(mut query:Query<(Entity,&Handle<Mesh>,&mut Transform)>) {
   
    let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 36000) as f32;
    let r = v * 0.01f32 * 0.0174533f32;
    
    for (_,_,mut t) in query.iter_mut() {
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, r, 0f32);
        
        t.local.rotation = r;
    }
}