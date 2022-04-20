use crate::{ImportData};

pub fn load_animation(gltf:&ImportData) {
    for skin in gltf.0.skins() {
        
    }
    for anim in gltf.0.animations() {
        for channel in  anim.channels() {
            
        }
        for sampler in anim.samplers() {
            
        }
    }
}