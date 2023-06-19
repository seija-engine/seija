use bevy_ecs::{schedule::{SystemLabel}};
use events::HierarchyEvent;
use seija_app::IModule;
use seija_app::App;
pub mod events;
pub mod hierarchy;
mod commands;
mod system;
mod transform;
mod events2;
pub mod ffi;

use seija_core::{CoreStage, StartupStage, AddCore};
pub use system::update_transform_system;
pub use  transform::{Transform,TransformMatrix};
pub use  commands::{PushChildren,BuildChildren,IEntityChildren,DespawnRecursive};


#[derive(Debug, Hash, PartialEq, Eq, Clone,SystemLabel)]
pub enum TransformLabel {
    Propagate,
    ParentUpdate,
}

pub struct TransformModule;


impl IModule for TransformModule {
    fn init(&mut self,app:&mut App) {
        app.add_event::<HierarchyEvent>();
        //app.add_system2(CoreStage::Startup, StartupStage::PostStartup,system::parent_update_system.label(TransformLabel::ParentUpdate));
        app.add_system2(CoreStage::Startup, StartupStage::PostStartup,update_transform_system);
        //app.add_system(CoreStage::PostUpdate,system::parent_update_system.label(TransformLabel::ParentUpdate));
    
        app.add_system(CoreStage::PostUpdate,update_transform_system);
    }
}