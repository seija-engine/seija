use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4, Vec2};
use seija_asset::{AssetServer, Assets};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::init_core_app;
use seija_render::camera::camera::{Camera, Orthographic, SortType};
use seija_transform::{events::EntityMutEx, Transform};
use seija_ui::{
    Rect2D,Thickness,
    components::{canvas::Canvas, sprite::Sprite, ui_canvas::UICanvas},
     update_ui_render,event::UIEventSystem,
     layout::{comps::{LSize, TiledItem}, types::{LayoutElement, FreeLayoutItem, LayoutAlignment, UISize}},
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
    view.common.margin = Thickness::new1(0f32);
    let panel_id = world.spawn((bg_sprite,view,Rect2D::default(),t,Canvas::new(false))).set_parent(Some(canvas_id)).id();

    let layout_id = {
        let left = TiledItem::empty(1);
        let right = TiledItem::empty(2);
        let row = TiledItem::row(0, LSize::Rate(0.3f32), vec![left,right]);
        let layout = LayoutElement::create_tiled(row);
        
        let bg_sprite = Sprite::sliced(bg_index,Some(h_sheet.clone()), Thickness::new1(35f32), Vec4::ONE);
        let t = Transform::default();
        let  rect2d = Rect2D::default();
        world.spawn((layout,bg_sprite,rect2d,t)).set_parent(Some(panel_id)).id()
    };

}

fn on_update(time:Res<Time>) {
    
}

