use bevy_ecs::{prelude::{Component, Entity}, world::World, query::{Changed}};
use seija_core::{window::AppWindow, math::Vec3};
use seija_render::camera::camera::{Camera, Projection};
use seija_transform::Transform;

#[repr(u8)]
pub enum UIScaleMode {
    WithScreenSize,
    ConstantPhysicalSize
}

impl Into<UIScaleMode> for u8 {
    fn into(self) -> UIScaleMode {
        match self {
            0 => UIScaleMode::WithScreenSize,
            _ => UIScaleMode::ConstantPhysicalSize
        }
    }
}

impl Default for UIScaleMode {
    fn default() -> Self {
        UIScaleMode::ConstantPhysicalSize
    }
}

#[repr(C)]
#[derive(Component,Default)]
pub struct UICanvas {
   pub(crate) scale_mode:UIScaleMode
}


pub(crate) fn update_ui_canvas(world:&mut World) {
   let mut changed_canvas = world.query_filtered::<(Entity,&Camera,&UICanvas,&mut Transform),Changed<UICanvas>>();
   let (window_width,window_height) = {
       let app_window = world.get_resource::<AppWindow>().unwrap();
       (app_window.inner.width(),app_window.inner.height())
   };
   
   for (_,camera,_,mut t) in changed_canvas.iter_mut(world) {
      if let Projection::Ortho(ref ortho) = camera.projection {
          let ortho_w = ortho.left.abs() + ortho.right.abs();
          let ortho_h = ortho.top.abs() + ortho.bottom.abs();

          let x_scale = ortho_w / window_width as f32;
          let y_scale = ortho_h / window_height as f32;
          t.local.scale = Vec3::new(x_scale, y_scale, 1f32);
      }
   }  
}