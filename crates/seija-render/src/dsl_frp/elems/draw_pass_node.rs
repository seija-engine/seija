use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_eval::Variable;
use anyhow::Result;
use lite_clojure_frp::{DynamicID, FRPSystem};
use wgpu::TextureFormat;
use crate::{dsl_frp::errors::Errors, resource::RenderResourceId, RenderContext};

use super::IUpdateNode;

pub struct DrawPassNode {
    query_index:usize,
    camera_entity:Option<Entity>,
    targets:Vec<DynamicID>,
    depth_texture:DynamicID,
    path_name:String,
    cache_formats:Vec<TextureFormat>,
    cache_textures:Vec<RenderResourceId>,
    targets_version:Vec<u32>,
    depth_version:u32
}

impl DrawPassNode {
    pub fn from_args(params:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let query_index = params.get(0).and_then(Variable::cast_int).ok_or(Errors::TypeCastError("int"))? as usize;
        let mut camera_entity = None;
        if let Some(index) = params.get(1).and_then(Variable::cast_int) {
            camera_entity = Some(Entity::from_bits(index as u64)); 
        }
        let mut targets = vec![];
        let texture_array = params.get(2).and_then(Variable::cast_vec).ok_or(Errors::TypeCastError("vector"))?;
        for texture_var in texture_array.borrow().iter() {
            if let Some(dyn_id) = texture_var.cast_int() {
                targets.push(dyn_id as DynamicID);
            }
        }
        let depth_texture = params.get(3).and_then(Variable::cast_int).ok_or(Errors::TypeCastError("int"))? as DynamicID;
        let path_name = params.get(3).and_then(Variable::cast_string).ok_or(Errors::TypeCastError("string"))?.borrow().clone();
        Ok(Box::new(DrawPassNode {
            query_index,
            camera_entity,
            targets,
            depth_texture,
            path_name,
            cache_textures:vec![],
            cache_formats:vec![],
            targets_version:vec![],
            depth_version:0
        }))
    }
}

impl DrawPassNode {
    pub fn check_update_textures(&mut self,frp_sys:&mut FRPSystem) -> Result<()> {
        self.update_textures(frp_sys)
    }

    fn update_textures(&mut self,frp_sys:&mut FRPSystem) -> Result<()> {
        for tex_dyn_id in self.targets.iter() {
            frp_sys.dynamics.get(&tex_dyn_id);
        }

        Ok(())
    }
}

impl IUpdateNode for DrawPassNode {
    fn active(&mut self,_world:&mut World,_ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        
        Ok(())
    }

    fn update(&mut self,_world:&mut World,_ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.check_update_textures(frp_sys)?;

        Ok(())
    }
}