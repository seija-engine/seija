use std::collections::HashMap;

use seija_app::ecs::{system::{CommandQueue, Command}};
use seija_asset::{AssetServer, HandleUntyped, Assets};
use seija_core::{bevy_ecs::{entity::Entity,world::{World}}, info::EInfo};
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
                childrens.push(instance_entity_sync(world,server,&childen,mgr,queue,child_tmpl)?);
            },
            TEntityChildren::Template(xml_template) => {
               let handle_id = child_tmpl.get(xml_template.res.as_str()).ok_or(TemplateError::NotFoundChild(xml_template.res.clone()))?.id;
               let templates = world.get_resource::<Assets<Template>>().unwrap();
               let template = templates.get(&handle_id).ok_or(TemplateError::NotFoundChild(xml_template.res.clone()))?.clone();
               let entity = instance_template_sync(world,&template)?;

               let template_entity = world.spawn_empty();
               let template_entity_id = template_entity.id();
               for component in xml_template.components.iter() {
                    mgr.create(component, server,queue,template_entity_id)?
               }
               childrens.push(template_entity_id);
               PushChildren {children:SmallVec::from_slice(&[entity]) ,parent:template_entity_id}.write(world);
            }
        }
    }
    let mut entity_mut = world.spawn_empty();
    let eid = entity_mut.id();
    let info = create_einfo(tentity);
    entity_mut.insert(info);
    for component in tentity.components.iter() {
        mgr.create(component, server,queue,eid)?
    }
   
    PushChildren {children:childrens,parent:eid}.write(world);   
   Ok(eid)
}

fn create_einfo(entity:&TEntity) -> EInfo {
    let mut info = EInfo::default();
    if let Some(name) = entity.name.as_ref() {
        info.name = Some(name.clone())
    }
    info.layer = entity.layer;
    if let Some(tag) = entity.tag.as_ref() {
        info.tag = Some(tag.clone())
    }
    info
}