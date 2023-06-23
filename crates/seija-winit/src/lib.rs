mod window;
pub mod event;
pub mod ffi;
use event::{WindowCreated, WindowResized, conv_mouse_input};
use seija_app::{IModule,App};
use seija_core::{ window::{AppWindow, WindowConfig},AddCore, math::Vec2};
use seija_core::bevy_ecs::{event::{Events}};
use window::WinitWindow;
use seija_input::{event::{KeyboardInput as IKeyboardInput, MouseInput, MouseWheelInput}, Input};
use winit::{event::{Event,WindowEvent,  MouseScrollDelta}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}};

use crate::event::conv_keyboard_input;

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
        let t = app.last_call.elapsed();
        if t > app.frame_duration {
          *control_flow = ControlFlow::Poll;
        } else {
            let next = app.last_call + app.frame_duration;   
            *control_flow = ControlFlow::WaitUntil(next);
        }
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
                    },
                    WindowEvent::KeyboardInput { device_id:_, input, is_synthetic:_ } => {
                        if let Some(mut events) = app.world.get_resource_mut::<Events<IKeyboardInput>>() {
                            events.send(conv_keyboard_input(input));
                        }
                    }
                    WindowEvent::MouseInput {  state, button, .. } => {
                        if let Some(mut events) = app.world.get_resource_mut::<Events<MouseInput>>() {
                            let mouse_input = conv_mouse_input(state, button);
                            events.send(mouse_input);
                        }
                    }
                    WindowEvent::CursorMoved { position,.. } => {
                        if let Some(mut input) = app.world.get_resource_mut::<Input>() {
                            input.is_mouse_move = true;
                            let new_pos = Vec2::new(position.x as f32, position.y as f32);
                            input.mouse_position = new_pos;
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        if let Some(mut events) = app.world.get_resource_mut::<Events<MouseWheelInput>>() {
                            let mut mouse_wheel = MouseWheelInput::default();
                             match delta {
                                MouseScrollDelta::LineDelta(x,y) => {
                                    mouse_wheel.delta.x = x;
                                    mouse_wheel.delta.y = y;
                                },
                                MouseScrollDelta::PixelDelta(v) => {
                                    mouse_wheel.delta.x = v.x as f32;
                                    mouse_wheel.delta.y = v.y as f32;
                                }
                            }
                            events.send(mouse_wheel);
                        }
                    }
                
                    _ => {}
                }
            },
            Event::MainEventsCleared => {
                if *control_flow == ControlFlow::Poll {
                    app.update();
                }
            }
            _ => {}
        }
    };

    event_loop.run( event_handle);
}