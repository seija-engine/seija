use bevy_ecs::component::Component;
use bevy_ecs::{prelude::*, schedule::RunOnce};
use bevy_ecs::schedule::{StageLabel};
use event::Events;
use seija_app::{IModule,App};
use time::{Time};
use std::fmt::Debug;
use std::sync::atomic::{AtomicU64,Ordering};

pub mod bytes;
pub mod time;
pub mod window;
pub mod event;
pub mod type_uuid;
pub use type_uuid::{TypeUuid,TypeUuidDynamic};
pub use uuid;
pub use bevy_ecs;

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


pub struct CoreModule;

impl IModule for CoreModule {
    fn init(&mut self, app:&mut App) {
        self.add_core_stages(app);
        app.init_resource::<Time>();
        app.add_system(CoreStage::First, time::time_system.exclusive_system());
        app.add_system(CoreStage::Last, World::clear_trackers.exclusive_system());
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
        app.schedule.add_stage(CoreStage::Last, SystemStage::parallel());
    }
}



pub trait AddCore {
    fn add_event<T:Component>(&mut self);
}

impl AddCore for App {
    fn add_event<T:Component>(&mut self) {
        self.add_resource(Events::<T>::default());
        self.add_system(CoreStage::First, Events::<T>::update_system.system());
    }
}



pub struct IDGenU64 {
    atom:AtomicU64,
}

impl IDGenU64 {
    pub fn new() -> IDGenU64 {
        IDGenU64 { atom:AtomicU64::new(0) }
    }

    pub fn next(&self) -> u64 {
        self.atom.fetch_add(1, Ordering::Relaxed)
    }
}


pub trait LogOption<T> {
    fn log_err (self,msg:&str) -> Option<T>;
    fn log_warn(self,msg:&str) -> Option<T>;
}

impl<T> LogOption<T> for Option<T> {
    fn log_err (self,msg:&str) -> Option<T> {
        match self {
            Some(t) => Some(t),
            None => {
                log::error!("{}",msg);
                None
            }
        }
    }

    fn log_warn(self,msg:&str) -> Option<T> {
        match self {
            Some(t) => Some(t),
            None => {
                log::warn!("{}",msg);
                None
            }
        }
    }
}

pub trait LogResult<T,E:Debug> {
    fn log_err (self) -> Result<T,E>;
    fn log_warn(self) -> Result<T,E>;
}

impl<T,E:Debug> LogResult<T,E> for Result<T,E> {
    fn log_err(self) -> Result<T,E> {
       match self {
           Ok(v) => Ok(v),
           Err(err) => {
               log::error!("{:?}",err);
               Err(err)
           }
       }
    }

    fn log_warn(self) -> Result<T,E> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => {
                log::warn!("{:?}",err);
                Err(err)
            }
        }
    }
}