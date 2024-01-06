use bevy_ecs::prelude::*;
use glam::{Vec4, Vec2};
use seija_asset::{AssetServer, Assets};
use seija_core::{ CoreStage, StartupStage};
use seija_examples::init_core_app;
use seija_render::{camera::camera::{Camera, Orthographic, SortType}, resource::Texture};
use seija_2d::components::{image::Image, sprite::Sprite2D};
use seija_transform::{Transform, events::EntityMutEx};
use seija_ui::Rect2D;
use seija_2d::components::screen_scaler::{ScreenScaler,ScalerMode,ScreenSizeMatchWHInfo};
use spritesheet::SpriteSheet;
fn main() {
    let mut app = init_core_app("FRPRender.clj",vec![],None);
    app.add_system2(CoreStage::Startup,StartupStage::Startup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
    let mut camera = Camera::from_2d(Orthographic::default());
    camera.sort_type = SortType::Z;
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let h_texture = server.load_sync::<Texture>(world, "texture/backyard_evening.png", None).unwrap();

    let wh_info = ScreenSizeMatchWHInfo { design_size:Vec2::new(1920f32,1080f32),wh_rate:1f32 };
    
    let camera_t = Transform::default();
    let camera_id = world.spawn_empty().insert(camera_t).insert(camera).id();
    let scaler = ScreenScaler {camera_entity:Some(camera_id),mode:ScalerMode::ScreenSizeMatchWH(wh_info) };
    let mut t = Transform::default();
    t.local.position.z = -1f32;
    let canvas_id = world.spawn((t,scaler)).id();
    log::error!("camera_id id:{:?}",camera_id);
   
    let img = Image::new(h_texture, Vec4::ONE);
    let mut rect2d = Rect2D::default();
    rect2d.width = 1920f32;
    rect2d.height = 1080f32;
    let image_id = world.spawn((img,rect2d,Transform::default())).set_parent(Some(canvas_id)).id();
    log::error!("image id:{:?}",image_id);

    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/default.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let btn3on_index = ui_sheet.get_index("dl").unwrap();
    let sprite_id = {
        let mut t = Transform::default();
        t.local.position.z = 0.5f32;
        let sprite2d = Sprite2D::simple(Some(h_sheet), btn3on_index, Vec4::ONE);
        world.spawn((sprite2d,t,Rect2D::new(100f32, 100f32))).set_parent(Some(canvas_id)).id()
    };
    log::error!("sprite id:{:?}",sprite_id);
}


fn on_update() {
    
}
