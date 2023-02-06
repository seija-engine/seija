use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_input::Input;

use seija_render::{
    camera::camera::{Camera, Orthographic},
    resource::{load_image_info},
};
use seija_transform::{IEntityChildren,Transform};
use seija_ui::{
    components::{panel::Panel, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
    update_sprite_alloc_render, SpriteAllocator, layout::{types::LayoutElement, comps::Orientation},
};

#[derive(Default, Resource)]
pub struct UIData {
   
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render]);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn load_sprite(path:&str,server:&AssetServer,sprite_alloc:&mut SpriteAllocator) -> u32 {
    let full_path = server.full_path(path).unwrap();
    let image_info = load_image_info(full_path).unwrap();
    let index = sprite_alloc.alloc(image_info).unwrap();
    index
}

fn start(world: &mut World) {
    let ui_data = UIData::default();
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let mut sprite_alloc = world.get_resource_mut::<SpriteAllocator>().unwrap();
    let bg_index = load_sprite("ui/lm-db.png", &server, &mut sprite_alloc);
    //Canvas
    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();
    //Background
    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -1f32);
    let panel_id = world.spawn((Panel::default(),panel_t,rect2d)).set_parent(Some(canvas_id)).id();
    
    let rect2d = Rect2D::new(400f32,300f32);
    let t = Transform::default();
    let stack_layout = LayoutElement::create_stack(10f32, Orientation::Vertical);
    world.spawn((Sprite::sliced(bg_index, Thickness::new1(35f32), Vec4::ONE),rect2d,t,stack_layout)).set_parent(Some(panel_id));
    
   
      
    world.insert_resource(ui_data);
}


fn on_update(mut commands: Commands,input: Res<Input>,time: Res<Time>,ui_data: ResMut<UIData>,mut sprites: Query<&mut Sprite>) {
    
   
}
