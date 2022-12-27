use bevy_ecs::{prelude::*, system::Command};
use glam::{Vec4, Vec3};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app, load_material};
use seija_render::{resource::{load_image_info, Mesh}, camera::camera::{Camera, Orthographic}, material::Material};
use seija_template::Template;
use seija_transform::{Transform, PushChildren, hierarchy::Parent};
use seija_ui::{components::{panel::Panel, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas}, update_sprite_alloc_render, SpriteAllocator};
use smallvec::SmallVec;


fn main() {
    let mut app = init_core_app("FRPRender.clj",vec![update_sprite_alloc_render]);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, on_update);
   
    app.run();
}

fn start(world:&mut World) {
    let server:AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let mut sprite_alloc = world.get_resource_mut::<SpriteAllocator>().unwrap();
    let btn_path = server.full_path("ui/Btn_V03.png").unwrap();
    let btn2_path = server.full_path("ui/Btn_V04.png").unwrap();
    let image_info = load_image_info(btn_path).unwrap();
    let image_info2 = load_image_info(btn2_path).unwrap();
    let index = sprite_alloc.alloc(image_info).unwrap();
    let index2 = sprite_alloc.alloc(image_info2).unwrap();

    let ui_camera = Camera::from_2d(Orthographic::default());
    
    let canvas_id = world.spawn().insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();

    let mut rect2d = Rect2D::default();
    rect2d.width = 1024f32;
    rect2d.height = 768f32;
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -60f32);
    let panel_id = world.spawn().insert(Panel::default()).insert(panel_t).insert(Parent(canvas_id)).insert(rect2d).id();

    let mut rect2d = Rect2D::default();
    rect2d.width = 366f32;
    rect2d.height = 67f32;
    let sprite_id = world.spawn().insert(Sprite::simple(index, Vec4::ONE))
                                         .insert(rect2d)
                                         .insert(Transform::default())
                                         .insert(Parent(panel_id)).id();
   
   
}

fn on_update() {

}