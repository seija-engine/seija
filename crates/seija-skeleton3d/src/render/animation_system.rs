use bevy_ecs::prelude::{Entity, Res,ResMut, Query};
use seija_asset::{Assets};
use seija_core::{time::Time};
use crate::{RuntimeSkeleton, AnimationControl, AnimationSet, Skeleton};

pub fn update_skeleton_system(time:Res<Time>,
                              skeletons:Res<Assets<Skeleton>>,
                              mut rtskeletons:ResMut<Assets<RuntimeSkeleton>>,
                              anims:Res<Assets<AnimationSet>>,
                              mut query:Query<(Entity,&mut AnimationControl)>,) {
    for (_,mut animation_control) in query.iter_mut() {
        let hid = animation_control.get_runtime_skeleton().id;
        if let Some(runtime_skeleton)  = rtskeletons.get_mut(&hid) {
            if let Err(err) = animation_control.process(&anims,&skeletons,time.delta_seconds(),runtime_skeleton) {
                log::error!("{:?}",err);
            }
        }
        
    }
}

