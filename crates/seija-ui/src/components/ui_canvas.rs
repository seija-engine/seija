use bevy_ecs::{prelude::{Component, Entity}, world::World};
use seija_render::camera::camera::{Camera, Projection};
pub enum UIScaleMode {
    WithScreenSize,
    ConstantPhysicalSize
}

#[derive(Component)]
pub struct UICanvas {
    camera_entity:Entity,
    scale_mode:UIScaleMode
}


pub(crate) fn update_ui_canvas(world:&mut World) {
   let mut canvas_query = world.query::<&UICanvas>();
   let mut camera_query = world.query::<(Entity,&Camera)>();
   for canvas in canvas_query.iter(world) {
      if let Ok((_,camera)) = camera_query.get(world, canvas.camera_entity) {
           if let Projection::Ortho(ref ortho) = camera.projection {
                
           }
      }
   }
}