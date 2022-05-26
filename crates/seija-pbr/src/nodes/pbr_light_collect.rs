use bevy_ecs::prelude::World;
use glam::Vec3;
use seija_render::{graph::{INode, nodes::UBOArrayCollect}, RenderContext, resource::RenderResourceId, UBONameIndex, UniformBuffer};
use seija_transform::Transform;

use crate::lights::{PBRLight, PBRLightType};

use super::pbr_light_backend::PBRLightBackend;

pub struct PBRLightCollect {
    array_collect:UBOArrayCollect<PBRLightBackend,PBRLight> 
}

impl PBRLightCollect {
    pub fn new(ubo_name:String) -> Self {
        let array_collect = UBOArrayCollect::new(ubo_name, 64);
        PBRLightCollect { array_collect }
    }
}

fn set_pbr_light(backend:&PBRLightBackend,index:usize,light:&PBRLight,buffer:&mut UniformBuffer,t:&Transform) {
    let dir = t.global().rotation * (-Vec3::Z);
    
    backend.set_ambile_color(buffer, Vec3::ONE);
    backend.set_lights_position(buffer,index,t.global().position);
    backend.set_lights_type(buffer, index, light.get_type().type_id() as i32);
    backend.set_lights_direction(buffer, index, dir.normalize());
    backend.set_lights_color(buffer, index, light.color);
    backend.set_lights_intensity(buffer, index, light.get_luminous_intensity());
    match light.get_type() {
        PBRLightType::Point => {
            backend.set_lights_falloff(buffer, index, light.get_falloff());
        },
        PBRLightType::Spot => {
            backend.set_lights_falloff(buffer, index, light.get_falloff());
            let scale_offset = light.get_scale_offset();
            backend.set_lights_spot_scale(buffer, index,scale_offset.x);
            backend.set_lights_spot_offset(buffer, index, scale_offset.y);
        },
        _ => {}
    }
}

impl INode for PBRLightCollect {
    fn init(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        self.array_collect.init(ctx);
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
        self.array_collect.update(world, ctx, set_pbr_light);
    }
}