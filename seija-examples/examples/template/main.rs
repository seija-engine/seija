use glam::{Vec3, Quat};
use seija_asset::{AssetServer, Assets, AssetRequest, Handle};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app, update_camera_trans_system, load_material};
use bevy_ecs::{prelude::*};
use seija_pbr::lights::{PBRLight, PBRGlobalAmbient};
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
    //app.add_system(CoreStage::PostUpdate, on_post_update.exclusive_system().at_end());
    app.run();
}

fn start(world:&mut World) {
    load_material("materials/pbrColor.mat.clj", world);
    load_material("materials/baseColor.mat.clj", world);
    world.insert_resource(PBRGlobalAmbient::new(Vec3::new(0.2f32, 0.2f32, 0.2f32)));
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

    

    

    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let req = asset_server.load_async::<Template>("template/first.xml", None).unwrap();
   
    let local_data = LocalData { req,hid:None };
    //log::info!("start instance template");
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
      Template::instance(template.clone(), world).unwrap();
   }
}
/*

fn on_post_update(world:&mut World) {
    let mut add_meshs = world.query_filtered::<(Entity,&Transform,&Handle<Mesh>,Option<&EInfo>),(Added<Handle<Mesh>>,Added<Handle<Material>>)>();
    let mut aabbs:Vec<AABB3> = vec![];
    let meshs =  world.get_resource::<Assets<Mesh>>().unwrap();
    for (e,t,mesh,einfo) in add_meshs.iter(world) {
        if let Some(info) = einfo {
            if info.name.as_ref().map(|v| v.as_str()) == Some("Skybox") {
                continue;
            }
        }
        if let Some(aabb) = meshs.get(&mesh.id).and_then(|v| v.aabb.as_ref()) {
            
            let new_aabb = aabb.transform(&t.global().matrix());
            aabbs.push(new_aabb);
        }
    }

    for aabb in aabbs.iter() {
        {
            let mesh = create_aabb_mesh(aabb);
            let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            let hmesh = meshs.add(mesh);
            let t = Transform::default();
           
            
            let material = Material::from_world(world, "materials/baseColor.mat.clj").unwrap();
            let mut materials = world.get_resource_mut::<Assets<Material>>().unwrap();
            let hmat = materials.add(material);
            let mut e_mut =  world.spawn();
            e_mut.insert(hmesh).insert(hmat).insert(t);
        };
    }
}*/