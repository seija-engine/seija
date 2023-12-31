use std::{ops::{Deref, DerefMut}};

use bevy_ecs::system::Resource;
use glam::Vec2;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, HasRawDisplayHandle};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

pub trait IWindow : HasRawWindowHandle + HasRawDisplayHandle {
    fn set_title(&mut self,str:&str);
    fn title(&self) -> &str;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn vsync(&self) -> bool;
    fn set_ime_position(&self,pos:Vec2);
    fn set_ime_allowed(&self,value:bool);
    fn set_fullscreen(&self);
    fn set_maximized(&self,value:bool);
    fn set_inner_size(&self,w:f32,h:f32);
}

#[derive(Resource)]
pub struct AppWindow {
    pub inner:Box<dyn IWindow + Send + Sync>
}

unsafe impl HasRawWindowHandle for AppWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for AppWindow {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.inner.raw_display_handle()
    }
}

impl Deref for AppWindow {
    type Target = Box<dyn IWindow + Send + Sync>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AppWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }  
}

impl AppWindow {
    pub fn new(win:impl IWindow + 'static + Send + Sync) -> AppWindow {
        AppWindow {
            inner:Box::new(win)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub mode: WindowMode,
    pub vsync:bool,
    pub title: String,
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig { 
            width: 1024f32, 
            height: 768f32, 
            title: String::from("seija"), 
            mode: WindowMode::Windowed,
            vsync:false,
        }
    }
}


