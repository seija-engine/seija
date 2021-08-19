mod window;
use seija_app::{IModule,App};
use seija_core::window::{AppWindow, WindowConfig};
use window::WinitWindow;
use winit::{event::{Event}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}};

#[derive(Default)]
pub struct WinitModule(WindowConfig);

impl IModule for WinitModule {
    fn init(&mut self,app:&mut App) {
        let (winit_window,event_loop) = WinitWindow::from_config(&self.0);
        let app_window = AppWindow::new(winit_window);
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
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
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