use bevy_ecs::{prelude::*, system::Command, world};
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app, load_material};
use seija_input::Input;
use seija_render::{
    camera::camera::{Camera, Orthographic},
    resource::{load_image_info, Mesh},
};
use seija_template::Template;
use seija_transform::{hierarchy::Parent, events::EntityMutEx, Transform};
use seija_ui::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
    update_ui_render, event::{UIEventSystem, EventNode, UIEvent, UIEventType},
};
use spritesheet::SpriteSheet;

#[derive( Resource)]
pub struct UIData {
    btn_entity:Entity,
    btn2_entity:Entity,
    normal_sprite:usize,
    down_sprite:usize,

}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/default.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let bg_index = ui_sheet.get_index("lm-db").unwrap();
    let btn4down_index = ui_sheet.get_index("Btn4Down").unwrap();
    let btn4_index = ui_sheet.get_index("Btn4On").unwrap();

    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(UIEventSystem::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();

    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -1f32);
    let panel_id = world.spawn((Canvas::default(),panel_t,rect2d)).set_parent(Some(canvas_id)).id();

    let rect2d = Rect2D::new(640f32,480f32);
    let t = Transform::default();
    world.spawn((Sprite::sliced(bg_index, Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id));

    let rect2d = Rect2D::new(120f32,50f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(100f32, 0f32, 0f32);
    let mut event_node = EventNode::default();
    event_node.user_key = Some("test_button".into());
    event_node.event_type = UIEventType::TOUCH_END | UIEventType::TOUCH_START | UIEventType::CLICK;
    let btn_entity = world.spawn((event_node,Sprite::sliced(btn4_index,Some(h_sheet.clone()),Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();

    let rect2d = Rect2D::new(120f32,50f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(-100f32, 0f32, 0f32);
    let mut event_node = EventNode::default();
    event_node.user_key = Some("test_button".into());
    event_node.event_type = UIEventType::MOUSE_ENTER | UIEventType::MOUSE_LEAVE;
    let btn2_entity = world.spawn((event_node,Sprite::sliced(btn4_index,Some(h_sheet.clone()),Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();
    
    world.insert_resource(UIData {
        btn2_entity,
        btn_entity,
        normal_sprite:btn4_index,
        down_sprite:btn4down_index,
    });
}



fn on_update(mut render:EventReader<UIEvent>, _:Commands,
             _: Res<Input>,_: Res<Time>,ui_data: ResMut<UIData>,
             mut sprites: Query<&mut Sprite>) {
    for event in render.iter() {
        if event.entity == ui_data.btn_entity {
            if event.event_type == UIEventType::TOUCH_START {
                let mut sprite = sprites.get_mut(ui_data.btn_entity).unwrap();
                sprite.sprite_index = ui_data.down_sprite;
            } else if event.event_type == UIEventType::TOUCH_END {
                let mut sprite = sprites.get_mut(ui_data.btn_entity).unwrap();
                sprite.sprite_index = ui_data.normal_sprite;
            } else if event.event_type == UIEventType::CLICK {
                log::error!("click!");
            }
        }
        if event.entity == ui_data.btn2_entity {
            if event.event_type == UIEventType::MOUSE_ENTER {
                let mut sprite = sprites.get_mut(ui_data.btn2_entity).unwrap();
                sprite.sprite_index = ui_data.down_sprite;
            } else if event.event_type == UIEventType::MOUSE_LEAVE {
                let mut sprite = sprites.get_mut(ui_data.btn2_entity).unwrap();
                sprite.sprite_index = ui_data.normal_sprite;
            }
        }
    }
}
