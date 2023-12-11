use bevy_ecs::prelude::Component;

#[derive(Component,Debug,Clone)]
#[repr(C)]
pub struct Input {
    pub is_focus:bool,
    pub text:String
}