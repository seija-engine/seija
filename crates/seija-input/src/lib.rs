use event::{KeyboardInput, InputState, MouseInput, MouseWheelInput};

use seija_app::{IModule, App, ecs::{world::World, system::ResMut, prelude::EventReader}};
use seija_core::{AddCore, CoreStage};
pub mod keycode;
pub mod event;
mod input;
pub use input::{Input};
pub mod ffi;
pub struct InputModule;

impl IModule for InputModule {
    fn init(&mut self,app:&mut App) {
        app.init_resource::<Input>();
        app.add_event::<event::KeyboardInput>();
        app.add_event::<event::MouseInput>();
        app.add_event::<event::MouseWheelInput>();
        
        app.add_system(CoreStage::PreUpdate, input_system);
        app.add_system(CoreStage::Last, clear_input);
    }

    fn start(&self,_world:&mut World) {
        
    }
}

fn input_system(mut input:ResMut<Input>,mut key_inputs:EventReader<KeyboardInput>,
                                        mut mouse_inputs:EventReader<MouseInput>,
                                        mut mouse_wheel_inputs:EventReader<MouseWheelInput>) {
    for key in key_inputs.iter() {
        match key.state {
            InputState::Pressed => {
                if !input.key_pressing.contains(&key.key_code) {
                    input.frame_keydown.insert(key.key_code);
                    input.key_pressing.insert(key.key_code);
                }
            },
            InputState::Released => {
                input.frame_keyup.insert(key.key_code);
                input.key_pressing.remove(&key.key_code);
            }
        }
    }

    for mouse in mouse_inputs.iter() {
        match mouse.state {
            InputState::Pressed =>  {
                input.frame_mousedown.insert(mouse.button);
            },
            InputState::Released => {
                input.frame_mouseup.insert(mouse.button);
            }
        }
    }

    for mouse_wheel in mouse_wheel_inputs.iter() {
        input.frame_mouse_wheel = Some(mouse_wheel.delta);
    }
}

fn clear_input(mut input:ResMut<Input>) {
    input.clear();
}