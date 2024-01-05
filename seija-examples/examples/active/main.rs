use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4, Vec2};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage, info::EStateInfo};
use seija_examples::init_core_app;
use seija_render::camera::camera::{Camera, Orthographic, SortType};
use seija_transform::{events::{EntityMutEx,WorldEntityEx,EntityCommandsEx}, Transform};
use seija_ui::{Rect2D,
    components::{canvas::Canvas, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
     update_ui_render, text::{Font, Text}, 
     event::{EventNode, UIEventType, UIEvent, UIEventSystem}, 
     layout::{types::{LayoutElement, LayoutAlignment, SizeValue, UISize}, comps::Orientation},
};
use spritesheet::SpriteSheet;

#[derive( Resource)]
pub struct UIData {
    number:i32,
    panel_id:Entity,
    h_font:Handle<Font>,
    btn_id:Entity,
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
    let btn3on_index = ui_sheet.get_index("Btn3On").unwrap();
    let btn4on_index = ui_sheet.get_index("Btn4On").unwrap();
    let btn4_select = ui_sheet.get_index("Btn4Select").unwrap();
   
    
    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, -2f32);
    let view = LayoutElement::create_view();
    let panel_id = world.spawn((rect2d,t,Canvas::default(),view)).set_parent(Some(canvas_id)).id();
    {
       let t = Transform::default();
       let rect2d = Rect2D::new(1024f32, 768f32);
       let bg_sprite = Sprite::sliced(bg_index,Some(h_sheet.clone()),Thickness::new1(35f32), Vec4::ONE);
       let e_bg = world.spawn((bg_sprite,rect2d,t)).set_parent(Some(panel_id)).id();
       log::error!("bg:{:?}",e_bg);
    }
    
    let stack_id = {
        let mut view = LayoutElement::create_stack(10f32, Orientation::Vertical);
        view.common.ui_size.width = SizeValue::Pixel(600f32);
        view.common.ui_size.height = SizeValue::Auto;
        view.common.ver = LayoutAlignment::Start;
        view.common.hor = LayoutAlignment::Stretch;
        let rect2d = Rect2D::default();
        let t = Transform::default();
        let sprite = Sprite::sliced(bg_index,Some(h_sheet.clone()),Thickness::new1(35f32), Vec4::ONE);
        world.spawn((view,rect2d,t,sprite)).set_parent(Some(panel_id)).id()
    };

    let mut add_entitys = vec![];
    for _index in 0..3 {
        let mut view = LayoutElement::create_view();
        
        let t = Transform::default();
        view.common.ui_size.height = SizeValue::Pixel(50f32);
        view.common.ui_size.width = SizeValue::Pixel(200f32);
        let rect2d = Rect2D::new(640f32, 50f32);
        let index = match _index {
            0 => {
                view.common.hor = LayoutAlignment::Start;
                btn4_select
            },
            1 => btn3on_index,
            _ => btn4on_index
        };
        let sprite = Sprite::sliced(index,Some(h_sheet.clone()),Thickness::new1(35f32), Vec4::ONE);
        let item_id = world.spawn((view,rect2d,t,sprite)).set_parent(Some(stack_id)).id();
        add_entitys.push(item_id);
    }
    let btn_id = {
        let t = Transform::default();
        let rect2d = Rect2D::new(32f32, 32f32);
        let mut view = LayoutElement::create_view();
        view.common.ui_size = UISize::from_number(Vec2::new(32f32, 32f32));
        let mut event = EventNode::default();
        event.event_type = UIEventType::CLICK;
        let e_btn = world.spawn((Sprite::simple(add_index,Some(h_sheet.clone()), Vec4::ONE),rect2d,t,event,view)).set_parent(Some(panel_id)).id();
        log::error!("btn:{:?}",e_btn);
        e_btn
    };

    world.insert_resource(UIData { number:0,panel_id,h_font: h_font.clone(),add_entitys,btn_id });
    
}


fn on_update(mut commands: Commands,state_infos:Query<&EStateInfo>,
             time:Res<Time>,ui_data: Res<UIData>,mut render:EventReader<UIEvent>) {
    for event in render.iter() {
        if event.event_type == UIEventType::CLICK {
            let frame = time.frame();
            let opt_entity = ui_data.add_entitys[0];
            let mut info_entity = commands.entity(opt_entity);
            let old_active = state_infos.get(opt_entity).map(|v| v.is_active()).unwrap_or(true);
            println!("set active:{:?} {:?} frame:{:?}",opt_entity,!old_active,frame);
            info_entity.set_active(!old_active);
            if old_active == false {
                commands.entity(ui_data.btn_id).set_active(false);
            }
        }
    }
}
