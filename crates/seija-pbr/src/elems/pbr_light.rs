use bevy_ecs::world::World;
use glam::Vec3;
use lite_clojure_eval::Variable;
use seija_render::{dsl_frp::{UBOArrayCollect, IUpdateNode}, RenderContext, UniformBuffer};
use anyhow::{Result,anyhow};
use seija_transform::Transform;
use crate::lights::{PBRLight, PBRLightType, PBRGlobalAmbient};

use super::pbr_light_backend::PBRLightBackend;

pub struct PBRLightNode {
    pub ubo_name:String,
    array_collect:Option<UBOArrayCollect<PBRLightBackend,PBRLight>> 
}


impl PBRLightNode {
    pub fn from_args(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let name = args.get(0).and_then(Variable::cast_string)
                                          .ok_or(anyhow!("type cast error"))?;
        let br_name = name.borrow().clone();
        let node = Box::new(PBRLightNode {
            ubo_name:br_name,
            array_collect:None
        });
        Ok(node)
    }
}

impl IUpdateNode for PBRLightNode {
    fn active(&mut self,_world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let mut array_collect = UBOArrayCollect::new(self.ubo_name.clone(), 10);
        array_collect.init(ctx);
        self.array_collect = Some(array_collect);
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        if let Some(array_collect) = self.array_collect.as_mut() {
            array_collect.update(world, ctx, set_pbr_light);
            if let Some(mut ambient) = world.get_resource_mut::<PBRGlobalAmbient>() {
                if ambient.is_dirty() {
                    if let (Some(backend),Some(name_index)) = (array_collect.backend.as_ref(),array_collect.name_index) {
                        ctx.ubo_ctx.set_buffer(&name_index, None, |v| {
                            backend.set_ambile_color(&mut v.buffer, ambient.color);
                        });
                        ambient.clear_dirty();
                    }
                   
                }
            }
        }
        Ok(())
    }
}


fn set_pbr_light(backend:&PBRLightBackend,index:usize,light:&PBRLight,buffer:&mut UniformBuffer,t:&Transform) {
    let dir = t.global().rotation * Vec3::Z;
    log::debug!("set_pbr_light dir:{:?}",dir.normalize());
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