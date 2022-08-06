use bevy_ecs::{ schedule::{Schedule, Stage, StageLabel, IntoSystemDescriptor}, world::World};

use crate::IModule;

pub struct App {
    pub world:World,
    pub schedule: Schedule,
    runner: Option<Box<dyn FnOnce(App)>>,
    modules:Vec<Box<dyn IModule>>,
}

impl App {
    pub fn new() -> App {
        App {
            world:World::default(),
            schedule:Schedule::default(),
            runner:None,
            modules:vec![]
        }
    }

    pub fn add_module<T:IModule + 'static>(&mut self,mut module:T) {
        module.init(self);
        self.modules.push(Box::new(module));
    }

    pub fn start(&mut self) {
        for module in self.modules.iter() {
            module.start(&mut self.world)   
        }
    }

    pub fn init_resource<T>(&mut self) where T:Default + Send + Sync  + 'static {
        self.world.insert_resource(T::default());
    }

    pub fn add_resource<T>(&mut self,res:T) where T: Send + Sync  + 'static {
        self.world.insert_resource(res);
    }

   

    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }

    pub fn set_runner(&mut self, run_fn: impl FnOnce(App) + 'static) {
        self.runner = Some(Box::new(run_fn));
    }

    pub fn add_system<Params>(&mut self,stage_label: impl StageLabel,system:impl IntoSystemDescriptor<Params>) {
        self.schedule.add_system_to_stage(stage_label, system);
        
    }

    pub fn add_system2<Params>(&mut self,label: impl StageLabel,label2: impl StageLabel,system:impl IntoSystemDescriptor<Params>) {
        self.schedule.stage(label, |s: &mut Schedule| {
            s.add_system_to_stage(label2, system)
        });
    }


    pub fn run(mut self) {
       if let Some(run_fn) = self.runner.take() {
           run_fn(self)
       } else {
           eprintln!("app runner is empty");
       }
    }
}