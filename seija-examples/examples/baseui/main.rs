use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_render::{
    camera::camera::{Camera, Orthographic, SortType}
};
use seija_transform::{events::{EntityMutEx,EntityCommandsEx}, Transform};
use seija_ui::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
     update_ui_render, text::{Font, Text, LineMode}, event::{EventNode, UIEventType, UIEvent, UIEventSystem},
};
use spritesheet::SpriteSheet;

#[derive( Resource)]
pub struct UIData {
    text:Entity,
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
    let btn3on_index = ui_sheet.get_index("Btn3On").unwrap();
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
    
    let text_id = {
        let mut t = Transform::default();
        t.local.position.x = 60f32;
        let rect2d = Rect2D::new(100f32, 50f32);
        let mut text = Text::new(h_font.clone(),"swash demo".to_string());
        text.font_size = 12;
        text.line_mode = LineMode::Wrap;
        let canvas = Canvas::default();
        let e_text = world.spawn((text,rect2d,t,canvas)).set_parent(Some(panel_id)).id();
        log::error!("text:{:?}",e_text);
        e_text
    };
   

    world.insert_resource(UIData {text:text_id,number:0,panel_id,h_font: h_font.clone(),add_entitys:vec![] });
    
}


fn on_update(mut commands: Commands,
             mut trans:Query<&mut Transform>,
             mut texts:Query<&mut Text>,
             time: Res<Time>,
             mut ui_data: ResMut<UIData>,
             mut render:EventReader<UIEvent>) {
    for event in render.iter() {
        ui_data.number += 1;
        let mut number_text = texts.get_mut(ui_data.text).unwrap();
        if let Ok(mut text_trans) = trans.get_mut(ui_data.text) {
            text_trans.local.position.x = 30f32 + ui_data.number as f32 * 15f32;
        }
       
        number_text.text = format!("+{}",ui_data.number);
        if ui_data.number <= 5 {
            let mut t = Transform::default();
            t.local.position.x = 30f32;
            t.local.position.y = ui_data.number as f32 * 25f32;
            let rect2d = Rect2D::new(100f32, 50f32);
            let mut text = Text::new(ui_data.h_font.clone(),format!("新-{:?}",ui_data.number));
            text.font_size = 24;
            let text_id = commands.spawn((text,rect2d,t)).set_parent(Some(ui_data.panel_id)).id();
            ui_data.add_entitys.push(text_id);
        } else if ui_data.number == 6 {
           
            if let Some(pop_entity) = ui_data.add_entitys.pop() {
                commands.entity(pop_entity).delete();
                log::error!("despawn {:?}",&ui_data.add_entitys);
            }
            std::process::exit(0);
        } else {
            if let Some(pop_entity) = ui_data.add_entitys.pop() {
                commands.entity(pop_entity).set_parent(Some(ui_data.text));
            }
        }
    }
}
