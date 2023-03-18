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
    update_sprite_alloc_render, SpriteAllocator, event::{UIEventSystem, EventNode, UIEvent, UIEventType},
};

#[derive( Resource)]
pub struct UIData {
    btn_entity:Entity,
    btn2_entity:Entity,
    normal_sprite:u32,
    down_sprite:u32,
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
   
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let mut sprite_alloc = world.get_resource_mut::<SpriteAllocator>().unwrap();
    let btn4down_index = sprite_alloc.alloc(load_image_info(server.full_path("ui/Btn4Down.png").unwrap()).unwrap()).unwrap();
    let btn4_index = sprite_alloc.alloc(load_image_info(server.full_path("ui/Btn4On.png").unwrap()).unwrap()).unwrap();

    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(UIEventSystem::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();

    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -1f32);
    let mut panel_event = EventNode::default();
    //panel_event.event_type = UIEventType::TOUCH_END | UIEventType::TOUCH_START;
    let panel_id = world.spawn((Panel::default(),panel_event,panel_t,rect2d)).set_parent(Some(canvas_id)).id();

    let rect2d = Rect2D::new(120f32,50f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(100f32, 0f32, 0f32);
    let mut event_node = EventNode::default();
    event_node.user_key = Some("test_button".into());
    event_node.event_type = UIEventType::TOUCH_END | UIEventType::TOUCH_START | UIEventType::CLICK;
    let btn_entity = world.spawn((event_node,Sprite::sliced(btn4_index, Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();
    
    let rect2d = Rect2D::new(120f32,50f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(-100f32, 0f32, 0f32);
    let mut event_node = EventNode::default();
    event_node.user_key = Some("test_button".into());
    event_node.event_type = UIEventType::MOUSE_ENTER | UIEventType::MOUSE_LEAVE;
    let btn2_entity = world.spawn((event_node,Sprite::sliced(btn4_index, Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();
 
    world.insert_resource(UIData {
        btn2_entity,
        btn_entity,
        normal_sprite:btn4_index,
        down_sprite:btn4down_index,
    });
}



fn on_update(mut render:EventReader<UIEvent>,mut commands:Commands,
             input: Res<Input>,time: Res<Time>,ui_data: ResMut<UIData>,
             mut sprites: Query<&mut Sprite>) {
    for event in render.iter() {
        if event.entity == ui_data.btn_entity {
            if event.event_type == UIEventType::TOUCH_START {
                let mut sprite = sprites.get_mut(ui_data.btn_entity).unwrap();
                sprite.sprite_index = Some(ui_data.down_sprite);
            } else if event.event_type == UIEventType::TOUCH_END {
                let mut sprite = sprites.get_mut(ui_data.btn_entity).unwrap();
                sprite.sprite_index = Some(ui_data.normal_sprite);
            } else if event.event_type == UIEventType::CLICK {
                seija_core::log::error!("click!");
            }
        }
        if event.entity == ui_data.btn2_entity {
            if event.event_type == UIEventType::MOUSE_ENTER {
                let mut sprite = sprites.get_mut(ui_data.btn2_entity).unwrap();
                sprite.sprite_index = Some(ui_data.down_sprite);
            } else if event.event_type == UIEventType::MOUSE_LEAVE {
                let mut sprite = sprites.get_mut(ui_data.btn2_entity).unwrap();
                sprite.sprite_index = Some(ui_data.normal_sprite);
            }
        }
    }
}
