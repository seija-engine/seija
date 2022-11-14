use bevy_ecs::prelude::Component;
use seija_asset::Handle;

use crate::material::Material;


pub struct EffectItem {
    pub material:Handle<Material>,
    order:u32  
}

#[derive(Component,Default)]
pub struct PostEffectStack {
    pub items:Vec<EffectItem>
}

impl PostEffectStack {
    pub fn add_item(&mut self,material:Handle<Material>,order:u32) {
        self.items.push(EffectItem { material,order});
    }
}
