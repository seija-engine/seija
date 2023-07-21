use active_system::{active_system,active_add_system};
use bevy_ecs::schedule::IntoSystemDescriptor;
use bevy_ecs::schedule::SystemLabel;
use events::HierarchyEvent;
use seija_app::IModule;
use seija_app::App;
pub mod events;
pub mod hierarchy;
mod active_system;
mod system;
mod transform;
pub mod ffi;
use seija_core::{CoreStage,AddCore};
use system::clear_delete_entity;
pub use system::update_transform_system;
pub use  transform::{Transform,TransformMatrix};


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
       // app.add_system2(CoreStage::Startup, StartupStage::PostStartup,update_transform_system);
        //app.add_system(CoreStage::PostUpdate,system::parent_update_system.label(TransformLabel::ParentUpdate));
    
        app.add_system(CoreStage::PostUpdate,update_transform_system);
        app.add_system(CoreStage::Update, active_add_system.at_end());
        app.add_system(CoreStage::Last, active_system);
        app.add_system(CoreStage::Last, clear_delete_entity);
    }
}