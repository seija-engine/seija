use bevy_ecs::{prelude::*, system::Command, world};
use glam::{Vec3, Vec4};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app, load_material};
use seija_input::Input;
use seija_render::{
    camera::camera::{Camera, Orthographic},
    material::Material,
    resource::{load_image_info, Mesh},
};
//TODO Fix UIRender
use seija_template::Template;
use seija_transform::{hierarchy::Parent, BuildChildren, IEntityChildren, PushChildren, Transform};
use seija_ui::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
     update_ui_render,
};
use smallvec::SmallVec;
use spritesheet::SpriteSheet;

#[derive(Default, Resource)]
pub struct UIData {
    btn: Option<Entity>,
    sprite_index: u32,
    parent: Option<Entity>,
    panel2: Option<Entity>,
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();
    
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let h_sheet = server.load_sync::<SpriteSheet>(world, "ui/ui.json", None).unwrap();
    let sheets = world.get_resource::<Assets<SpriteSheet>>().unwrap();
    let ui_sheet = sheets.get(&h_sheet.id).unwrap();
    let btn3on_index = ui_sheet.get_index("Btn3On").unwrap();
    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, -1f32);
    let panel_id = world.spawn((Canvas::default(),t,rect2d)).set_parent(Some(canvas_id)).id();
    
    let mut t = Transform::default();
    let rect2d = Rect2D::new(100f32, 50f32);
    let btn_id = world.spawn((Sprite::sliced(btn3on_index,Some(h_sheet), Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();

    world.insert_resource(UIData::default());
    
}


fn on_update(mut commands: Commands,input: Res<Input>,time: Res<Time>,ui_data: ResMut<UIData>,mut sprites: Query<&mut Sprite>) {

}
