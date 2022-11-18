use bevy_ecs::system::{Commands, ResMut, CommandQueue, EntityCommands};
use glam::{Vec3, Quat};
use seija_asset::{Assets, AssetServer,  Handle, AssetRequest};
use seija_core::{CoreStage, StartupStage, window::AppWindow, time::Time};
use seija_examples::{init_core_app, add_pbr_camera, load_material, update_camera_trans_system};
use seija_gltf::{create_gltf, asset::GltfAsset};
use seija_pbr::lights::PBRLight;
use bevy_ecs::prelude::*;
use seija_render::{material::{MaterialDefineAsset, Material}, dsl_frp::PostEffectStack, camera::camera::Camera};
use seija_transform::Transform;

#[derive(Default)]
struct GameData {
    shiba_asset:Option<Handle<GltfAsset>>,
    track:Option<AssetRequest>
}

pub fn main() {
    let mut app = init_core_app("FRPRender.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.add_system(CoreStage::Update, on_update);
    app.init_resource::<GameData>();
    app.run();
}

fn start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    load_material("materials/baseTexture.mat.clj", world);
    let htonemap = server.load_sync::<Material>(world, "mats/tonemap.json", None).unwrap();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    let window = world.get_resource::<AppWindow>().unwrap();
    add_pbr_camera(&mut commands,&window,Vec3::new(0f32, -0.2f32, 2f32),Quat::IDENTITY,|cmds:&mut EntityCommands| {
        let mut effect_stack = PostEffectStack::default();
        effect_stack.add_item(htonemap, 1000);
        //effect_stack.add_item(htonemap, 1000);
        //cmds.insert(effect_stack);
    },None,None);
    queue.apply(world);
    
    let track = server.load_async::<GltfAsset>("gltf/shiba/scene.gltf", None).unwrap();
    let mut data = world.get_resource_mut::<GameData>().unwrap();
    data.track = Some(track);
    
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = world.spawn();
        l.insert(light);
        l.insert(t);
    };
}

fn on_update(mut commands:Commands,mut mats:ResMut<Assets<Material>>,time:Res<Time>,
    server:Res<AssetServer>,mut data:ResMut<GameData>,gltfs:Res<Assets<GltfAsset>>,
    defs:Res<Assets<MaterialDefineAsset>>,mut cameras:Query<(Entity,&mut Camera)>) {
    let is_finish = data.track.as_ref().map(|v| v.is_finish()).unwrap_or(false);
    if is_finish {
       let h_def = server.get_asset("materials/baseTexture.mat.clj").unwrap().make_handle().typed::<MaterialDefineAsset>();
       let define = defs.get(&h_def.id).unwrap().define.clone();
       data.shiba_asset = data.track.as_ref().map(|v| v.make_handle().typed());
       data.track = None;
       if let Some(asset) = gltfs.get(&data.shiba_asset.as_ref().unwrap().id) {
        let _ = create_gltf(&asset, &mut commands,  |gltf_mat| {
            let mut mat = Material::from_def(define.clone(), &server).unwrap();
            mat.texture_props.set("mainTexture", gltf_mat.base_color_texture.as_ref().unwrap().clone());
            mat.props.set_float4("color", gltf_mat.base_color_factor, 0);
            mats.add(mat).into()
        });
       }
    }

    if time.frame() == 8 * 30 {
        for (_,mut camera) in cameras.iter_mut() {
            //camera.is_hdr = false;
        }
    }
}