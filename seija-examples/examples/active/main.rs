use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage, info::EStateInfo};
use seija_examples::init_core_app;
use seija_render::camera::camera::{Camera, Orthographic, SortType};
use seija_transform::{events::{EntityMutEx,WorldEntityEx,EntityCommandsEx}, Transform};
use seija_ui::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
     update_ui_render, text::{Font, Text}, event::{EventNode, UIEventType, UIEvent, UIEventSystem},
};
use spritesheet::SpriteSheet;

#[derive( Resource)]
pub struct UIData {
    number:i32,
    panel_id:Entity,
    h_font:Handle<Font>,
    add_entitys:Vec<Entity>
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
    let mut ui_camera = Camera::from_2d(Orthographic::default());
    ui_camera.sort_type = SortType::Z;
    let event_system = UIEventSystem::default();
    let canvas_id = world.spawn_empty().insert(Transform::default())
                         .insert(ui_camera)
                         .insert(event_system).insert(UICanvas::default()).id();
    
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let h_font = server.load_sync::<Font>(world, "ui/WenQuanYiMicroHei.ttf", None).unwrap();
    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/default.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let bg_index = ui_sheet.get_index("lm-db").unwrap();
    let add_index = ui_sheet.get_index("AddOn").unwrap();
    
    
    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, -2f32);
    let panel_id = world.spawn((rect2d,t,Canvas::default())).set_parent(Some(canvas_id)).id();
    {
       let t = Transform::default();
       let rect2d = Rect2D::new(1024f32, 768f32);
       let e_bg = world.spawn((Sprite::sliced(bg_index,Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();
       log::error!("bg:{:?}",e_bg);
    }
    {
        let mut t = Transform::default();
        let rect2d = Rect2D::new(32f32, 32f32);
        t.local.position.x = -50f32;
        let mut event = EventNode::default();
        event.event_type = UIEventType::CLICK;
        let e_btn = world.spawn((Sprite::simple(add_index,Some(h_sheet), Vec4::ONE),rect2d,t,event)).set_parent(Some(panel_id)).id();
        log::error!("btn:{:?}",e_btn);
        
     
    }
   

    world.insert_resource(UIData { number:0,panel_id,h_font: h_font.clone(),add_entitys:vec![] });
    
}


fn on_update(mut commands: Commands,
             mut trans:Query<&mut Transform>,
             state_infos:Query<&EStateInfo>,
             time: Res<Time>,
             mut ui_data: Res<UIData>,
             mut render:EventReader<UIEvent>) {
    for event in render.iter() {
        if event.event_type == UIEventType::CLICK {
            let frame = time.frame();
            let mut info_entity = commands.entity(ui_data.panel_id);
            let old_active = state_infos.get(ui_data.panel_id).map(|v| v.is_active()).unwrap_or(true);
            println!("set active:{:?} {:?} frame:{:?}",ui_data.panel_id,!old_active,frame);
            info_entity.set_active(!old_active);
        }
    }
}
