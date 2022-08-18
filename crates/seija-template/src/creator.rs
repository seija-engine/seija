use std::{collections::HashMap};
use seija_app::ecs::{system::{CommandQueue, Insert}, world::World, prelude::Entity};
use seija_transform::Transform;
use smol_str::SmolStr;
use seija_core::{anyhow::{Result,anyhow}, math::{Vec3, Quat}};
use crate::TComponent;

pub type FromTComponentFunc = fn(world:&mut World,entity:Entity,&TComponent,queue:&mut CommandQueue) -> Result<()>;

#[derive(Default)]
pub struct TComponentCreator {
    creators:HashMap<SmolStr,FromTComponentFunc>
}

impl TComponentCreator {
    pub fn add(&mut self,name:&str,value:FromTComponentFunc) {
        self.creators.insert(name.into(), value);
    }

    pub fn create<'w,'s,'a>(&self,t_component:&TComponent,world:&mut World,queue:&mut CommandQueue,entity:Entity) -> Result<()> {
        if let Some(f) = self.creators.get(&t_component.typ) {
            f(world,entity,t_component,queue)
        } else {
            Err(anyhow!(format!("not found {} creator",&t_component.typ)))
        }
    }
}

pub(crate) fn tcomponent_transform(_:&mut World,entity:Entity,component:&TComponent,queue:&mut CommandQueue) -> Result<()> {
    let p = component.read_v3("position").unwrap_or(Vec3::ZERO);
    let r = component.read_v3("rotation").unwrap_or(Vec3::ZERO);
    let s = component.read_v3("scale").unwrap_or(Vec3::ONE);
    let t = Transform::new(p, Quat::from_euler(Default::default(), r.y.to_radians(), r.x.to_radians(), r.z.to_radians()), s);
    queue.push(Insert {entity,component:t } );
    Ok(())
}