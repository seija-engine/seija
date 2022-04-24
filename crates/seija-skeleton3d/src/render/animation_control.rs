use seija_asset::{Handle, Assets};


use crate::{Skeleton, AnimationSet, jobs::SamplingJob};

use super::runtime_skeleton::RuntimeSkeleton;


pub struct AnimationControl {
    skeleton:Handle<Skeleton>,
    animation_set:Handle<AnimationSet>,
    runtime_skeleton:Handle<RuntimeSkeleton>,

    pub(crate) anim_index:usize,
    pub(crate) last_anim_index:i32,
    pub(crate) ratio:f32,
    pub(crate) play:bool,

    pub(crate) sample_job:SamplingJob
}

impl AnimationControl {
    pub fn new(skeleton:Handle<Skeleton>,animation_set:Handle<AnimationSet>,rk_assets:&mut Assets<RuntimeSkeleton>) -> Self {
        let runtime_skeleton = rk_assets.add(RuntimeSkeleton::default());
        AnimationControl {
            skeleton,
            animation_set,
            runtime_skeleton,
            anim_index:0,
            last_anim_index:-1 as i32,
            ratio:0f32,
            play:false,
            sample_job:SamplingJob::default()
        }
    }

    pub fn play_index(&mut self,idx:usize) {
        self.play = true;
        self.ratio = 0f32;
        self.anim_index = idx;

    }

    pub fn stop(&mut self) {
        self.play = false;
        self.ratio = 0f32;
        self.last_anim_index = -1;
    }
}