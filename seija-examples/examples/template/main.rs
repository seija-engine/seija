use glam::{Vec3, Quat};
use seija_asset::{AssetServer, Assets, AssetRequest, Handle};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app, update_camera_trans_system, load_material};
use bevy_ecs::{prelude::*};
use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh, shape::Sphere}, material::Material};
use seija_template::{Template};
use seija_transform::Transform;


pub struct LocalData {
    req:AssetRequest,
    hid:Option<Handle<Template>>
}

pub fn main() {
    let mut app = init_core_app("model_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::Startup, start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.add_system(CoreStage::Update, async_system.exclusive_system());
    app.run();
}

fn start(world:&mut World) {
    load_material("materials/pbrColor.mat.clj", world);
    
    
     //light
     {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , -30f32.to_radians(),  50f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = world.spawn();
        l.insert(light);
        l.insert(t);
    }

    //sphere
    {
        let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh =  Sphere::new(0.5f32);
        let hmesh = meshs.add(mesh.into());

        let mut material = Material::from_world(world, "materials/pbrColor.mat.clj").unwrap();
        let mut materials = world.get_resource_mut::<Assets<Material>>().unwrap();
        material.props.set_f32("metallic",  0.3f32, 0);
        material.props.set_f32("roughness", 0.7f32, 0);
        let hmat = materials.add(material);

        let mut t = Transform::default();
        t.local.position = Vec3::new(0f32, 0f32, -1f32);
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, 0f32.to_radians(), 0f32);
        //world.spawn().insert(hmesh).insert(hmat).insert(t);
    };

    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let req = asset_server.load_async::<Template>("template/first.xml", None).unwrap();
   
    let local_data = LocalData { req,hid:None };
    log::info!("start instance template");
    world.insert_resource(local_data);
}

fn async_system(world:&mut World) {
   let mut local_data = world.get_resource_mut::<LocalData>().unwrap();
   if local_data.req.is_finish() && local_data.hid.is_none() {
      
      let h = local_data.req.make_handle().typed();
      let hid = h.id;
      local_data.hid = Some(h);
      log::error!("load finish");
      
      let templates = world.get_resource::<Assets<Template>>().unwrap();
      let template = templates.get(&hid).unwrap();
      Template::instance(template.entity.clone(), world).unwrap();
   }
}