use std::collections::HashMap;
use smol_str::SmolStr;
use seija_core::{anyhow::{Result}, info::EInfo, math::Vec3};

use crate::read_tmpl_entity;
use seija_core::{TypeUuid,uuid::Uuid};

#[derive(Default,Debug,TypeUuid)]
#[uuid = "92a98d82-b2f8-4618-8ca7-4d4bd93eb824"]
pub struct Template {
   pub entity:TEntity
}

impl Template {
    pub fn from_str(xml_string:&str) -> Result<Template> {
        let entity = read_tmpl_entity(xml_string)?;
        Ok(Template {
            entity
        })
    }
}

#[derive(Default,Debug)]
pub struct TEntity {
    pub name:Option<SmolStr>,
    pub layer:u32,
    pub tag:Option<SmolStr>,
    pub components:Vec<TComponent>,
    pub children:Vec<TEntity>
}

impl TEntity {
    pub fn not_default_info(&self) -> Option<EInfo> {
        if self.name.is_some() || self.layer > 0 || self.tag.is_some() {
            return Some(EInfo {
                name:self.name.clone(),
                layer:self.layer,
                tag:self.tag.clone()
            });
        }
        None
    }
}

#[derive(Default,Debug)]
pub struct TComponent {
    pub typ:SmolStr,
    pub attrs:HashMap<SmolStr,SmolStr> 
}

impl TComponent {
    pub fn new(typ:SmolStr) -> Self {
        TComponent { typ, attrs:HashMap::default() }
    }
    
    pub fn read_float(&self,name:&str,default:f32) -> f32 {
        self.attrs.get(name).and_then(|v| v.parse().ok()).unwrap_or(default)
    }

    pub fn read_v3(&self,name:&str) -> Option<Vec3> {
        if let Some(str) = self.attrs.get(name) {
            let mut arr = str.split(',');
            let x:f32 = arr.next()?.parse().ok()?;
            let y:f32 = arr.next()?.parse().ok()?;
            let z:f32 = arr.next()?.parse().ok()?;
            return Some(Vec3::new(x, y, z));
        }
        None
    }
}

pub trait FormTComponent<T> {
    fn from(&self,attrs:&HashMap<SmolStr,SmolStr>) -> Result<T>;
}