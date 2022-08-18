use seija_app::ecs::system::{CommandQueue, Command};
use seija_core::bevy_ecs::{entity::Entity,world::{World,Mut}};
use seija_core::anyhow::{Result};
use seija_transform::{ PushChildren};
use smallvec::SmallVec;

use crate::creator::TComponentCreator;
use crate::{Template, TEntity};

pub fn instance_template_sync(world:&mut World,template:&Template) -> Result<Entity> {
    world.resource_scope(|w:&mut World,creator:Mut<TComponentCreator>| {
        let mut queue = CommandQueue::default();
        let ret = instance_entity_sync(w,&template.entity,&creator,&mut queue);
        queue.apply(w);
        ret
    })
    
}

fn instance_entity_sync(world:&mut World,t_entity:&TEntity,creator:&TComponentCreator,queue:&mut CommandQueue) -> Result<Entity> {
    println!("create entity:{:?}",t_entity);
    let mut childrens:SmallVec<[Entity;8]> = SmallVec::new();
    for child in t_entity.children.iter() {
        childrens.push(instance_entity_sync(world,child,creator,queue)?);
    }
    let entity_mut = world.spawn();
    let eid = entity_mut.id();
    
    for component in t_entity.components.iter() {
        creator.create(component, world,queue,eid)?;
    }

    PushChildren {children:childrens,parent:eid}.write(world);
    
    

   
   Ok(eid)
}