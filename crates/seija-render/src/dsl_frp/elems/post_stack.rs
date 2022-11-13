use anyhow::{Result,anyhow};
use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_eval::Variable;
use lite_clojure_frp::{DynamicID, FRPSystem};
use seija_asset::{Handle, AssetServer, Assets};
use crate::{dsl_frp::{errors::Errors, PostEffectStack}, RenderContext, resource::{RenderResourceId, Mesh, Texture}};
use super::IUpdateNode;

pub struct PostStackNode {
    camera_entity: Entity,
    src_texture_id: DynamicID,
    dst_texture_id: DynamicID,

    src_texture:Option<RenderResourceId>,
    dst_texture:Option<RenderResourceId>,
    cache_texture:Option<RenderResourceId>,
    src_version:Option<u32>,
    dst_version:Option<u32>,
    quad_mesh:Option<Handle<Mesh>>
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
            dst_version:None,
            quad_mesh:None,
            cache_texture:None
        }))
    }
}

impl PostStackNode {
    pub fn check_update_textures(&mut self,frp_sys:&mut FRPSystem,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let mut stack_effect_count = 0;
        let camera_entity = world.get_entity(self.camera_entity).ok_or(anyhow!("camera entity error"))?;
        if let Some(post_stack) = camera_entity.get::<PostEffectStack>() {
            stack_effect_count = post_stack.items.len();
        }

        let src_dynamic = frp_sys.dynamics.get(&self.src_texture_id).ok_or(Errors::NotFoundDynamic)?;
        if Some(src_dynamic.get_version()) != self.src_version {
            self.src_version = Some(src_dynamic.get_version());
            self.update_textures(frp_sys,ctx)?;
            if stack_effect_count > 1 {
                self.update_cache_texture(ctx, world)?;
            }
            return Ok(());
        }
        let dst_dynamic = frp_sys.dynamics.get(&self.dst_texture_id).ok_or(Errors::NotFoundDynamic)?;
        if Some(dst_dynamic.get_version()) != self.dst_version {
            self.dst_version = Some(dst_dynamic.get_version());
            self.update_textures(frp_sys,ctx)?;
            return Ok(());
        }

        if stack_effect_count > 1 && self.cache_texture.is_none() {
            self.update_cache_texture(ctx, world)?;
        }

        Ok(())
    }

    fn update_cache_texture(&mut self,ctx:&RenderContext,world:&mut World) -> Result<()> {
        let src_texture_id = self.src_texture.as_ref().ok_or(Errors::NotFoundDynamic)?;
        let desc_info = ctx.resources.get_texture_desc(src_texture_id, world)
                                            .ok_or(anyhow!("get src texture desc error"))?;
        let new_texture = Texture::create_by_desc(desc_info);
        let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
        let h_texture = textures.add(new_texture);
        self.cache_texture = Some(RenderResourceId::Texture(h_texture));
        log::error!("in ????");
        Ok(())
    }

    pub fn update_textures(&mut self,frp_sys:&mut FRPSystem,ctx:&mut RenderContext) -> Result<()> {
        let src_dyn_id = frp_sys.dynamics.get(&self.src_texture_id).ok_or(Errors::NotFoundDynamic)?;
        let res_ptr = src_dyn_id.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
        self.src_texture = Some(unsafe { &*(res_ptr as *mut RenderResourceId)}.clone());
       
        let dst_dyn_id = frp_sys.dynamics.get(&self.dst_texture_id).ok_or(Errors::NotFoundDynamic)?;
        let res_ptr = dst_dyn_id.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
        self.dst_texture = Some(unsafe { &*(res_ptr as *mut RenderResourceId)}.clone());
        Ok(())
    }

    fn create_pass(&self,ctx:&RenderContext) {
        let mut color_attachments:Vec<wgpu::RenderPassColorAttachment> = vec![];
        //let texture = res.get_texture_view_by_resid(target)//.ok_or(PassError::ErrTargetView)?;
    }
}

impl IUpdateNode for PostStackNode {
    fn init(&mut self,world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let server = world.get_resource::<AssetServer>().unwrap();
        self.quad_mesh = Some(server.get_asset("mesh:quad2").as_ref().unwrap().make_weak_handle().typed());
       
        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.check_update_textures(frp_sys,world,ctx)?;
        
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.check_update_textures(frp_sys,world,ctx)?;
        let camera_entity = world.get_entity(self.camera_entity).ok_or(anyhow!("camera entity error"))?;
        if let Some(post_stack) = camera_entity.get::<PostEffectStack>() {
            let mut command = ctx.command_encoder.take().unwrap();
            for effect_item in post_stack.items.iter() {
                
            }
        }
        Ok(())
    }

    
}
