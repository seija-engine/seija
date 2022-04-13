use crate::animation::Animation;

use super::raw_animation::RawAnimation;

pub struct AnimationBuilder;

impl AnimationBuilder {
    pub fn build(raw_animation:&RawAnimation) -> Animation {
        let mut animation = Animation::default();
        animation.name = raw_animation.name.clone();
        animation.duration = raw_animation.duration;
        animation.num_tracks = raw_animation.num_tracks();
        animation
    }
}