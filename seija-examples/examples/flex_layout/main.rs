use bevy_ecs::{prelude::*, world};
use glam::{Vec4, Vec2};
use seija_asset::{AssetServer};
use seija_core::{ CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_ui::layout::comps::{FlexAlignItems, FlexJustify, FlexWrap};
use seija_render::{
    resource::{load_image_info}, camera::camera::{Orthographic, Camera},
};
use seija_ui::layout::comps::FlexAlignContent;
use seija_transform::{Transform, IEntityChildren};
use seija_ui::{  update_sprite_alloc_render, SpriteAllocator, components::{panel::Panel, ui_canvas::UICanvas, sprite::Sprite, rect2d::Rect2D}, layout::{types::{LayoutElement, LayoutAlignment, SizeValue, UISize}, comps::{FlexLayout, FlexItem}}, types::Thickness };


fn load_sprite(path:&str,server:&AssetServer,sprite_alloc:&mut SpriteAllocator) -> u32 {
    let full_path = server.full_path(path).unwrap();
    let image_info = load_image_info(full_path).unwrap();
    let index = sprite_alloc.alloc(image_info).unwrap();
    index
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render],Some(Vec2::new(1400f32, 900f32)));
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
     
     let no_wrap_size = Vec2::new(466f32, 100f32);
     let nowrap_infos = vec![(Vec2::ZERO,FlexJustify::Start),
                             (Vec2::new(466f32, 0f32),FlexJustify::Center),
                             (Vec2::new(932f32, 0f32),FlexJustify::End),
                             (Vec2::new(0f32, 100f32),FlexJustify::SpaceBetween),
                             (Vec2::new(466f32, 100f32),FlexJustify::SpaceAround)];
     
     for info in nowrap_infos {
        create_background(world, info.0, no_wrap_size, canvas_id, bg_index);
        let mut flex = LayoutElement::create_flex(FlexLayout {
            justify:info.1,
            warp:FlexWrap::NoWrap,
            ..Default::default()
         });
         flex.common.ver = LayoutAlignment::Start;
         flex.common.hor = LayoutAlignment::Start;
         flex.common.margin.top = info.0.y;
         flex.common.margin.left = info.0.x;
         flex.common.padding = Thickness::new1(10f32);
         flex.common.ui_size = UISize::from_number(no_wrap_size);
         let flex_id = world.spawn((flex,Rect2D::default(),Panel::default(),Transform::default())).set_parent(Some(canvas_id)).id();
         for _ in 0..3 {
            let item = FlexItem::default();
            let t = Transform::default();
            let mut view = LayoutElement::create_view();
            
            view.common.ui_size = UISize::from_number(Vec2::new(100f32, 60f32));
            
            world.spawn((Sprite::sliced(sprite_index,Thickness::new1(20f32), Vec4::ONE),Rect2D::default(),t,view,item)).set_parent(Some(flex_id));
         }
     }
     let wrap_infos = vec![FlexAlignContent::Start,FlexAlignContent::Center,FlexAlignContent::End,
                          FlexAlignContent::Stretch,FlexAlignContent::SpaceBetween,FlexAlignContent::SpaceAround];
     let wrap_size = Vec2::new(233f32, 700f32);
     for idx in 0..6 {
        let cur_x = idx as f32 * 233f32;
        let cur_y = 200f32;
        create_background(world, Vec2::new(cur_x, cur_y), wrap_size, canvas_id, bg_index);
        let mut flex = LayoutElement::create_flex(FlexLayout {
            align_items:FlexAlignItems::Start,
            align_content:wrap_infos[idx],
            warp:FlexWrap::Wrap,
            ..Default::default()
        });
        flex.common.hor = LayoutAlignment::Start;
        flex.common.ver = LayoutAlignment::Start;
        flex.common.ui_size = UISize::from_number(wrap_size);
        flex.common.margin.top = cur_y;
        flex.common.margin.left = cur_x;
        flex.common.padding.left = 15f32;
        flex.common.padding.top = 15f32;
        flex.common.padding.bottom = 15f32;
        let flex_id = world.spawn((flex,Rect2D::default(),Panel::default(),Transform::default())).set_parent(Some(canvas_id)).id();
        for _ in 0..6 {
            let item = FlexItem::default();
            let t = Transform::default();
            let mut view = LayoutElement::create_view();

            view.common.ui_size = UISize::from_number(Vec2::new(100f32, 60f32));
            world.spawn((Sprite::sliced(sprite_index,Thickness::new1(20f32), Vec4::ONE),Rect2D::default(),t,view,item)).set_parent(Some(flex_id));
        }
     }
}

fn create_background(world:&mut World,pos:Vec2,size:Vec2,canvas_id: Entity, bg_index: u32) {
    let mut view = LayoutElement::create_view();
    view.common.margin.top = pos.y;
    view.common.margin.left = pos.x;
    view.common.hor = LayoutAlignment::Start;
    view.common.ver = LayoutAlignment::Start;
    view.common.ui_size = UISize::from_number(size);
    world.spawn((Sprite::sliced(bg_index, Thickness::new1(35f32), Vec4::ONE),view,Rect2D::default(),Transform::default())).set_parent(Some(canvas_id));
}


