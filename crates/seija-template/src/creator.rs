use std::{collections::HashMap, sync::Arc};
use seija_app::ecs::{system::{CommandQueue, Insert}, world::World, prelude::Entity};
use seija_transform::Transform;
use smol_str::SmolStr;
use seija_core::{anyhow::{Result,anyhow}, math::{Vec3, Quat}};
use crate::TComponent;

pub type FromTComponentFunc = fn(world:&mut World,entity:Entity,&TComponent,queue:&mut CommandQueue) -> Result<()>;

pub struct TComponentManager {
    caches:Option<HashMap<SmolStr,FromTComponentFunc>>,
   pub creator:TComponentCreator
}

impl TComponentManager {
    pub fn new() -> Self {
        TComponentManager {
            caches:Some(HashMap::default()),
            creator:Default::default()
        }
    }

    pub fn start(&mut self) {
       if let Some(values) = self.caches.take() {
         self.creator = TComponentCreator { values:Arc::new(values) }
       }
    }
}

#[derive(Default)]
pub struct TComponentCreator {
    values:Arc<HashMap<SmolStr,FromTComponentFunc>>
}

impl TComponentManager {
    pub fn add(&mut self,name:&str,value:FromTComponentFunc) {
        if let Some(creators) = self.caches.as_mut() {
            creators.insert(name.into(), value);
        }
    }
}

impl TComponentCreator {
    pub fn create<'w,'s,'a>(&self,t_component:&TComponent,world:&mut World,queue:&mut CommandQueue,entity:Entity) -> Result<()> {
        if let Some(f) = self.values.get(&t_component.typ) {
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