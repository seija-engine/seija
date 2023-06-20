use bevy_ecs::{prelude::*};
use glam::{Vec4, Vec2};
use seija_asset::{AssetServer, Assets};
use seija_core::{ CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_render::{camera::camera::{Orthographic, Camera, SortType},
};
use seija_transform::{Transform, events::{WorldEntityEx,EntityMutEx}};
use seija_ui::{update_ui_render, 
               components::{canvas::Canvas, ui_canvas::UICanvas, 
               sprite::Sprite, rect2d::Rect2D}, 
               types::Thickness };
use spritesheet::SpriteSheet;


fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],Some(Vec2::new(1024f32,768f32)));
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.run();
}



fn start(world: &mut World) {
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/default.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let bg_index = ui_sheet.get_index("lm-db").unwrap();
    let sprite_index = ui_sheet.get_index("Btn4On").unwrap();
    //CameraCanvas
    let mut ortho = Orthographic::default();
    ortho.far = 1000f32;
    ortho.near = -1000f32;
    let mut ui_camera = Camera::from_2d(ortho);
    ui_camera.sort_type = SortType::Z;
    let camera_id = world.spawn_empty().insert(Canvas::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();
    log::error!("canvas_id:{:?}",camera_id);

   
    let root_entity = world.spawn((Transform::default(),Rect2D::default(),Canvas::default())).set_parent(Some(camera_id)).id();

    let sprite = Sprite::sliced(bg_index, Some(h_sheet.clone()), Thickness::new1(30f32), Vec4::ONE);
    world.spawn((Transform::default(),Rect2D::default(),sprite)).set_parent(Some(root_entity));
}

