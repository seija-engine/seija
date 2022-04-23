use std::collections::HashMap;

use crate::Animation;


#[derive(Debug,Default)]
pub struct AnimationSet {
   animations:HashMap<String,Animation>
}

impl AnimationSet {
    pub fn add(&mut self,anim:Animation) {
        self.animations.insert(anim.name.clone(), anim);
    }
}
