use std::sync::Arc;

use bevy_ecs::world::World;
use glam::{Vec3, Vec4, Vec2};
use seija_app::App;
use seija_asset::{AssetModule, AssetServer, Assets};
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_examples::init_core_app;
use seija_input::InputModule;
use seija_render::{RenderModule, RenderConfig, camera::camera::{Camera, Orthographic, SortType}};
use seija_transform::{Transform, IEntityChildren};
use seija_ui::{UIModule, update_ui_render, event::UIEventSystem, components::{ui_canvas::UICanvas, canvas::Canvas, rect2d::Rect2D, sprite::Sprite}, layout::types::{LayoutElement, UISize, LayoutAlignment}};
use seija_winit::WinitModule;
use spritesheet::SpriteSheet;


pub fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, on_start);
    

    app.run();
}

fn on_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/default.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let btn3on_index = ui_sheet.get_index("Btn3On").unwrap();

    let mut ui_camera = Camera::from_2d(Orthographic::default());
    ui_camera.sort_type = SortType::Z;
    let event_system = UIEventSystem::default();
    let canvas_id = world.spawn_empty().insert(Transform::default())
                         .insert(ui_camera)
                         .insert(event_system).insert(UICanvas::default()).id();
    let panel_id = {
        let mut item = LayoutElement::create_view();
        item.common.hor = LayoutAlignment::Stretch;
        item.common.ver = LayoutAlignment::Stretch;
        let rect2d = Rect2D::default();
        let mut t = Transform::default();
        t.local.position = Vec3::new(0f32, 0f32, -2f32);
        world.spawn((rect2d,t,Canvas::default(),item)).set_parent(Some(canvas_id)).id()
    };
    let checkbox_id = {
        let mut item = LayoutElement::create_view();
        item.common.hor = LayoutAlignment::Stretch;
        item.common.ver = LayoutAlignment::Stretch;
        let rect2d = Rect2D::default();
        let t = Transform::default();
        world.spawn((rect2d,t,item)).set_parent(Some(panel_id)).id()
    };
    {
        let t = Transform::default();
        let sprite = Sprite::simple(btn3on_index,Some(h_sheet), Vec4::ONE);
        let mut item = LayoutElement::create_view();
        item.common.hor = LayoutAlignment::Center;
        item.common.ver = LayoutAlignment::Start;
        item.common.ui_size = UISize::from_number(Vec2::new(100f32, 100f32));
        let e_btn = world.spawn((sprite,Rect2D::default(),t,item)).set_parent(Some(checkbox_id)).id();
        log::error!("btn:{:?}",e_btn);
    }
}