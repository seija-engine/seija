use bevy_ecs::{prelude::*, system::Command};
use glam::Vec4;
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_render::resource::load_image_info;
use seija_template::Template;
use seija_transform::{Transform, PushChildren};
use seija_ui::{components::{panel::Panel, rect2d::Rect2D, sprite::Sprite}, update_sprite_alloc_render, SpriteAllocator};
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

    let mut rect2d = Rect2D::default();
    rect2d.width = 1024f32;
    rect2d.height = 768f32;
    let panel_id = world.spawn().insert(Panel::default()).insert(Transform::default()).insert(rect2d).id();

    let mut rect2d = Rect2D::default();
    rect2d.width = 100f32;
    rect2d.height = 100f32;
    let sprite_id = world.spawn().insert(Sprite::simple(index, Vec4::ONE))
                                         .insert(rect2d)
                                         .insert(Transform::default()).id();
    PushChildren {
        parent:panel_id,
        children:SmallVec::from_slice(&[sprite_id])
    }.write(world);
    
    

    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut handle = server.load_sync::<Template>(world,"template/sky.xml", None).unwrap();
    handle.forget();
    {
        let templates = world.get_resource::<Assets<Template>>().unwrap();
        let template = templates.get(&handle.id).unwrap();
        Template::instance(template.clone(), world).unwrap();
    }
}

fn on_update() {

}