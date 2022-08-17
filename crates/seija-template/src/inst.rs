use seija_app::ecs::prelude::Component;
use seija_core::bevy_ecs::system::Command;
use seija_core::bevy_ecs::{entity::Entity,world::World};
use seija_core::anyhow::{Result};
use seija_transform::PushChildren;
use smallvec::SmallVec;
use downcast_rs::Downcast;

use crate::creator::TComponentCreator;
use crate::{Template, TEntity};

pub fn instance_template_sync(world:&mut World,template:&Template) -> Result<Entity> {
    create_entity_sync(world,&template.entity)
}

fn create_entity_sync(world:&mut World,t_entity:&TEntity) -> Result<Entity> {
    let mut childrens:SmallVec<[Entity;8]> = SmallVec::default();
    for child in t_entity.children.iter() {
        childrens.push(create_entity_sync(world,&child)?);
    }
    let mut entity = world.spawn();
    let eid = entity.id();
    if let Some(info) = t_entity.not_default_info() {
        entity.insert(info);
    }
    let creators = world.get_resource::<TComponentCreator>().unwrap();
    for component in t_entity.components.iter() {
        if let Some(comp) = creators.create(component.typ.as_str(), &component) {
            let v = comp?;
            
            //entity.insert(v);
        }
        //log::error!("{:?}",component);
    }
   
    let mut push_children = PushChildren::new(eid);
    push_children.children = childrens;
    
    push_children.write(world);
    

    Ok(eid)
}
