use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_eval::Variable;
use lite_clojure_frp::{DynamicID, FRPSystem};
use anyhow::{Result,anyhow};
use seija_asset::Assets;
use seija_core::OptionExt;
use crate::{dsl_frp::{errors::Errors, PostEffectStack}, RenderContext, UniformIndex, resource::{RenderResourceId, Texture}, pipeline::render_bindings::BindGroupBuilder};

use super::IUpdateNode;

pub struct PostStackNode {
    camera_entity: Entity,
    src_texture_id: DynamicID,
    src_version:i32,
    src_texture:Option<RenderResourceId>,
    src_format:Option<wgpu::TextureFormat>,
    src_bind_group:Option<wgpu::BindGroup>,

    dst_texture_id: DynamicID,
    dst_version:i32,
    dst_texture:Option<RenderResourceId>,
    dst_format:Option<wgpu::TextureFormat>,

    cache_texture_id:Option<RenderResourceId>,
    cache_bind_group:Option<wgpu::BindGroup>,

    post_effect_index:Option<UniformIndex>
}

impl PostStackNode {
    pub fn from_args(args: Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let camera_id = args.get(0).and_then(Variable::cast_int).ok_or(Errors::TypeCastError("int"))?;
        let src_texture_id = args.get(1).and_then(Variable::cast_int).ok_or(Errors::TypeCastError("int"))? as DynamicID;
        let dst_texture_id = args.get(2).and_then(Variable::cast_int).ok_or(Errors::TypeCastError("int"))? as DynamicID;

        Ok(Box::new(PostStackNode {
            camera_entity: Entity::from_bits(camera_id as u64),
            src_texture_id,
            src_texture:None,
            dst_texture:None,
            dst_texture_id,
            post_effect_index:None,
            src_version:-1,
            dst_version:-1,
            src_format:None,
            dst_format:None,
            cache_texture_id:None,
            src_bind_group:None,
            cache_bind_group:None
        }))
    }

    fn update_textures(&mut self,frp_system:&FRPSystem,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let camera_entity = world.get_entity(self.camera_entity).get()?;
        let post_stack = camera_entity.get::<PostEffectStack>();
        if post_stack.is_none() { return Ok(()); }
        let post_stack = post_stack.unwrap();
        
        let src_dynamic = frp_system.dynamics.get(&self.src_texture_id).get()?;
        let dst_dynamic = frp_system.dynamics.get(&self.dst_texture_id).get()?;
        let is_src_update = src_dynamic.get_version() as i32 != self.src_version;
        let is_dst_update = dst_dynamic.get_version() as i32 != self.dst_version;
        
        if is_src_update {
            let src_res_id = unsafe { &*(src_dynamic.get_value().cast_userdata().get()? as *mut RenderResourceId) };
            self.src_texture = Some(src_res_id.clone());
            self.src_format = Some(ctx.resources.get_texture_format(src_res_id, world).get()?);
            self.src_bind_group = Some(self.create_bind_group(src_res_id,ctx).get()?);
        }

        if post_stack.items.len() > 1 && (is_src_update || self.cache_texture_id.is_none()) {
            let desc_info = ctx.resources.get_texture_desc(self.src_texture.as_ref().get()?, world).get()?;
            let new_texture = Texture::create_by_desc(desc_info);
            let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
            let h_texture = textures.add(new_texture);
            let res_id = RenderResourceId::Texture(h_texture);
            self.cache_bind_group = Some(self.create_bind_group(&res_id,ctx).get()?);
            self.cache_texture_id = Some(res_id);
        }

        if is_dst_update {
            let dst_res_id = unsafe { &*(dst_dynamic.get_value().cast_userdata().get()? as *mut RenderResourceId) };
            self.dst_texture = Some(dst_res_id.clone());
            self.dst_format = Some(ctx.resources.get_texture_format(dst_res_id, world).get()?);
        }
        Ok(())
    }

    fn create_bind_group(&self,res_id:&RenderResourceId,ctx:&RenderContext) -> Option<wgpu::BindGroup> {
        if let RenderResourceId::Texture(h_texture) = res_id {
            let mut builder = BindGroupBuilder::new();
            builder.add_texture(h_texture.clone_weak());
            let layout = ctx.ubo_ctx.get_layout_(self.post_effect_index.as_ref()?);
            return Some(builder.build(layout, &ctx.device, &ctx.resources));
        };
        None
    }

    
}

impl IUpdateNode for PostStackNode {
    fn init(&mut self,_world:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let index = ctx.ubo_ctx.get_index("PostEffect").get()?;
        self.post_effect_index = Some(index);

        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_system:&mut FRPSystem) -> Result<()> {
        self.update_textures(frp_system, world, ctx)?;

        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_system:&mut FRPSystem) -> Result<()> {
        self.update_textures(frp_system, world, ctx)?;
        
        Ok(())
    }
}
