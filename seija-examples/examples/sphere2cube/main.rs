use glam::Vec3;
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{CoreStage, StartupStage};
use seija_app::ecs::prelude::*;
use seija_examples::{init_core_app, update_camera_trans_system, load_material};

use seija_render::{material::Material, resource::Mesh};
use seija_template::Template;
use seija_transform::Transform;

fn main() {
    let mut app = init_core_app("FRPRender.clj");
    app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    
    app.run();
}

fn on_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut handle = server.load_sync::<Template>(world,"template/sphere_sky.xml", None).unwrap();
    handle.forget();
    let templates = world.get_resource::<Assets<Template>>().unwrap();
    let template = templates.get(&handle.id).unwrap();
    Template::instance(template.clone(), world).unwrap();
    load_material("materials/pbrColor.mat.clj", world);

    let h_mesh:Handle<Mesh> = server.get_asset("mesh:sphere").unwrap().make_handle().typed();

    for m in 1..6 {
        for r in 1..6 {
            {
                
                let mut mat = Material::from_world(world, "materials/pbrColor.mat.clj").unwrap();
                mat.props.set_f32("metallic",  1f32 - 0.2f32 * m as f32, 0);
                mat.props.set_f32("roughness", 1f32 - 0.2f32 * r as f32, 0);
                let mut mats = world.get_resource_mut::<Assets<Material>>().unwrap();
                let hmat = mats.add(mat);
        
                let mut t = Transform::default();
                t.local.position = Vec3::new(m as f32 * 1.2f32, r as f32 * 1.2f32, 0f32);
                world.spawn().insert(h_mesh.clone()).insert(hmat).insert(t);
            };
        }
    }
}