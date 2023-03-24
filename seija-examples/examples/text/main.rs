use bevy_ecs::world::World;
use glam::{Vec4, Vec3};
use seija_asset::AssetServer;
use seija_core::{CoreStage, StartupStage};
use seija_examples::init_core_app;
use seija_render::{camera::camera::{Orthographic, Camera}, resource::load_image_info};
use seija_transform::{Transform, IEntityChildren};
use seija_ui::{update_ui_render, text::{Font, Text}, event::UIEventSystem, components::{ui_canvas::UICanvas, rect2d::Rect2D, sprite::Sprite, canvas::Canvas}, types::Thickness};

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);

    app.run();
}

fn start(world: &mut World) {
    /*
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut sprite_alloc = world.get_resource_mut::<SpriteAllocator>().unwrap();
    let btn4_index = sprite_alloc.alloc(load_image_info(server.full_path("ui/Btn4On.png").unwrap()).unwrap()).unwrap();

    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(UIEventSystem::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();
    let h_font = server.load_sync::<Font>(world,"ui/WenQuanYiMicroHei.ttf", None).unwrap();
    
    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -1f32);
    let panel_id = world.spawn((Panel::default(),panel_t,rect2d)).set_parent(Some(canvas_id)).id();

    let rect2d = Rect2D::new(120f32,50f32);
    let mut t = Transform::default();
    world.spawn((Sprite::sliced(btn4_index, Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id));

    let rect2d = Rect2D::new(120f32,50f32);
    let t = Transform::default();
    let mut text = Text::new(h_font.clone());
    text.text = "Fuck World".to_string();
    world.spawn((text,rect2d,t)).set_parent(Some(panel_id));

    let rect2d = Rect2D::new(120f32,50f32);
    let t = Transform::default();
    let mut text = Text::new(h_font);
    text.text = "天生万物".to_string();
    world.spawn((text,rect2d,t)).set_parent(Some(panel_id));*/
}



