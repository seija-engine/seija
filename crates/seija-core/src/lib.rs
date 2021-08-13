use bevy_ecs::prelude::*;
use bevy_ecs::schedule::StageLabel;
use seija_app::{IModule,App};
use time::{Time,};
pub mod time;

#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel )]
pub enum CoreStage {
    Startup,
    First,
    PreUpdate,
    Update,
    PostUpdate,
    Last,
}

pub struct CoreModule;

impl IModule for CoreModule {
    fn init(&mut self, app:&mut App) {
       
        app.add_resource::<Time>();
        app.add_system_to_stage(CoreStage::First, time::time_system.exclusive_system());
    }
}