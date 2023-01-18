use bevy_ecs::{prelude::*, system::Command};
use glam::{Vec4, Vec3};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{CoreStage, StartupStage, time::Time};
use seija_examples::{init_core_app, load_material};
use seija_input::Input;
use seija_render::{resource::{load_image_info, Mesh}, camera::camera::{Camera, Orthographic}, material::Material};
use seija_template::Template;
use seija_transform::{Transform, PushChildren, hierarchy::Parent, BuildChildren};
use seija_ui::{components::{panel::Panel, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas}, update_sprite_alloc_render, SpriteAllocator, types::Thickness};
use smallvec::SmallVec;

#[derive(Default)]
pub struct UIData {
    btn:Option<Entity>,
    sprite_index:u32,
    parent:Option<Entity>
}

fn main() {
    let mut app = init_core_app("FRPRender.clj",vec![update_sprite_alloc_render]);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, on_update);
    
    app.run();
}

fn start(world:&mut World) {
    let mut ui_data = UIData::default();
    let server:AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
    let mut sprite_alloc = world.get_resource_mut::<SpriteAllocator>().unwrap();
    let btn_path = server.full_path("ui/dl.png").unwrap();
    let btn2_path = server.full_path("ui/Btn_V04.png").unwrap();
    let bg_path = server.full_path("ui/lm-db.png").unwrap();
    let image_info = load_image_info(btn_path).unwrap();
    let image_info2 = load_image_info(btn2_path).unwrap();
    let image_info3 = load_image_info(bg_path).unwrap();
    let index = sprite_alloc.alloc(image_info).unwrap();
    let index2 = sprite_alloc.alloc(image_info2).unwrap();
    let index3 = sprite_alloc.alloc(image_info3).unwrap();

    let ui_camera = Camera::from_2d(Orthographic::default());
    
    let canvas_id = world.spawn().insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();

    let mut rect2d = Rect2D::default();
    rect2d.width = 1024f32;
    rect2d.height = 768f32;
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -60f32);
    let panel_id = world.spawn().insert(Panel::default()).insert(panel_t).insert(rect2d).id();
    PushChildren {children:SmallVec::from_slice(&[panel_id]),parent:canvas_id}.write(world);   
    let bg_sprite_id = {
        let mut rect2d = Rect2D::default();
        rect2d.width = 640f32;
        rect2d.height = 480f32;
        let mut t = Transform::default();
        world.spawn().insert(Sprite::sliced(index3,Thickness::new1(35f32),Vec4::ONE))
                                             .insert(rect2d)
                                             .insert(t).id()
    };
    let btn_sprite_id = {
        let mut rect2d = Rect2D::default();
        rect2d.width = 138f32;
        rect2d.height = 138f32;
        let mut t = Transform::default();
        t.local.position.y = -100f32;
        world.spawn().insert(Sprite::simple(index, Vec4::ONE))
                                             .insert(rect2d)
                                             .insert(t).id()
    };
    PushChildren {children:SmallVec::from_slice(&[bg_sprite_id,btn_sprite_id]),parent:panel_id}.write(world);   
    ui_data.btn = Some(btn_sprite_id);
    ui_data.sprite_index = index2;
    ui_data.parent = Some(panel_id);
   world.insert_resource(ui_data);
}

fn on_update(mut commands:Commands,input:Res<Input>,time:Res<Time>,ui_data:ResMut<UIData>,mut sprites:Query<&mut Sprite>) {
   let tick = time.frame();
   use seija_input::keycode::KeyCode;
   if input.get_key_down(KeyCode::U) {
        if let Some(id) = ui_data.btn {
           if let Ok(mut sprite) = sprites.get_mut(id) {
              sprite.sprite_index = Some(ui_data.sprite_index);
              log::error!("update set sprite {:?} index:{:?}",&id,ui_data.sprite_index);
           }
        }
   } else if input.get_key_down(KeyCode::A) {
        let btn_sprite_id = {
            let mut rect2d = Rect2D::default();
            rect2d.width = 366f32;
            rect2d.height = 67f32;
            let mut t = Transform::default();
            t.local.position.y = 0f32;
            commands.spawn().insert(Sprite::simple(ui_data.sprite_index, Vec4::ONE)) .insert(rect2d).insert(t).id() 
        };
        let parent_id = *ui_data.parent.as_ref().unwrap();
        commands.entity(parent_id).add_children(&vec![btn_sprite_id]);
   } else if input.get_key_down(KeyCode::D) {
        let parent_id = *ui_data.parent.as_ref().unwrap();
        commands.entity(parent_id).despawn();
   } else if input.get_key_down(KeyCode::W) {
        log::error!("keydown W {}",tick);
        let btn_id = *ui_data.btn.as_ref().unwrap();
        commands.entity(btn_id).despawn();
   }
}