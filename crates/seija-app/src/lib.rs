mod app;
pub use app::{App};
use bevy_ecs::prelude::World;
pub use bevy_ecs as ecs;
pub mod ffi;

pub trait IModule {
    fn init(&mut self,app:&mut App);
    fn start(&self,_world:&mut World) {}
}