use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4, Vec2};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_render::{
    camera::camera::{Camera, Orthographic, SortType}
};
use seija_transform::{events::{EntityMutEx}, Transform};
use seija_ui::{
    Rect2D,
    components::{canvas::Canvas, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
     update_ui_render,event::{UIEventSystem},
     layout::types::{LayoutElement, FreeLayoutItem, LayoutAlignment, UISize},
};
use spritesheet::SpriteSheet;

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
    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/default.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let btn3on_index = ui_sheet.get_index("Btn3On").unwrap();
    let bg_index = ui_sheet.get_index("lm-db").unwrap();

    let bg_sprite = Sprite::sliced(bg_index,Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, -2f32);
    let mut view = LayoutElement::create_free();
    view.common.margin = Thickness::new1(20f32);
    let panel_id = world.spawn((bg_sprite,view,Rect2D::default(),t,Canvas::new(false))).set_parent(Some(canvas_id)).id();
    let e2 = {
        let sprite = Sprite::sliced(btn3on_index,Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE);
        let mut view = LayoutElement::create_view();
        view.common.hor = LayoutAlignment::Start;
        view.common.hor = LayoutAlignment::Start;
        view.common.ui_size = UISize::from_number(Vec2::new(50f32, 50f32));
        let t = Transform::default();
        let mut rect2d = Rect2D::default();
        let mut free_item = FreeLayoutItem::default();
        free_item.pos.x = 100f32;
        free_item.pos.y = 100f32;
        world.spawn((free_item,sprite,view,rect2d,t,Canvas::default())).set_parent(Some(panel_id)).id()
    };   
}

fn on_update(mut freeitems:Query<&mut FreeLayoutItem>,time:Res<Time>) {
    for mut item in freeitems.iter_mut() {
       let dt = time.frame() as f32 / 3f32;
       item.pos.x = dt;
       println!("{:?}=====================",item.pos.x);
    }
}

