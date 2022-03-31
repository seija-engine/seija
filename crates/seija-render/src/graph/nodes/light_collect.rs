use bevy_ecs::prelude::{World, Entity, Changed, With, Or};
use fixedbitset::FixedBitSet;
use fnv::FnvHashMap;
use glam::{Quat, Vec3};
use seija_core::LogOption;
use seija_transform::Transform;
use crate::light::{LightEnv, Light, LightType};
use crate::memory::UniformBuffer;
use crate::{uniforms::{UBONameIndex, backends::LightBackend}, graph::node::INode, RenderContext, resource::RenderResourceId, memory::TypedUniformBuffer};

const MAX_LIGHT:usize = 10;

#[derive(Default)]
pub struct LightCollect {
   pub ubo_name:String,
   name_index:Option<UBONameIndex>,
   backend:Option<LightBackend>,

   light_idxs:FnvHashMap<u32,usize>,
   lights:Vec<Option<u32>>,
   cache_len:usize
}

impl INode for LightCollect {
    
    fn init(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        self.lights = vec![None;MAX_LIGHT];
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            match LightBackend::from_def(&info.props) {
                Ok(backend) => {
                    self.backend = Some(backend)
                },
                Err(err) => {
                    log::error!("LightBackend backend error :{}",err);
                }
            }
            self.name_index = Some(ctx.ubo_ctx.buffers.get_name_index(self.ubo_name.as_str()).unwrap())
         }
    }

    fn prepare(&mut self, world: &mut World, ctx:&mut RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>) {
        self._update(world,  ctx);
    }
}


impl LightCollect {
    pub fn _update(&mut self,world:&mut World,ctx:&mut RenderContext) -> Option<()> {
        
         //add
         let mut frame_eids:FixedBitSet = FixedBitSet::with_capacity(20);
         {
            let mut lights = world.query_filtered::<Entity,(With<Light>,With<Transform>)>();
            for e in lights.iter(world) {
                if !self.light_idxs.contains_key(&e.id()) {
                    self.add_light(e.id());
                }
                frame_eids.insert(e.id() as usize);
            }
        };

        let type_ubo = self.name_index.and_then(|index| ctx.ubo_ctx.buffers.get_buffer_mut(&index, None)).log_err("get buffer error")?;
        let backend = self.backend.as_ref().log_err("get backend error")?;
        if let Some(mut light_env) = world.get_resource_mut::<LightEnv>() {
            if light_env.is_dirty {
                backend.set_ambile_color(&mut type_ubo.buffer, light_env.ambient_color);
                light_env.clear_dirty();
            }
        }
        
        //update
        let mut lights = world.query_filtered::<(Entity,&Light,&Transform),Or<(Changed<Light>, Changed<Transform>)>>();
        for (e,light,t) in lights.iter(world) {
            let index = *self.light_idxs.get(&e.id()).log_err("get index error")?;
            Self::set_light(index, backend, light, &mut type_ubo.buffer,t);
        }
        
        if self.cache_len != frame_eids.len() {
            backend.set_light_count(&mut type_ubo.buffer, frame_eids.len() as i32);
            self.cache_len = frame_eids.len();
        }
        Some(())
    }

    fn set_light(index:usize,backend:&LightBackend,light:&Light,buffer:&mut UniformBuffer,t:&Transform) {
        backend.set_lights_position(buffer,index,t.global().position);
        backend.set_lights_type(buffer, index, light.typ.type_id() as i32);
        backend.set_lights_direction(buffer, index, t.global().rotation * Vec3::Z);
        backend.set_lights_color(buffer, index, light.color);
        backend.set_lights_intensity(buffer, index, light.intensity);
        match light.typ {
            LightType::Point => {
                backend.set_lights_ex1(buffer, index, light.range);
            },
            _ => {}
        }
    }

    fn add_light(&mut self,eid:u32) {
        for idx in 0..self.lights.len() {
            if self.lights[idx].is_none() {
                self.lights[idx] = Some(eid);
                self.light_idxs.insert(eid,idx);
                return;
            }
        }
    }
}

