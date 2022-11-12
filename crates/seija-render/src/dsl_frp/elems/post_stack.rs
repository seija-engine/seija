use anyhow::Result;
use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_eval::Variable;
use lite_clojure_frp::{DynamicID, FRPSystem};

use crate::{dsl_frp::errors::Errors, RenderContext, resource::RenderResourceId};

use super::IUpdateNode;

//(node POST_STACK camera_id hdr-texture window-texture)
pub struct PostStackNode {
    camera_entity: Entity,
    src_texture_id: DynamicID,
    dst_texture_id: DynamicID,

    src_texture:Option<RenderResourceId>,
    dst_texture:Option<RenderResourceId>,
    src_version:Option<u32>,
    dst_version:Option<u32>
}

impl PostStackNode {
    pub fn from_args(args: Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let camera_id = args
            .get(0)
            .and_then(Variable::cast_int)
            .ok_or(Errors::TypeCastError("int"))?;
        let src_texture_id = args
            .get(1)
            .and_then(Variable::cast_int)
            .ok_or(Errors::TypeCastError("int"))? as DynamicID;
        let dst_texture_id = args
            .get(2)
            .and_then(Variable::cast_int)
            .ok_or(Errors::TypeCastError("int"))? as DynamicID;

        Ok(Box::new(PostStackNode {
            camera_entity: Entity::from_bits(camera_id as u64),
            src_texture_id,
            dst_texture_id,
            src_texture:None,
            dst_texture:None,
            src_version:None,
            dst_version:None
        }))
    }
}

impl PostStackNode {
    pub fn check_update_textures(&mut self,frp_sys:&mut FRPSystem) -> Result<()> {
        let src_dynamic = frp_sys.dynamics.get(&self.src_texture_id).ok_or(Errors::NotFoundDynamic)?;
        if Some(src_dynamic.get_version()) != self.src_version {
            self.update_textures(frp_sys)?;
            return Ok(());
        }
        let dst_dynamic = frp_sys.dynamics.get(&self.dst_texture_id).ok_or(Errors::NotFoundDynamic)?;
        if Some(dst_dynamic.get_version()) != self.dst_version {
            self.update_textures(frp_sys)?;
            return Ok(());
        }
        Ok(())
    }

    pub fn update_textures(&mut self,frp_sys:&mut FRPSystem) -> Result<()> {
        let src_dyn_id = frp_sys.dynamics.get(&self.src_texture_id).ok_or(Errors::NotFoundDynamic)?;
        let res_ptr = src_dyn_id.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
        self.src_texture = Some(unsafe { &*(res_ptr as *mut RenderResourceId)}.clone());
       
        let dst_dyn_id = frp_sys.dynamics.get(&self.dst_texture_id).ok_or(Errors::NotFoundDynamic)?;
        let res_ptr = dst_dyn_id.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
        self.dst_texture = Some(unsafe { &*(res_ptr as *mut RenderResourceId)}.clone());
        Ok(())
    }
}

impl IUpdateNode for PostStackNode {
    fn init(&mut self,_world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        
        Ok(())
    }

    fn active(&mut self,_world:&mut World,_ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.check_update_textures(frp_sys)?;

        Ok(())
    }

    fn update(&mut self,_world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        
        Ok(())
    }
}
