use seija_app::ecs::system::{CommandQueue, Commands};
use seija_asset::AssetServer;
use seija_core::bevy_ecs::{entity::Entity,world::World};
use seija_core::anyhow::{Result, anyhow};
use seija_transform::BuildChildren;

use crate::creator::TComponentCreator;
use crate::{Template, TEntity};

pub fn instance_template_sync(world:&mut World,template:&Template) -> Result<Entity> {
    let mut command_queue = CommandQueue::default();
    let mut commands = Commands::new(&mut command_queue, world);
    if let Some(creator) = world.get_resource::<TComponentCreator>() {
        let asset_server = world.get_resource::<AssetServer>().ok_or(anyhow!("not found asset server"))?;
        match create_entity_sync(creator,asset_server,&template.entity,&mut commands) {
            Ok(v) => {
                command_queue.apply(world);
                Ok(v)
            },
            Err(e) => { Err(e) },
        }
    } else {
        return Err(anyhow!("not found TComponentCreator"));
    }
   
}

fn create_entity_sync(creator:&TComponentCreator,assets:&AssetServer,t_entity:&TEntity,commands:&mut Commands) -> Result<Entity> {
    let mut childrens:Vec<Entity> = vec![];
    for child in t_entity.children.iter() {
        childrens.push(create_entity_sync(creator,assets,&child,commands)?);
    }

    let mut entity = commands.spawn();
    if let Some(info) = t_entity.not_default_info() {
        entity.insert(info);
    }
    for component in t_entity.components.iter() {
        creator.create(component, assets,&mut entity)?;
    }
    entity.add_children(&childrens);
    Ok(entity.id())
}
