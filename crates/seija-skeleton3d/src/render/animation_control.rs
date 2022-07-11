use bevy_ecs::prelude::Component;
use seija_asset::{Handle, Assets};
use seija_transform::TransformMatrix;


use crate::{Skeleton, AnimationSet, jobs::{SamplingJob, LocalToModelJob}};

use super::runtime_skeleton::RuntimeSkeleton;

#[derive(Debug)]
pub enum AnimationError {
    NotFoundAnimSet,
    NotFoundAnim,
    NotFoundSkeleton
}
#[derive(Component)]
pub struct AnimationControl {
    skeleton:Handle<Skeleton>,
    animation_set:Handle<AnimationSet>,
    runtime_skeleton:Handle<RuntimeSkeleton>,

    pub(crate) anim_index:usize,
    pub(crate) last_anim_index:i32,
    pub(crate) ratio:f32,
    pub(crate) play:bool,
    pub(crate) is_loop:bool,

    pub(crate) sample_job:SamplingJob
}

impl AnimationControl {
    pub fn new(count:usize,skeleton:Handle<Skeleton>,animation_set:Handle<AnimationSet>,rk_assets:&mut Assets<RuntimeSkeleton>) -> Self {
        let runtime_skeleton = rk_assets.add(RuntimeSkeleton::new(count));
        AnimationControl {
            skeleton,
            animation_set,
            runtime_skeleton,
            anim_index:0,
            last_anim_index:-1 as i32,
            ratio:0f32,
            play:false,
            is_loop:true,
            sample_job:SamplingJob::default()
        }
    }

    pub fn get_runtime_skeleton(&self) -> &Handle<RuntimeSkeleton> {
        &self.runtime_skeleton
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

    pub fn process(&mut self,anims:&Assets<AnimationSet>,skeletons:&Assets<Skeleton>,dt:f32,rt_skeleton:&mut RuntimeSkeleton) -> Result<(),AnimationError> {
        if self.play == false { return Ok(()) }
        let anim_set = anims.get(&self.animation_set.id).ok_or(AnimationError::NotFoundAnimSet)?;
        let anim = anim_set.get_index(self.anim_index).ok_or(AnimationError::NotFoundAnim)?;
        if self.last_anim_index != self.anim_index as i32 {
            self.sample_job.init(anim);
            self.last_anim_index = self.anim_index as i32;
        }
        let skeleton = skeletons.get(&self.skeleton.id).ok_or(AnimationError::NotFoundSkeleton)?;
        
        self.sample_job.run(anim, &mut rt_skeleton.values, self.ratio);
        let ltw = LocalToModelJob::new(skeleton);
        ltw.run(&rt_skeleton.values, &mut rt_skeleton.mat4s);

        self.ratio += dt / anim.duration;
        //println!("{:?}",rt_skeleton.mat4s);
        if self.ratio > 1f32 {
            if self.is_loop {
                self.play_index(self.anim_index);
            } else {
                self.stop();
            }
            
        }
        Ok(())
    }
}