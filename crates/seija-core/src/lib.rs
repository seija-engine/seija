use bevy_ecs::event::Events;
use bevy_ecs::{prelude::*};
use bevy_ecs::schedule::{StageLabel, ShouldRun};
use seija_app::{IModule,App};
use time::{Time};
use std::fmt::Debug;
use std::sync::atomic::{AtomicU64,Ordering, AtomicU32};
pub mod info;
pub mod bytes;
pub mod time;
pub mod window;
pub mod type_uuid;
pub mod ffi;
pub use type_uuid::{TypeUuid,TypeUuidDynamic};
pub use uuid;
pub use bevy_ecs;
pub use glam as math;
pub use smol;
pub use anyhow;
pub use smol_str;

#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel )]
pub enum CoreStage {
    Startup,
    First,
    PreUpdate,
    Update,
    LateUpdate,
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
        app.add_system(CoreStage::First, time::time_system);
        app.add_system(CoreStage::Last, World::clear_trackers.at_end());
    }
}

impl CoreModule {
    fn add_core_stages(&mut self,app :&mut App) {
        app.schedule.add_stage(CoreStage::First, SystemStage::parallel());
        
        let mut startup = Schedule::default().with_run_criteria(ShouldRun::once);
        startup.add_stage(StartupStage::PreStartup, SystemStage::parallel());
        startup.add_stage(StartupStage::Startup, SystemStage::parallel());
        startup.add_stage(StartupStage::PostStartup, SystemStage::parallel());
        app.schedule.add_stage(CoreStage::Startup, startup);
        app.schedule.add_stage(CoreStage::PreUpdate, SystemStage::parallel());
        app.schedule.add_stage(CoreStage::Update, SystemStage::single_threaded());
        app.schedule.add_stage(CoreStage::LateUpdate, SystemStage::single_threaded());
        app.schedule.add_stage(CoreStage::PostUpdate, SystemStage::parallel());
        app.schedule.add_stage(CoreStage::Last, SystemStage::parallel());
    }
}



pub trait AddCore {
    fn add_event<T:Send + Sync + 'static>(&mut self);
}

impl AddCore for App {
    fn add_event<T:Send + Sync + 'static>(&mut self) {
        self.add_resource(Events::<T>::default());
        self.add_system(CoreStage::First, Events::<T>::update_system);
    }
}

macro_rules! idgen {
    ($name:ident,$type:ty,$type2:ty) => {
        #[derive(Debug)]
        pub struct $name {
            atom:$type
        }
        impl $name {
            pub fn new() -> $name {
                $name { atom:<$type>::default() }
            }
            pub fn next(&self) -> $type2 {
                self.atom.fetch_add(1, Ordering::SeqCst)
            }
        }
    }
}

idgen!(IDGenU32,AtomicU32,u32);
idgen!(IDGenU64,AtomicU64,u64);

pub trait OptionExt<T> {
    fn log_err (self,msg:&str) -> Option<T>;
    fn log_warn(self,msg:&str) -> Option<T>;
    fn get(self) -> anyhow::Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
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

    #[track_caller]
    fn get(self) -> anyhow::Result<T> {
        match self {
            Some(v) => Ok(v),
            None => {
               let caller = std::panic::Location::caller();
               return Err(anyhow::anyhow!("{} ",caller));
            }
        }
    }

}

pub trait ResultExt<T,E:Debug> {
    fn log_err (self) -> Result<T,E>;
    fn log_warn(self) -> Result<T,E>;
}

impl<T,E:Debug> ResultExt<T,E> for Result<T,E> {
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

#[derive(Debug,Clone,Default)]
pub struct Rect<T:Default> {
    pub x:T,
    pub y:T,
    pub width:T,
    pub height:T
}

#[derive(Component,Default,Clone)]
#[repr(C)]
pub struct FrameDirty {
    pub frame:u64,
    pub index:i32
}