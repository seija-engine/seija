use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use seija_core::window::{IWindow, WindowConfig,WindowMode};
use winit::{dpi::LogicalSize, event_loop::EventLoop, monitor::{MonitorHandle, VideoMode}, window::{Window,Fullscreen}, platform::windows::WindowBuilderExtWindows};
use winit::platform::windows;
pub struct WinitWindow {
    title:String,
    vsync:bool,
    window:Window
}

unsafe impl HasRawWindowHandle for WinitWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        
        self.window.raw_window_handle()
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
                builder.with_inner_size(LogicalSize::new(config.width,config.height))
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
                Equal => b.refresh_rate().cmp(&a.refresh_rate()),
                default => default,
            },
            default => default,
        }
    });
    modes.first().unwrap().clone()
}