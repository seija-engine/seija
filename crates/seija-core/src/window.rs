use std::{ops::{Deref, DerefMut}};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen { use_size: bool },
}

pub trait IWindow : HasRawWindowHandle {
    fn set_title(&mut self,str:&str);
    fn title(&self) -> &str;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn vsync(&self) -> bool;
}



pub struct AppWindow {
    pub inner:Box<dyn IWindow + Send + Sync>
}

unsafe impl HasRawWindowHandle for AppWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
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

pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
    pub mode: WindowMode,
    pub vsync:bool,
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig { 
            width: 640f32, 
            height: 480f32, 
            title: String::from("seija"), 
            mode: WindowMode::Windowed,
            vsync:false,
        }
    }
}