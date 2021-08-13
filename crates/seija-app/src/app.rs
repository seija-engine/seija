use bevy_ecs::{component::Component, prelude::FromWorld, schedule::{Schedule, Stage, StageLabel, SystemDescriptor}, world::World};

use crate::IModule;

pub struct App {
    world:World,
    schedule: Schedule,
    runner: Option<Box<dyn Fn(App)>>
}

impl App {
    pub fn new() -> App {
        App {
            world:World::default(),
            schedule:Schedule::default(),
            runner:None
        }
    }

    pub fn add_module<T:IModule>(&mut self,mut module:T) {
        module.init(self);
    }

    pub fn add_resource<T>(&mut self) where T:Default + Send + Sync  + 'static {
        self.world.insert_resource(T::default());
    }

    pub fn add_system_to_stage(&mut self,stage_label: impl StageLabel,system: impl Into<SystemDescriptor>) {
        self.schedule.add_system_to_stage(stage_label, system);
    }

    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }

    pub fn set_runner(&mut self, run_fn: impl Fn(App) + 'static) {
        self.runner = Some(Box::new(run_fn));
    }

    pub fn run(mut self) {
       if let Some(run_fn) = self.runner.take() {
           run_fn(self)
       } else {
           eprintln!("app runner is empty");
       }
    }
}