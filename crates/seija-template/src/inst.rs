use std::collections::HashMap;

use seija_app::ecs::{system::{CommandQueue, Command}};
use seija_asset::{AssetServer, HandleUntyped, Assets};
use seija_core::bevy_ecs::{entity::Entity,world::{World}};
use seija_core::anyhow::{Result};
use seija_transform::{ PushChildren};
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::{TEntity, TComponentManager, types::TEntityChildren, Template, errors::TemplateError};


pub fn instance_template_sync(world:&mut World,template:&Template) -> Result<Entity> {
    let mgr = world.get_resource::<TComponentManager>().unwrap().clone();
    let mut queue = CommandQueue::default();
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let child_tmpl = &template.inner.childrens;
    let ret = instance_entity_sync(world,&server,&template.inner.entity,&mgr,&mut queue,child_tmpl);
    queue.apply(world);

    ret
}

fn instance_entity_sync(world:&mut World,server:&AssetServer,tentity:&TEntity,mgr:&TComponentManager,queue:&mut CommandQueue,child_tmpl:&HashMap<SmolStr,HandleUntyped>) -> Result<Entity> {
    let mut childrens:SmallVec<[Entity;8]> = SmallVec::new();
    for child in tentity.children.iter() {
        match child {
            TEntityChildren::TEntity(childen) => {
                childrens.push(instance_entity_sync(world,server,childen,mgr,queue,child_tmpl)?);
            },
            TEntityChildren::Template(path) => {
                log::error!("inst template:{}",path.as_str());
               let handle_id = child_tmpl.get(path.as_str()).ok_or(TemplateError::NotFoundChild(path.clone()))?.id;
               let templates = world.get_resource::<Assets<Template>>().unwrap();
               let template = templates.get(&handle_id).ok_or(TemplateError::NotFoundChild(path.clone()))?.clone();
               let entity = instance_template_sync(world,&template)?;
               childrens.push(entity);
            }
        }
    }
    let entity_mut = world.spawn();
    let eid = entity_mut.id();
    
    for component in tentity.components.iter() {
        mgr.create(component, server,queue,eid)?;
        
    }
   
    PushChildren {children:childrens,parent:eid}.write(world);   
   Ok(eid)
}