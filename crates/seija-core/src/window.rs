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

unsafe impl Send for AppWindow {}
unsafe impl Sync for AppWindow {}
pub struct AppWindow {
    pub inner:Box<dyn IWindow>
}



impl AppWindow {
    pub fn new(win:impl IWindow + 'static) -> AppWindow {
        AppWindow {
            inner:Box::new(win)
        }
    }

    pub fn inner(&self) -> &Box<dyn IWindow>  {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Box<dyn IWindow> {
        &mut self.inner
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