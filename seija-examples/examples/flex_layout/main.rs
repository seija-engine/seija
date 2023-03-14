use bevy_ecs::prelude::*;
use glam::{Vec4, Vec2};
use seija_asset::{AssetServer};
use seija_core::{ CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_ui::layout::comps::{FlexAlignItems, FlexJustify};
use seija_render::{
    resource::{load_image_info}, camera::camera::{Orthographic, Camera},
};
use seija_transform::{Transform, IEntityChildren};
use seija_ui::{  update_sprite_alloc_render, SpriteAllocator, components::{panel::Panel, ui_canvas::UICanvas, sprite::Sprite, rect2d::Rect2D}, layout::{types::{LayoutElement, LayoutAlignment, SizeValue, UISize}, comps::{FlexLayout, FlexItem}}, types::Thickness };


fn load_sprite(path:&str,server:&AssetServer,sprite_alloc:&mut SpriteAllocator) -> u32 {
    let full_path = server.full_path(path).unwrap();
    let image_info = load_image_info(full_path).unwrap();
    let index = sprite_alloc.alloc(image_info).unwrap();
    index
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render]);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.run();
}



fn start(world: &mut World) {
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
     
     
     for idx in 0..5 {
        let mut view = LayoutElement::create_view();
        view.common.hor = LayoutAlignment::Stretch;
        view.common.ver = LayoutAlignment::Start;
        view.common.margin.top = idx as f32 * 100f32;
        view.common.ui_size.height = SizeValue::Pixel(100f32);
        world.spawn((Sprite::sliced(bg_index, Thickness::new1(35f32), Vec4::ONE),view,Rect2D::default(),Transform::default())).set_parent(Some(canvas_id));
     }
     let justify = vec![FlexJustify::Start,FlexJustify::End,FlexJustify::Center,FlexJustify::SpaceBetween,FlexJustify::SpaceAround];
     for idx in 0..5 {
        let mut flex = LayoutElement::create_flex(FlexLayout {
            align_items: FlexAlignItems::Center,
            justify: justify[idx],
            ..Default::default()
         });
         flex.common.ver = LayoutAlignment::Start;
         flex.common.hor = LayoutAlignment::Stretch;
         flex.common.margin.top = idx as f32 * 100f32;
         flex.common.margin.left = 10f32;
         flex.common.margin.right = 10f32;
         flex.common.ui_size.height = SizeValue::Pixel(100f32);
        
         let flex_id = world.spawn((flex,Rect2D::default(),Panel::default(),Transform::default())).set_parent(Some(canvas_id)).id();
         for _ in 0..3 {
            let item = FlexItem::default();
            let t = Transform::default();
            let mut view = LayoutElement::create_view();
            view.common.ui_size = UISize::from_number(Vec2::new(100f32, 60f32));
            world.spawn((Sprite::sliced(sprite_index,Thickness::new1(20f32), Vec4::ONE),Rect2D::default(),t,view,item)).set_parent(Some(flex_id));
         }
     }
     
     
     
    
}
