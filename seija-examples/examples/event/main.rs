use bevy_ecs::{prelude::*, system::Command, world};
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app, load_material};
use seija_input::Input;
use seija_render::{
    camera::camera::{Camera, Orthographic},
    material::Material,
    resource::{load_image_info, Mesh},
};
use seija_template::Template;
use seija_transform::{hierarchy::Parent, BuildChildren, IEntityChildren, PushChildren, Transform};
use seija_ui::{
    components::{panel::Panel, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
    update_sprite_alloc_render, SpriteAllocator, event::{UIEventSystem, EventNode},
};

#[derive(Default, Resource)]
pub struct UIData {
   
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
    let mut ui_data = UIData::default();
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let mut sprite_alloc = world.get_resource_mut::<SpriteAllocator>().unwrap();
    let btn4down_index = sprite_alloc.alloc(load_image_info(server.full_path("ui/Btn4Down.png").unwrap()).unwrap()).unwrap();
    let btn4_index = sprite_alloc.alloc(load_image_info(server.full_path("ui/Btn4On.png").unwrap()).unwrap()).unwrap();

    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(UIEventSystem::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();

    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -1f32);
    let panel_id = world.spawn((Panel::default(),panel_t,rect2d)).set_parent(Some(canvas_id)).id();

    let rect2d = Rect2D::new(120f32,50f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(100f32, 0f32, 0f32);
    let mut event_node = EventNode::default();
    world.spawn((event_node,Sprite::sliced(btn4_index, Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id));
    
   
 
    world.insert_resource(ui_data);
}



fn on_update(mut commands: Commands,input: Res<Input>,time: Res<Time>,ui_data: ResMut<UIData>,mut sprites: Query<&mut Sprite>) {
   
    
}
