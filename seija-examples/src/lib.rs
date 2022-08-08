use std::sync::Arc;

use bevy_ecs::{prelude::{Commands, Entity}, system::{EntityCommands, Res, Local, Query}};
use glam::{Quat, Vec3, Vec4, Vec2};
use lite_clojure_eval::EvalRT;
use seija_app::App;
use seija_asset::{Assets, Handle, AssetModule};
use seija_core::{window::AppWindow, info::EInfo, CoreModule, time::Time};
use seija_input::{InputModule, Input, event::MouseButton};
use seija_pbr::{PBRCameraInfo, create_pbr_plugin};
use seija_render::{camera::{camera::Perspective,camera::Camera}, 
                   material::{MaterialStorage, read_material_def}, resource::{Mesh, Texture, TextureDescInfo}
                  ,RenderConfig, GraphSetting, RenderModule};
use seija_transform::{Transform, TransformModule};
use seija_winit::WinitModule;


pub fn init_core_app(render_file:&str) -> App {
    env_logger::Builder::new().filter_level(log::LevelFilter::Info).try_init().unwrap();
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(InputModule);
    let win = WinitModule::default();
    //win.0.width = 480f32;
    //win.0.height = 320f32;
    app.add_module(win);
    app.add_module(TransformModule);
    app.add_module(AssetModule);

    let render_config = RenderConfig {
        config_path:".render/shaders".into(),
        script_path:format!(".render/{}",render_file).into(),
        setting:Arc::new(GraphSetting::default() ),
        plugins:vec![create_pbr_plugin()],
        render_lib_paths:vec!["../crates/seija-pbr/res".into(),"../crates/seija-render/res".into(),"examples".into()],
    };
    app.add_module(RenderModule(Arc::new(render_config)));

    app.start();
    app
}

pub fn add_pbr_camera(commands:&mut Commands,window:&AppWindow,pos:Vec3,r:Quat,f:Option<fn(&mut EntityCommands)>) -> Entity {
    let mut camera_entity = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    t.local.rotation = r;
    camera_entity.insert(t);

    let mut per = Perspective::default();
    per.far = 50f32;
    per.aspect_ratio = window.width() as f32 / window.height() as f32;
    let camera = Camera::from_3d(per);
    camera_entity.insert(camera);

    let mut info = EInfo::default();
    info.name = Some("Camera3D".into());
    camera_entity.insert(info);

    let pbr_camera = PBRCameraInfo::default();
    camera_entity.insert(pbr_camera);
    if let Some(f) = f {
        f(&mut camera_entity);
    }
    camera_entity.id()
}

pub fn load_material(path:&str,mats:&MaterialStorage) {
    println!("load_material:{}",path);
    let code_string = std::fs::read_to_string(path).unwrap();
    let mut vm = EvalRT::new();
    let mat_def = read_material_def(&mut vm, &code_string,false).unwrap();
    mats.add_def(mat_def);
}


pub fn load_texture(textures:&mut Assets<Texture>,path:&str) -> Handle<Texture> {
    let texture = Texture::from_image_bytes(&std::fs::read(path).unwrap(),TextureDescInfo::default()).unwrap();
    textures.add(texture)
}

pub fn add_render_mesh(
                       commands:&mut Commands,
                       mesh:Handle<Mesh>,
                       texture:Handle<Texture>,
                       mat_name:&str,
                       pos:Vec3,
                       mats:&MaterialStorage) -> Entity {
   
    let mut elem = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    t.local.rotation = Quat::from_rotation_y(45f32);
    elem.insert(t);
    
    elem.insert(mesh);

    let material = mats.create_material(mat_name).unwrap();
   
    
    let mut mats = mats.mateials.write();
    let mat = mats.get_mut(&material.id).unwrap();
    elem.insert(material);
    mat.texture_props.set("mainTexture", texture);
    mat.props.set_float4("color", Vec4::new(1f32, 1f32, 1f32, 1f32), 0);
    
    
    elem.id()
}



enum ControllerState {
    None,
    HOrVMove,
    Rotation
}

pub struct CameraTranController {
    move_speed:f32,
    hv_move_speed:f32,
    state:ControllerState,
    last_position:Vec2
}

impl Default for CameraTranController {
    fn default() -> Self {
        CameraTranController { 
            move_speed:10f32,
            hv_move_speed:5f32,
            state:ControllerState::None,
            last_position:Vec2::ZERO
         }
    }
}

pub fn update_camera_trans_system(time:Res<Time>,
                              input:Res<Input>,
                              mut controller:Local<CameraTranController>,
                              mut cameras:Query<(Entity,&Camera,&mut Transform)>) {
    
    if let Some((_entity,_camera,mut trans)) = cameras.iter_mut().next() {
        match controller.state {
            ControllerState::None => {
                if let Some(delta) = input.get_mouse_wheel() {
                    let foward = trans.global().rotation * -Vec3::Z;
                    //后滑
                    if delta.y > 0f32 {
                         trans.local.position += foward * time.delta_seconds() * controller.move_speed;
                    } else {
                         trans.local.position -= foward * time.delta_seconds() * controller.move_speed;
                    }
                }
                if input.get_mouse_down(MouseButton::Right) {
                    controller.state = ControllerState::Rotation
                }
                if input.get_mouse_down(MouseButton::Middle) {
                    controller.state = ControllerState::HOrVMove;
                    controller.last_position = input.mouse_position;
                }
            },
            ControllerState::Rotation => {
                let delta = input.mouse_position - controller.last_position;
                if delta.x != 0f32 {
                    let x_dir = if delta.x > 0f32 { -1f32 } else { 1f32 }; 
                    let (euler_y,euler_x,euler_z) = trans.local.rotation.to_euler(glam::EulerRot::YXZ);
                    trans.local.rotation = Quat::from_euler(glam::EulerRot::YXZ,euler_y + (x_dir * 80f32 * time.delta_seconds()).to_radians(), euler_x, euler_z);
                }
                if delta.y != 0f32 {
                    let y_dir = if delta.y > 0f32 { -1f32 } else { 1f32 }; 
                    let (euler_y,euler_x,euler_z) = trans.local.rotation.to_euler(glam::EulerRot::YXZ);
                    trans.local.rotation = Quat::from_euler(glam::EulerRot::YXZ,euler_y, euler_x + (y_dir * 80f32 * time.delta_seconds()).to_radians(), euler_z);
                }
                if input.get_mouse_up(MouseButton::Right) {
                    controller.state = ControllerState::None
                }
                controller.last_position = input.mouse_position;
            },
            ControllerState::HOrVMove => {
                let delta = input.mouse_position - controller.last_position;
                if delta.x != 0f32 {
                    let right = trans.global().rotation * Vec3::X;
                    trans.local.position += -delta.x * controller.hv_move_speed * right * time.delta_seconds();
                }
                if delta.y != 0f32 {
                    let top = trans.global().rotation * Vec3::Y;
                    trans.local.position += delta.y * controller.hv_move_speed * top * time.delta_seconds();
                }
                if input.get_mouse_up(MouseButton::Middle) {
                    controller.state = ControllerState::None
                }
                controller.last_position = input.mouse_position;
            }
        }
    }
}


