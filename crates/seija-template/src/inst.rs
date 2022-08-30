use std::sync::Arc;

use seija_app::ecs::{system::{CommandQueue, Command}};
use seija_asset::AssetServer;
use seija_core::bevy_ecs::{entity::Entity,world::{World,Mut}};
use seija_core::anyhow::{Result};
use seija_transform::{ PushChildren};
use smallvec::SmallVec;

use crate::{TEntity, TComponentManager};


pub fn instance_template_sync(world:&mut World,tentity:Arc<TEntity>) -> Result<Entity> {
    world.resource_scope(|w:&mut World,mgr:Mut<TComponentManager>| {
        let mut queue = CommandQueue::default();
        let server = w.get_resource::<AssetServer>().unwrap().clone();
        let ret = instance_entity_sync(w,&server,&tentity,&mgr,&mut queue);
        queue.apply(w);

        ret
    })
    
}

fn instance_entity_sync(world:&mut World,server:&AssetServer,t_entity:&TEntity,mgr:&TComponentManager,queue:&mut CommandQueue) -> Result<Entity> {

    let mut childrens:SmallVec<[Entity;8]> = SmallVec::new();
    for child in t_entity.children.iter() {
        childrens.push(instance_entity_sync(world,server,child,mgr,queue)?);
    }
    let entity_mut = world.spawn();
    let eid = entity_mut.id();
    
    for component in t_entity.components.iter() {
        mgr.create(component, server,queue,eid)?;
        
    }
   
    PushChildren {children:childrens,parent:eid}.write(world);   
   Ok(eid)
}