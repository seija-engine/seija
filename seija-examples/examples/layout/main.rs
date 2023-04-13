use bevy_ecs::prelude::*;
use glam::{Vec4};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_input::Input;
use seija_render::{
    camera::camera::{Camera, Orthographic, SortType},
   
};
use seija_transform::{IEntityChildren,Transform};
use seija_ui::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
    update_ui_render, layout::{types::{LayoutElement, LayoutAlignment, SizeValue}, comps::Orientation},
};
use spritesheet::SpriteSheet;

#[derive(Default, Resource)]
pub struct UIData {
   
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_ui_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}



fn start(world: &mut World) {
    let ui_data = UIData::default();
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
    //背景图
    let mut view = LayoutElement::create_view();
    view.common.hor = LayoutAlignment::Stretch;
    view.common.ver = LayoutAlignment::End;
    view.common.ui_size.height = SizeValue::Pixel(200f32);
    let bg_id = world.spawn((Sprite::sliced(bg_index,Some(h_sheet.clone()), 
                            Thickness::new1(35f32), Vec4::ONE),view,
                            Rect2D::default(),Transform::default())).set_parent(Some(canvas_id)).id();
    log::error!("bg_id:{:?}",bg_id);
    //StackPanel 
    let stack_id = create_stackpanel(world, Some(canvas_id));
    log::error!("stack_id:{:?}",stack_id);
    create_sprite(world,sprite_index,Some(stack_id),h_sheet.clone(),LayoutAlignment::Start);
    create_sprite(world,sprite_index,Some(stack_id),h_sheet.clone(),LayoutAlignment::Center);
    create_sprite(world,sprite_index,Some(stack_id),h_sheet.clone(),LayoutAlignment::End);
    create_sprite(world,sprite_index,Some(stack_id),h_sheet.clone(),LayoutAlignment::Stretch);
    create_sprite(world,sprite_index,Some(stack_id),h_sheet.clone(),LayoutAlignment::Center);

    world.insert_resource(ui_data);  
}

fn create_sprite(world:&mut World,sprite_index:usize,parent:Option<Entity>,sheet:Handle<SpriteSheet>,ver:LayoutAlignment) -> Entity {
    let mut view_layout = LayoutElement::create_view();
    view_layout.common.hor = LayoutAlignment::Center;
    view_layout.common.ver = ver;
    view_layout.common.ui_size.width = SizeValue::Pixel(100f32);
    if ver != LayoutAlignment::Stretch {
        view_layout.common.ui_size.height = SizeValue::Pixel(50f32);
    } else {
        view_layout.common.ui_size.height = SizeValue::Auto;
    }
    let t = Transform::default();
    world.spawn((Sprite::sliced(sprite_index,Some(sheet),Thickness::new1(20f32), Vec4::ONE),Rect2D::default(),t,view_layout)).set_parent(parent).id()
}

fn create_stackpanel(world: &mut World,parent:Option<Entity>) -> Entity {
    let rect2d = Rect2D::default();
    let t = Transform::default();
    let mut stack_layout = LayoutElement::create_stack(10f32, Orientation::Horizontal);
    stack_layout.common.ui_size.height = SizeValue::Pixel(200f32);
    stack_layout.common.padding.left = 50f32;
    stack_layout.common.padding.top = 10f32;
    stack_layout.common.padding.bottom = 10f32;
    stack_layout.common.hor = LayoutAlignment::Stretch;
    stack_layout.common.ver = LayoutAlignment::End;
    world.spawn((rect2d,t,stack_layout,Canvas::default())).set_parent(parent).id()
}



fn on_update(mut commands: Commands,input: Res<Input>,time: Res<Time>,ui_data: ResMut<UIData>,mut sprites: Query<&mut Sprite>) {
    
   
}
