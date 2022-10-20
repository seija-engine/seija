use bevy_ecs::prelude::Component;
use seija_asset::Handle;
use crate::material::Material;

#[derive(Default,Component)]
pub struct PostEffectStack {
   pub material_list:Vec<Handle<Material>>,
}

impl PostEffectStack {
    pub fn add(&mut self,material:Handle<Material>) {
        self.material_list.push(material)
    }
}