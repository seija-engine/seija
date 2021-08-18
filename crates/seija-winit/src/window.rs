use seija_core::window::{AppWindow,IWindow};
use winit::window::{Window};
pub struct WinitWindow {
    title:String,
    window:Option<Window>
}


impl IWindow for WinitWindow {
    fn set_title(&mut self,str:&str) {
        self.title = String::from(str);
        self.window.as_mut().map(|w| w.set_title(str));
    }

    fn title(&self) -> &str {
       self.title.as_str()
    }
}

impl WinitWindow {
    
}