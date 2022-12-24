use bevy_ecs::{prelude::{Component, Entity}, world::World, query::{Changed, Added}};
use seija_core::{window::AppWindow, math::Vec3};
use seija_render::camera::camera::{Camera, Projection};
use seija_transform::Transform;
pub enum UIScaleMode {
    WithScreenSize,
    ConstantPhysicalSize
}

impl Default for UIScaleMode {
    fn default() -> Self {
        UIScaleMode::ConstantPhysicalSize
    }
}

#[derive(Component,Default)]
pub struct UICanvas {
    scale_mode:UIScaleMode
}


pub(crate) fn update_ui_canvas(world:&mut World) {
   let mut changed_canvas = world.query_filtered::<(Entity,&Camera,&UICanvas,&mut Transform),Changed<UICanvas>>();
   let (window_width,window_height) = {
       let app_window = world.get_resource::<AppWindow>().unwrap();
       (app_window.inner.width(),app_window.inner.height())
   };
   for (entity,camera,canvas,mut t) in changed_canvas.iter_mut(world) {
      if let Projection::Ortho(ref ortho) = camera.projection {
          let ortho_w = ortho.left.abs() + ortho.right.abs();
          let ortho_h = ortho.top.abs() + ortho.bottom.abs();

          let x_scale = ortho_w / window_width as f32;
          let y_scale = ortho_h / window_height as f32;
          t.local.scale = Vec3::new(x_scale, y_scale, 1f32);
      }
   }  
}