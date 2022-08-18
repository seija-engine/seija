use std::{collections::HashMap};
use seija_app::ecs::system::{ EntityCommands};
use seija_asset::AssetServer;
use seija_transform::Transform;
use smol_str::SmolStr;
use seija_core::{anyhow::{Result,anyhow}, math::{Vec3, Quat}};

use crate::TComponent;

pub trait IFromTComponent : Send + Sync + 'static {
    
    fn from<'w,'s,'a>(&self,component:&TComponent,assets:&AssetServer,commands:&mut EntityCommands<'w,'s,'a>) -> Result<()>;
}


#[derive(Default)]
pub struct TComponentCreator {
    creators:HashMap<SmolStr,Box<dyn IFromTComponent>>
}

impl TComponentCreator {
    pub fn add<T:IFromTComponent + 'static>(&mut self,name:&str,value:T) {
        self.creators.insert(name.into(), Box::new(value));
    }

    pub fn create<'w,'s,'a>(&self,t_component:&TComponent,assets:&AssetServer,commands:&mut EntityCommands<'w,'s,'a>) -> Result<()> {
        if let Some(creator) = self.creators.get(&t_component.typ) {
            creator.from(&t_component,assets,commands)
        } else {
            Err(anyhow!(format!("not found {} creator",&t_component.typ)))
        }
    }
}
#[derive(Default)]
pub struct TransformCreator;

impl IFromTComponent for TransformCreator {
    fn from<'w,'s,'a>(&self,component:&TComponent,_:&AssetServer,commands:&mut EntityCommands<'w,'s,'a>) -> Result<()> {
        let p = component.read_v3("position").unwrap_or(Vec3::ZERO);
        let r = component.read_v3("rotation").unwrap_or(Vec3::ZERO);
        let s = component.read_v3("scale").unwrap_or(Vec3::ONE);
        let t = Transform::new(p, Quat::from_euler(Default::default(), r.x, r.y, r.z), s);
        commands.insert(t);
        Ok(())
    }
}