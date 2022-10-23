use bevy_ecs::{entity::Entity, prelude::World};
use lite_clojure_eval::Variable;
use anyhow::{anyhow,Result};
use crate::{IUpdateNode, RenderContext};

#[derive(Default)]
pub struct UsePostStack {
    camera_id:Option<Entity>
}

impl IUpdateNode for UsePostStack {
    fn update_params(&mut self,params:Vec<Variable>) -> Result<()> {
        let camera_id = params[0].cast_int().ok_or(anyhow!("type cast error"))?;
        self.camera_id = Some(Entity::from_raw(camera_id as u32));
        
        Ok(())
    }

    fn update(&mut self, _world:&mut World, _ctx:&mut RenderContext) {
        
    }
}