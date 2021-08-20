use std::{ops::{Deref, DerefMut}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen { use_size: bool },
}

pub trait IWindow {
    fn set_title(&mut self,str:&str);
    fn title(&self) -> &str;
}

pub struct AppWindow {
    pub inner:Box<dyn IWindow + Send + Sync>
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
    pub mode: WindowMode
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig { 
            width: 1024f32, 
            height: 768f32, 
            title: String::from("seija"), 
            mode: WindowMode::Windowed
        }
    }
}