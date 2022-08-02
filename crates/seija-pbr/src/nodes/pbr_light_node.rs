use bevy_ecs::prelude::World;
use glam::Vec3;
use lite_clojure_eval::Variable;
use seija_render::{IUpdateNode, RenderContext, UniformBuffer, UBOArrayCollect};
use seija_transform::Transform;
use anyhow::{Result,anyhow};

use crate::lights::{PBRLight, PBRLightType};

use super::pbr_light_backend::PBRLightBackend;

#[derive(Default)]
pub struct PBRLightNode {
    pub ubo_name:String,
    array_collect:Option<UBOArrayCollect<PBRLightBackend,PBRLight>> 
}


fn set_pbr_light(backend:&PBRLightBackend,index:usize,light:&PBRLight,buffer:&mut UniformBuffer,t:&Transform) {
    let dir = t.global().rotation * Vec3::Z;
    log::debug!("set_pbr_light dir:{:?}",dir.normalize());
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

impl IUpdateNode for PBRLightNode {
    fn update_params(&mut self,params:Vec<Variable>) -> Result<()> {
        if let Some(string) = params.get(0).and_then(Variable::cast_string) {
            self.ubo_name = string.borrow().clone();
        }
        Ok(())
    }

    fn init(&mut self,_:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let mut array_collect = UBOArrayCollect::new(self.ubo_name.clone(), 10);
        array_collect.init(ctx);
        self.array_collect = Some(array_collect);
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        if let Some(array_collect) = self.array_collect.as_mut() {
            array_collect.update(world, ctx, set_pbr_light);
        }
    }
}