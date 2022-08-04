use event::{KeyboardInput, KeyboardInputState};

use seija_app::{IModule, App, ecs::{world::World, system::ResMut, prelude::EventReader}};
use seija_core::{AddCore, CoreStage};

pub mod event;
mod input;
pub use input::{Input};
pub struct InputModule;

impl IModule for InputModule {
    fn init(&mut self,app:&mut App) {
        app.init_resource::<Input>();
        app.add_event::<event::KeyboardInput>();
        
        app.add_system(CoreStage::PreUpdate, input_system)
    }

    fn start(&self,_world:&mut World) {
        
    }
}

fn input_system(mut input:ResMut<Input>,mut key_inputs:EventReader<KeyboardInput>) {
    input.clear();
    for key in key_inputs.iter() {
        match key.state {
            KeyboardInputState::Pressed => {
                if !input.key_pressing.contains(&key.key_code) {
                    input.frame_keydown.insert(key.key_code);
                    input.key_pressing.insert(key.key_code);
                }
            },
            KeyboardInputState::Released => {
                input.frame_keyup.insert(key.key_code);
                input.key_pressing.remove(&key.key_code);
            }
        }
    }

   
}