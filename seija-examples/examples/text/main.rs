use bevy_ecs::world::World;
use seija_asset::AssetServer;
use seija_core::{CoreStage, StartupStage};
use seija_examples::init_core_app;
use seija_render::camera::camera::{Orthographic, Camera};
use seija_transform::{Transform, IEntityChildren};
use seija_ui::{update_sprite_alloc_render, text::{Font, Text}, event::UIEventSystem, components::{ui_canvas::UICanvas, rect2d::Rect2D}};

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);

    app.run();
}

fn start(world: &mut World) {
    
    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(UIEventSystem::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();

    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let h_font = server.load_sync::<Font>(world,"ui/WenQuanYiMicroHei.ttf", None).unwrap();
    
    let rect2d = Rect2D::new(120f32,50f32);
    let t = Transform::default();
    let mut text = Text::new(h_font);
    text.text = "Fuck World".to_string();
    world.spawn((text,rect2d,t)).set_parent(Some(canvas_id));
}



