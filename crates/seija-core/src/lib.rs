use bevy_ecs::{prelude::*, schedule::RunOnce};
use bevy_ecs::schedule::{StageLabel};
use seija_app::{IModule,App};
use time::{Time};
pub mod time;
pub mod window;

#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel )]
pub enum CoreStage {
    Startup,
    First,
    PreUpdate,
    Update,
    PostUpdate,
    Last,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StartupStage {
    PreStartup,
    Startup,
    PostStartup,
}

#[derive(Default)]
pub struct TestInt(pub i32);
pub struct CoreModule;

impl IModule for CoreModule {
    fn init(&mut self, app:&mut App) {
        self.add_core_stages(app);
        app.init_resource::<Time>();
        app.init_resource::<TestInt>();
        app.add_system(CoreStage::First, time::time_system.exclusive_system());
    }
}

impl CoreModule {
    fn add_core_stages(&mut self,app :&mut App) {
        app.schedule.add_stage(CoreStage::First, SystemStage::parallel());
        
        let mut startup = Schedule::default().with_run_criteria(RunOnce::default());
        startup.add_stage(StartupStage::PreStartup, SystemStage::parallel());
        startup.add_stage(StartupStage::Startup, SystemStage::parallel());
        startup.add_stage(StartupStage::PostStartup, SystemStage::parallel());
        app.schedule.add_stage(CoreStage::Startup, startup);
        app.schedule.add_stage(CoreStage::PreUpdate, SystemStage::parallel());
        app.schedule.add_stage(CoreStage::Update, SystemStage::parallel());
        app.schedule.add_stage(CoreStage::PostUpdate, SystemStage::parallel());
    }
}