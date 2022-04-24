pub mod offine;
pub mod jobs;
mod skeleton;
mod animation;
mod render;
mod animation_set;

pub use animation::{Animation,QuaternionKey,Float3Key};
use seija_app::{IModule, App};
use seija_asset::AddAsset;
pub use skeleton::{Skeleton};
pub use animation_set::{AnimationSet};
pub use render::{runtime_skeleton::RuntimeSkeleton,animation_control::{AnimationControl}};

pub struct Skeleton3dModule;

impl IModule for Skeleton3dModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<Skeleton>();
        app.add_asset::<AnimationSet>();
        app.add_asset::<RuntimeSkeleton>();
    }
}