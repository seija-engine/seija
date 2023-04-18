use bevy_ecs::{prelude::*};
use glam::{Vec4, Vec2};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{ CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_ui::layout::comps::{FlexAlignItems, FlexJustify, FlexWrap};
use seija_render::{camera::camera::{Orthographic, Camera, SortType},
};
use seija_ui::layout::comps::FlexAlignContent;
use seija_transform::{Transform, IEntityChildren};
use seija_ui::{update_ui_render, 
               components::{canvas::Canvas, ui_canvas::UICanvas, 
               sprite::Sprite, rect2d::Rect2D}, 
               layout::{types::{LayoutElement, LayoutAlignment, SizeValue, UISize}, comps::{FlexLayout, FlexItem}}, types::Thickness };
use spritesheet::SpriteSheet;


fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],Some(Vec2::new(1400f32, 900f32)));
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.run();
}



fn start(world: &mut World) {
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/ui.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let bg_index = ui_sheet.get_index("lm-db").unwrap();
    let sprite_index = ui_sheet.get_index("Btn4On").unwrap();
    //CameraCanvas
    let mut ortho = Orthographic::default();
    ortho.far = 1000f32;
    ortho.near = -1000f32;
    let mut ui_camera = Camera::from_2d(ortho);
    ui_camera.sort_type = SortType::Z;
    let canvas_id = world.spawn_empty().insert(Canvas::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();
    log::error!("canvas_id:{:?}",canvas_id);

    let sprite = Sprite::sliced(bg_index, Some(h_sheet.clone()), Thickness::new1(30f32), Vec4::ONE);
    let mut flex = LayoutElement::create_flex(FlexLayout { 
        ..Default::default()
     });
     flex.common.padding = Thickness::new1(30f32);
    let flex_entity = world.spawn((Transform::default(),Rect2D::default(),sprite,flex)).set_parent(Some(canvas_id)).id();

    let sprite = Sprite::sliced(sprite_index, Some(h_sheet.clone()), Thickness::new1(30f32), Vec4::ONE);
    let mut item = FlexItem::default();
    let mut view = LayoutElement::create_view();
    view.common.ver = LayoutAlignment::Start;
    view.common.hor = LayoutAlignment::Start;
    view.common.ui_size = UISize::from_number(Vec2::new(100f32, 50f32));
    world.spawn((Transform::default(),Rect2D::default(),sprite,item,view)).set_parent(Some(flex_entity));

    let sprite = Sprite::sliced(sprite_index, Some(h_sheet.clone()), Thickness::new1(30f32), Vec4::ONE);
    let mut item = FlexItem::default();
    item.grow = 1f32;
    let mut view = LayoutElement::create_view();
    view.common.ver = LayoutAlignment::Start;
    view.common.hor = LayoutAlignment::Start;
    view.common.ui_size.width = SizeValue::Pixel(100f32);
    view.common.ui_size.height = SizeValue::Pixel(50f32);
    world.spawn((Transform::default(),Rect2D::default(),sprite,item,view)).set_parent(Some(flex_entity));
}

