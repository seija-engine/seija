use bevy_ecs::prelude::{Entity, World};
use wgpu::{BufferUsage, util::DeviceExt};

use crate::render::{AppRender, RenderContext};

use super::Material;


pub fn update_material(world:&mut World,app:&mut AppRender,render_ctx:&mut RenderContext) {
    let mut query = world.query::<(Entity,&mut Material)>();
   
    for (_,mut material) in query.iter_mut(world) {
        material.update(app);
    }
}