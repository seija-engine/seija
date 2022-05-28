mod log_node;
mod pass_node;
mod swapchain_node;
mod camera_collect;
mod transform_collect;
mod light_collect;
mod ubo_array_collect;
mod screen_texture_node;
mod shadow_map;

use bevy_ecs::prelude::World;
use seija_core::{event::{ManualEventReader, Events}, window::AppWindow};
use seija_winit::event::{WindowResized, WindowCreated};
pub use ubo_array_collect::{UBOArrayCollect};
pub use log_node::LogNode;
pub use pass_node::PassNode;
pub use swapchain_node::SwapchainNode;
pub use camera_collect::{CameraCollect};
pub use transform_collect::{TransformCollect};
pub use light_collect::{LightCollect};
pub use screen_texture_node::{ScreenTextureNode};
pub use shadow_map::{ShadowMapNode};


#[derive(Default)]
pub struct WindowEvent {
    pub window_resized_event_reader: ManualEventReader<WindowResized>,
    pub window_created_event_reader: ManualEventReader<WindowCreated>,
    last_win_size:(u32,u32),
}

impl WindowEvent {
    pub fn get_new_window_size(&mut self,world:&World) -> Option<(u32,u32)> {
        if let Some(events) =  world.get_resource::<Events<WindowCreated>>() {
            if self.window_created_event_reader.iter(events).next().is_some() {
                let app_window = world.get_resource::<AppWindow>().unwrap();
                self.last_win_size = (app_window.width(),app_window.height());
                return Some(self.last_win_size);
            }
        }

        if let Some(events) =  world.get_resource::<Events<WindowResized>>() {
            if self.window_resized_event_reader.iter(events).next().is_some() {
                let app_window = world.get_resource::<AppWindow>().unwrap();
                let new_size = (app_window.width(),app_window.height());
                if self.last_win_size != new_size && new_size.0 > 0 && new_size.1 > 0 {
                    self.last_win_size = new_size;
                    return Some(self.last_win_size)
                }
            }
        }

        None
    }
}