mod window;
pub mod event;
use event::{WindowCreated, WindowResized};
use seija_app::{IModule,App};
use seija_core::{event::{Events}, window::{AppWindow, WindowConfig},AddCore};
use window::WinitWindow;
use winit::{event::{Event,WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}};

#[derive(Default)]
pub struct WinitModule(pub WindowConfig);

impl IModule for WinitModule {
    fn init(&mut self,app:&mut App) {
        let (winit_window,event_loop) = WinitWindow::from_config(&self.0);
        let app_window = AppWindow::new(winit_window);
        app.add_event::<WindowCreated>();
        app.add_event::<WindowResized>();
        let mut window_created_events = app.world.get_resource_mut::<Events<WindowCreated>>().unwrap();
        window_created_events.send(WindowCreated);
 
        app.add_resource(app_window);
        app.set_runner(|app| { winit_runner(event_loop,app); });
    }
}


fn winit_runner(event_loop:EventLoop<()>,mut app:App) {
    let event_handle = move |event: Event<()>,_event_loop: &EventLoopWindowTarget<()>,control_flow: &mut ControlFlow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {event,..} => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::Resized(new_size) => {
                        
                        let mut resize_events = app.world.get_resource_mut::<Events<WindowResized>>().unwrap();
                        resize_events.send(WindowResized {
                            width: new_size.width as f32,
                            height: new_size.height as f32,
                        });
                    }
                    _ => {}
                }
            },
            Event::MainEventsCleared => {
                app.update();
            }
            _ => {}
        }
    };

    event_loop.run( event_handle);
}