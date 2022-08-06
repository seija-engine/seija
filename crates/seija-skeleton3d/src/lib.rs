pub mod offine;
pub mod jobs;
mod skeleton;
mod animation;
mod render;
mod animation_set;
mod skin;

pub use animation::{Animation,QuaternionKey,Float3Key};
use render::animation_system::update_skeleton_system;
use seija_app::{IModule, App};
use seija_asset::AddAsset;
use seija_core::{CoreStage};
pub use skeleton::{Skeleton};
pub use animation_set::{AnimationSet};
pub use render::{render_plugin::{create_skeleton_plugin},
                 runtime_skeleton::RuntimeSkeleton,animation_control::{AnimationControl}};
pub use skin::{Skin};


pub struct Skeleton3dModule;


impl IModule for Skeleton3dModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<Skeleton>();
        app.add_asset::<AnimationSet>();
        app.add_asset::<RuntimeSkeleton>();
        app.add_asset::<Skin>();

        app.add_system(CoreStage::PreUpdate, update_skeleton_system);
    }
}