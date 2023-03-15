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
use seija_template::Template;
use seija_transform::{hierarchy::Parent, BuildChildren, IEntityChildren, PushChildren, Transform};
use seija_ui::{
    components::{panel::Panel, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    types::Thickness,
    update_sprite_alloc_render, SpriteAllocator,
};
use smallvec::SmallVec;

#[derive(Default, Resource)]
pub struct UIData {
    btn: Option<Entity>,
    sprite_index: u32,
    parent: Option<Entity>,
    panel2: Option<Entity>,
}

fn main() {
    let mut app = init_core_app("FRPRender.clj", vec![update_sprite_alloc_render],None);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);

    app.run();
}

fn start(world: &mut World) {
    let mut ui_data = UIData::default();
    let server: AssetServer = world.get_resource::<AssetServer>().unwrap().clone();
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
    let btn3_index = sprite_alloc.alloc(load_image_info(server.full_path("ui/Btn3On.png").unwrap()).unwrap()).unwrap();
    let btn4_index = sprite_alloc.alloc(load_image_info(server.full_path("ui/Btn4On.png").unwrap()).unwrap()).unwrap();

    let ui_camera = Camera::from_2d(Orthographic::default());
    let canvas_id = world.spawn_empty().insert(Transform::default()).insert(ui_camera).insert(UICanvas::default()).id();

    let rect2d = Rect2D::new(1024f32, 768f32);
    let mut panel_t = Transform::default();
    panel_t.local.position = Vec3::new(0f32, 0f32, -1f32);
    let panel_id = world.spawn((Panel::default(),panel_t,rect2d)).set_parent(Some(canvas_id)).id();

    let rect2d = Rect2D::new(640f32,480f32);
    let t = Transform::default();
    let bg_id = world.spawn((Sprite::sliced(index3, Thickness::new1(35f32), Vec4::ONE),rect2d,t)).set_parent(Some(panel_id)).id();
    log::error!("bg sprite id:{:?}",bg_id);
    /*
    let btn_sprite_id = {
        let mut rect2d = Rect2D::default();
        rect2d.width = 138f32;
        rect2d.height = 138f32;
        let t = Transform::default();
        world.spawn((Sprite::simple(index, Vec4::ONE),rect2d,t)).id()
    };
    PushChildren {children: SmallVec::from_slice(&[bg_sprite_id, btn_sprite_id]),parent: panel_id}.write(world);
    */
   

   
    ui_data.sprite_index = index2;
    ui_data.parent = Some(panel_id);
    world.insert_resource(ui_data);
}

fn create_panel2(world: &mut World,btn3_index: u32,btn4_index: u32,parent: Option<Entity>) -> Entity {
    let btn3_size = Rect2D::new(100f32, 45f32);
    let mut panel_t = Transform::default();
    panel_t.local.position.y = 100f32;

    let panel_id = world.spawn((panel_t,Panel::new(false),Rect2D::new(1024f32, 768f32))).id();
    let mut sprite_t: Transform = Transform::default();
    sprite_t.local.position.x = -100f32;
    let sprite_id = world
        .spawn_empty()
        .insert(sprite_t)
        .insert(btn3_size.clone())
        .insert(Sprite::simple(btn3_index, Vec4::ONE))
        .id();
    let mut sprite2_t: Transform = Transform::default();
    sprite2_t.local.position.x = 100f32;
    let sprite2_id = world
        .spawn_empty()
        .insert(sprite2_t)
        .insert(btn3_size)
        .insert(Sprite::simple(btn4_index, Vec4::ONE))
        .id();

    PushChildren {
        children: SmallVec::from_slice(&[sprite_id, sprite2_id]),
        parent: panel_id,
    }
    .write(world);
    if let Some(parent) = parent {
        PushChildren {
            children: SmallVec::from_slice(&[panel_id]),
            parent,
        }
        .write(world);
    }
    panel_id
}

fn on_update(mut commands: Commands,input: Res<Input>,time: Res<Time>,ui_data: ResMut<UIData>,mut sprites: Query<&mut Sprite>) {
    let panel0 = ui_data.parent.unwrap();
    let tick = time.frame();
    use seija_input::keycode::KeyCode;
    //Sprite 增
    if input.get_key_down(KeyCode::A) {
        let rect2d = Rect2D::new(138f32,138f32);
        let t = Transform::default();
        log::error!("sprite add:{}",tick);
        commands.spawn((Sprite::simple(ui_data.sprite_index, Vec4::ONE),rect2d,t)).set_parent(Some(panel0));
    }
    /*
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
             commands.spawn_empty().insert(Sprite::simple(ui_data.sprite_index, Vec4::ONE)) .insert(rect2d).insert(t).id()
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
    } else if input.get_key_down(KeyCode::Q) {
         let panel2 = *ui_data.panel2.as_ref().unwrap();
         commands.entity(panel2).despawn_recursive();
    } else if input.get_key_down(KeyCode::G) {
         log::error!("keydown G");
         let btn_entity = ui_data.btn.unwrap();
         let new_parent = ui_data.panel2.unwrap();
         commands.entity(btn_entity).set_parent(Some(new_parent));
    }*/
}
