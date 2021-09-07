use bevy_ecs::prelude::IntoSystem;
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;
use bevy_ecs::schedule::SystemLabel;
use seija_app::IModule;
use seija_app::App;
use seija_core::CoreStage;
use seija_core::StartupStage;

pub mod hierarchy;
mod parent;
mod transform;

pub use  transform::Transform;


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum TransformLabel {
    Propagate,
    ParentUpdate,
}

pub struct TransformModule;

impl IModule for TransformModule {
    fn init(&mut self,app:&mut App) {
        app.add_system2(CoreStage::Startup, StartupStage::PostStartup,
                       parent::parent_update_system.system().label(TransformLabel::ParentUpdate));
        app.add_system2(CoreStage::Startup, StartupStage::PostStartup, 
                       transform::update_transform_system.system().label(TransformLabel::Propagate).after(TransformLabel::ParentUpdate));
        app.add_system(CoreStage::PreUpdate, 
                           parent::parent_update_system.system().label(TransformLabel::ParentUpdate));
        app.add_system(CoreStage::PreUpdate, 
                           transform::update_transform_system.system().label(TransformLabel::Propagate).after(TransformLabel::ParentUpdate));
    }
}