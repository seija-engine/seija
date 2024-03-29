use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, HasRawDisplayHandle};
use seija_core::{window::{IWindow, WindowConfig,WindowMode}, math::Vec2};
use winit::{dpi::{PhysicalSize, PhysicalPosition}, event_loop::EventLoop, monitor::{MonitorHandle, VideoMode}, window::{Window,Fullscreen}};
#[cfg(target_os = "windows")] 
use winit::platform::windows::WindowBuilderExtWindows;
pub struct WinitWindow {
    title:String,
    vsync:bool,
    pub window:Window
}

unsafe impl HasRawWindowHandle for WinitWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        
        self.window.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for WinitWindow {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.window.raw_display_handle()
    }
}


impl IWindow for WinitWindow {
    fn set_title(&mut self,str:&str) {
        self.title = String::from(str);
        self.window.set_title(str);
    }

    fn title(&self) -> &str {  self.title.as_str() }

    fn width(&self) -> u32 { self.window.inner_size().width }

    fn height(&self) -> u32 { self.window.inner_size().height }

    fn vsync(&self) -> bool { self.vsync }
    
    fn set_ime_position(&self,pos:Vec2) {
        self.window.set_ime_position(PhysicalPosition::new(pos.x, pos.y));
    }
    fn set_ime_allowed(&self,value:bool) {
        self.window.set_ime_allowed(value);
    }
    fn set_fullscreen(&self) {
        self.window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    }

    fn set_maximized(&self,value:bool) {
        self.window.set_maximized(value);
    }

    fn set_inner_size(&self,w:f32,h:f32) {
        self.window.set_inner_size(PhysicalSize::new(w, h));
    }
}

impl WinitWindow {
    pub fn from_config(config:&WindowConfig) -> (WinitWindow,EventLoop<()>) {
        let event_loop = EventLoop::new();
        let mut builder = winit::window::WindowBuilder::new();
        builder = match config.mode {
            WindowMode::BorderlessFullscreen => { 
                builder.with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
            },
            WindowMode::Fullscreen => {
               let max_mode = get_max_video_mode(&event_loop.primary_monitor().unwrap());
               builder.with_fullscreen(Some(Fullscreen::Exclusive(max_mode)))
            },
            WindowMode::Windowed => { 
                builder.with_inner_size(PhysicalSize::new(config.width,config.height))
             }
        };
        #[cfg(target_os = "windows")] 
        {
            builder = builder.with_drag_and_drop(false);
        };
        let window = builder.with_title(&config.title).build(&event_loop).unwrap();
        (WinitWindow {  title: config.title.clone(), window,vsync:config.vsync },event_loop)
    }
}

fn get_max_video_mode(monitor: &MonitorHandle) -> VideoMode {
    let mut modes = monitor.video_modes().collect::<Vec<_>>();
    modes.sort_by(|a, b| {
        use std::cmp::Ordering::*;
        match b.size().width.cmp(&a.size().width) {
            Equal => match b.size().height.cmp(&a.size().height) {
                Equal => b.refresh_rate_millihertz().cmp(&a.refresh_rate_millihertz()),
                default => default,
            },
            default => default,
        }
    });
    modes.first().unwrap().clone()
}