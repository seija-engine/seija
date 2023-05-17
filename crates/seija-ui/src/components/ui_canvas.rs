use bevy_ecs::{prelude::{Component, Entity, EventReader,Res}, query::{Changed}, system::Query};
use seija_core::{window::AppWindow, math::Vec3};
use seija_render::camera::camera::{Camera, Projection};
use seija_transform::Transform;
use seija_winit::event::WindowResized;

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


pub(crate) fn update_ui_canvas(mut ui_canvas:Query<(&mut Transform,&UICanvas,&Camera)>,
                               canvas_changed:Query<Entity,Changed<UICanvas>>,
                               resize_event:EventReader<WindowResized>,
                               window:Res<AppWindow>) {
   let has_canvas_change = !canvas_changed.is_empty();
   let has_win_change = !resize_event.is_empty();
   if has_canvas_change || has_win_change {
    let window_width = window.width();
    let window_height = window.height();
    for (mut t,_,camera) in ui_canvas.iter_mut() {
        if let Projection::Ortho(ref ortho) = camera.projection {
            let ortho_w = ortho.left.abs() + ortho.right.abs();
            let ortho_h = ortho.top.abs() + ortho.bottom.abs();
  
            let x_scale = ortho_w / window_width as f32;
            let y_scale = ortho_h / window_height as f32;
            t.local.scale = Vec3::new(x_scale, y_scale, 1f32);
        }
    }
   }
}