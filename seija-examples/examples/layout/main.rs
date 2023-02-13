use bevy_ecs::prelude::*;
use glam::{Vec3, Vec4, Vec2, Quat};
use seija_asset::{AssetServer};
use seija_core::{time::Time, CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_input::Input;

use seija_render::{
    camera::camera::{Camera, Orthographic},
    resource::{load_image_info},
};
use seija_transform::{IEntityChildren,Transform};
use seija_ui::{
    components::{panel::Panel, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
    update_sprite_alloc_render, SpriteAllocator, layout::{types::{LayoutElement, LayoutAlignment}, comps::Orientation},
};

#[derive(Default, Resource)]
pub struct UIData {
   
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render]);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn load_sprite(path:&str,server:&AssetServer,sprite_alloc:&mut SpriteAllocator) -> u32 {
    let full_path = server.full_path(path).unwrap();
    let image_info = load_image_info(full_path).unwrap();
    let index = sprite_alloc.alloc(image_info).unwrap();
    index
}

fn start(world: &mut World) {
    let ui_data = UIData::default();
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let mut sprite_alloc = world.get_resource_mut::<SpriteAllocator>().unwrap();
    let bg_index = load_sprite("ui/lm-db.png", &server, &mut sprite_alloc);
    let sprite_index = load_sprite("ui/Btn4On.png", &server, &mut sprite_alloc);
    //CameraCanvas
    let mut ortho = Orthographic::default();
    ortho.far = 1000f32;
    ortho.near = -1000f32;
    let ui_camera = Camera::from_2d(ortho);
    let canvas_id = world.spawn_empty().insert(Panel::default()).insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();
    //背景图
    let mut view = LayoutElement::create_view();
    view.common.hor = LayoutAlignment::Stretch;
    view.common.ver = LayoutAlignment::End;
    view.common.size = Vec2::new(-1f32, 100f32);
    world.spawn((Sprite::sliced(bg_index, Thickness::new1(35f32), Vec4::ONE),view,Rect2D::default(),Transform::default())).set_parent(Some(canvas_id));
    

    //let stack_id = create_stackpanel(world, Some(canvas_id));
    //create_sprite(world,sprite_index,Some(stack_id));
    
      
    world.insert_resource(ui_data);
}


fn create_stackpanel(world: &mut World,parent:Option<Entity>) -> Entity {
    let rect2d = Rect2D::default();
    let t = Transform::default();
    let stack_layout = LayoutElement::create_stack(10f32, Orientation::Vertical);
    world.spawn((rect2d,t,stack_layout,Panel::default())).set_parent(parent).id()
}

fn create_sprite(world:&mut World,sprite_index:u32,parent:Option<Entity>) {
    let mut view_layout = LayoutElement::create_view();
    view_layout.common.hor = LayoutAlignment::Stretch;
    view_layout.common.ver = LayoutAlignment::Start;
    view_layout.common.size.y = 50f32;
    let t = Transform::default();
    world.spawn((Sprite::sliced(sprite_index,Thickness::new1(20f32), Vec4::ONE),Rect2D::default(),t,view_layout)).set_parent(parent);
}

fn on_update(mut commands: Commands,input: Res<Input>,time: Res<Time>,ui_data: ResMut<UIData>,mut sprites: Query<&mut Sprite>) {
    
   
}
