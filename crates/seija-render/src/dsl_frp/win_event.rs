use bevy_ecs::prelude::World;
use seija_core::{ window::AppWindow};
use seija_winit::event::{WindowResized, WindowCreated};
use bevy_ecs::event::{ManualEventReader, Events};

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