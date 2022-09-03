use bevy_ecs::{schedule::{SystemLabel, ParallelSystemDescriptorCoercion}};
use seija_app::IModule;
use seija_app::App;

pub mod hierarchy;
mod children_command;
mod system;
mod transform;

use seija_core::{CoreStage, StartupStage};
use system::update_transform_system;
pub use  transform::{Transform,TransformMatrix};
pub use  children_command::{PushChildren,BuildChildren};


#[derive(Debug, Hash, PartialEq, Eq, Clone,SystemLabel)]
pub enum TransformLabel {
    Propagate,
    ParentUpdate,
}

pub struct TransformModule;


impl IModule for TransformModule {
    fn init(&mut self,app:&mut App) {
        app.add_system2(CoreStage::Startup, StartupStage::PostStartup,
                       system::parent_update_system.label(TransformLabel::ParentUpdate));
        app.add_system2(CoreStage::Startup, StartupStage::PostStartup, 
                       update_transform_system.label(TransformLabel::Propagate).after(TransformLabel::ParentUpdate));
        app.add_system(CoreStage::PostUpdate, 
                           system::parent_update_system.label(TransformLabel::ParentUpdate));
        app.add_system(CoreStage::PostUpdate, 
                           update_transform_system.label(TransformLabel::Propagate).after(TransformLabel::ParentUpdate));
    }
}