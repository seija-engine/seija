use std::collections::HashMap;
use seija_core::{TypeUuid,uuid::{Uuid}};
use crate::Animation;


#[derive(Debug,Default,TypeUuid)]
#[uuid = "2cb0848b-bd8b-43ed-8574-f7dab4073fb7"]
pub struct AnimationSet {
   animations:Vec<Animation>,
   names:HashMap<String,usize>
}

impl AnimationSet {
    pub fn add(&mut self,anim:Animation) {
        self.names.insert(anim.name.clone(), self.animations.len());
        self.animations.push(anim);
    }
}