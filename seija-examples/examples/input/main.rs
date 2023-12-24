use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4, Vec2};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_render::{
    camera::camera::{Camera, Orthographic, SortType}
};
use seija_transform::{events::{EntityMutEx,EntityCommandsEx}, Transform};
use seija_ui::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas, input::Input},
    types::{Thickness, AnchorAlign},
     update_ui_render, text::{Font, Text, LineMode}, event::{EventNode, UIEventType, UIEvent, UIEventSystem},
};
use seija_input::{Input as SysInput, keycode::KeyCode};
use spritesheet::SpriteSheet;

#[derive( Resource)]
pub struct UIData {
    input_entity:Option<Entity>,
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
    
    
    let rect2d = Rect2D::new(640f32, 480f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 100f32, -2f32);
    let panel_id = world.spawn((rect2d,t,Canvas::default())).set_parent(Some(canvas_id)).id();
    {
       let t = Transform::default();
       let rect2d = Rect2D::new(640f32, 480f32);
       let e_bg = world.spawn((Sprite::sliced(bg_index,Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();
       log::error!("bg:{:?}",e_bg);
    }
    
    {
        let canvas = Canvas::default();
        let t = Transform::default();
        let mut rect2d = Rect2D::new(100f32, 50f32);
        let sprite = Sprite::simple(btn3on_index,Some(h_sheet.clone()),Vec4::ONE);
        world.spawn((sprite,rect2d,t,canvas)).set_parent(Some(panel_id));
    };

    
    let text_id = {
        let t = Transform::default();
        let mut rect2d = Rect2D::new(100f32, 50f32);
        let mut text = Text::new(h_font.clone(),"口".to_string());
        text.font_size = 24;
        text.is_auto_size = false;
        text.anchor = AnchorAlign::Left;
        text.line_mode = LineMode::Wrap;
        text.color = Vec4::new(1f32, 1f32, 1f32, 1f32);
        world.spawn((text,rect2d,t)).set_parent(Some(panel_id)).id()
    };

    let input_id = {
        let t = Transform::default();
        let mut rect2d = Rect2D::new(100f32, 50f32);
        let canvas = Canvas::default();
        let mut input = Input::default();
        input.text = String::from("123456");
        input.text_entity = Some(text_id);
        input.font_size = 22;
        input.caret_color = Vec3::new(1f32, 1f32, 1f32);
        let mut event = EventNode::default();
        event.event_type = UIEventType::TOUCH_START;
        let e_text = world.spawn((input,rect2d,t,canvas,event)).set_parent(Some(panel_id)).id();
        log::error!("text:{:?}",e_text);
        e_text
    };
    world.insert_resource(UIData {
        input_entity:Some(input_id)
    });
   
    
}

fn on_update(mut _events:EventReader<UIEvent>,input:Res<SysInput>,mut data:ResMut<UIData>,mut commands:Commands) {
   if input.get_key_down(KeyCode::Delete) {
      if let Some(e) = data.input_entity {
        commands.entity(e).delete();
        data.input_entity = None;
      }
   }
}