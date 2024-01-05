use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_render::{
    camera::camera::{Camera, Orthographic, SortType}
};
use seija_transform::{Transform,events::EntityMutEx};
use seija_ui::{Rect2D,
    components::{ui_canvas::UICanvas, canvas::Canvas, sprite::Sprite},
     update_ui_render, event::{UIEventSystem}, types::Thickness, layout::types::{LayoutElement, LayoutAlignment, SizeValue},
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
    let add_index = ui_sheet.get_index("Btn3On").unwrap();

    let bg_sprite = Sprite::sliced(bg_index,Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE);
    let rect2d = Rect2D::new(640f32, 480f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, -2f32);
    let mut view = LayoutElement::create_view();
    view.common.ver = LayoutAlignment::Stretch;
    view.common.hor = LayoutAlignment::Stretch;
    view.common.margin = Thickness::new1(20f32);
    let panel_id = world.spawn((view,rect2d,t,Canvas::new(false))).set_parent(Some(canvas_id)).id();
    let e2 = {
        let mut view = LayoutElement::create_view();
        let t = Transform::default();
        let rect2d = Rect2D::new(120f32, 120f32);
        //view.common.margin = Thickness { left:50f32,top:50f32,right:50f32,bottom:50f32 };
        //Sprite::simple(bg_index,Some(h_sheet.clone()), Vec4::ONE)
        world.spawn((bg_sprite,view,rect2d,t))
                         .set_parent(Some(panel_id)).id()
    };
    /*{
        let mut view = LayoutElement::create_view();
        let t = Transform::default();
        let rect2d = Rect2D::new(120f32, 120f32);
        //view.common.ver = LayoutAlignment::Start;
        //view.common.ui_size.height = SizeValue::Pixel(100f32);
        //view.common.margin = Thickness { left:0f32,top:50f32,right:0f32,bottom:0f32 };
        world.spawn((view,bg_sprite,rect2d,t))
                         .set_parent(Some(e2));
    }*/
}