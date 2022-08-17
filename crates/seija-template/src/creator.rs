use std::{collections::HashMap, any::Any};
use downcast_rs::Downcast;
use seija_transform::Transform;
use smol_str::SmolStr;
use seija_core::{anyhow::{Result,anyhow}, math::{Vec3, Quat}};

use crate::TComponent;

pub trait IFromTComponent : Send + Sync + 'static {
    
    fn from(&self,component:&TComponent) -> Result<Box<dyn Downcast>>;
}


#[derive(Default)]
pub struct TComponentCreator {
    creators:HashMap<SmolStr,Box<dyn IFromTComponent>>
}

impl TComponentCreator {
    pub fn add<T:IFromTComponent + 'static>(&mut self,name:&str,value:T) {
        self.creators.insert(name.into(), Box::new(value));
    }

    pub fn create(&self,name:&str,t_component:&TComponent) -> Option<Result<Box<dyn Downcast>>> {
        if let Some(creator) = self.creators.get(name) {
            let box_t = creator.from(&t_component);
            
            return Some(box_t);
        }

        None
    }
}
#[derive(Default)]
pub struct TransformCreator;

impl IFromTComponent for TransformCreator {
    fn from(&self,component:&TComponent) -> Result<Box<dyn Downcast>> {
        let p = component.read_v3("position").unwrap_or(Vec3::ZERO);
        let r = component.read_v3("rotation").unwrap_or(Vec3::ZERO);
        let s = component.read_v3("scale").unwrap_or(Vec3::ONE);
        let t = Transform::new(p, Quat::from_euler(Default::default(), r.x, r.y, r.z), s);
        Ok(Box::new(t))
    }
}