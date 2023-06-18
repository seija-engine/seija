use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_render::{
    camera::camera::{Camera, Orthographic, SortType}
};
use seija_transform::{Transform,IEntityChildren};
use seija_ui::{
    components::{ui_canvas::UICanvas, rect2d::Rect2D, canvas::Canvas, sprite::Sprite},
     update_ui_render, text::{Font}, event::{UIEventSystem}, types::Thickness, layout::types::{LayoutElement, LayoutAlignment, SizeValue},
};
use spritesheet::SpriteSheet;



fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    //for stage in app.schedule.iter_stages() {
    //    println!("=================={:?}",stage.0.as_str());
    //}
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
    let bg_index = ui_sheet.get_index("lm-db").unwrap();
    let add_index = ui_sheet.get_index("dl").unwrap();

    let bg_sprite = Sprite::sliced(bg_index,Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE);
    let rect2d = Rect2D::new(640f32, 480f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, -2f32);
    //let mut view = LayoutElement::create_view();
    //view.common.ver = LayoutAlignment::Start;
    //view.common.ui_size.height = SizeValue::Pixel(300f32);
    let panel_id = world.spawn((bg_sprite,rect2d,t,Canvas::new(true))).set_parent(Some(canvas_id)).id();
    {
        let mut t = Transform::default();
        let rect2d = Rect2D::new(120f32, 120f32);
        t.local.position.x = 300f32;
        t.local.position.y = -200f32;
        world.spawn((Sprite::simple(add_index,Some(h_sheet), Vec4::ONE),rect2d,t))
                         .set_parent(Some(panel_id));
    }
}