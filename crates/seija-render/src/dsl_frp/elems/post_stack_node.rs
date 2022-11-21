use std::{collections::HashMap};

use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_eval::Variable;
use lite_clojure_frp::{DynamicID, FRPSystem};
use anyhow::{Result};
use seija_asset::{Assets, Handle, AssetServer};
use seija_core::OptionExt;
use wgpu::{CommandEncoder,Operations,Color};
use crate::{dsl_frp::{errors::Errors, PostEffectStack}, 
RenderContext, UniformIndex, resource::{RenderResourceId, Texture, Mesh}, 
pipeline::render_bindings::BindGroupBuilder, material::Material, uniforms::UBOApplyType};

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

    post_effect_index:Option<UniformIndex>,

    cache_quads:HashMap<Handle<Material>,Entity>,

    quad_mesh:Option<Handle<Mesh>>,
    
    cache_pass_format:Vec<wgpu::TextureFormat>,
    operations:Operations<Color>,
    last_state:LastTextureState
}

#[derive(PartialEq, Eq,Clone, Copy)]
enum LastTextureState {
    SrcToCache,
    CacheToSrc
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
            cache_bind_group:None,
            cache_quads:Default::default(),
            quad_mesh:None,
            cache_pass_format:vec![wgpu::TextureFormat::Rgba8Unorm],
            operations:wgpu::Operations {
                load:wgpu::LoadOp::Clear(Color {r:0f64,g:0f64,b:0f64,a:1f64 }),
                store:true  
            },
            last_state:LastTextureState::SrcToCache
        }))
    }

    fn update_textures(&mut self,frp_system:&FRPSystem,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let camera_entity = world.get_entity(self.camera_entity).get()?;
        let post_stack = camera_entity.get::<PostEffectStack>();
        if post_stack.is_none() { return Ok(()); }
        let post_stack_count = post_stack.get()?.items.len();
        
        let src_dynamic = frp_system.dynamics.get(&self.src_texture_id).get()?;
        let dst_dynamic = frp_system.dynamics.get(&self.dst_texture_id).get()?;
        let is_src_update = src_dynamic.get_version() as i32 != self.src_version;
        let is_dst_update = dst_dynamic.get_version() as i32 != self.dst_version;
        
        if is_src_update {
            let src_res_id = unsafe { &*(src_dynamic.get_value().cast_userdata().get()? as *mut RenderResourceId) };
            self.src_texture = Some(src_res_id.clone());
            self.src_format = Some(ctx.resources.get_texture_format(src_res_id, world).get()?);
            if let RenderResourceId::Texture(h_texture) = src_res_id {
                Texture::to_gpu(h_texture, world, ctx)?;
            }
            self.src_bind_group = Some(self.create_bind_group(src_res_id,ctx).get()?);
            self.src_version = src_dynamic.get_version() as i32;
        }

        if post_stack_count > 1 && (is_src_update || self.cache_texture_id.is_none()) {
            let desc_info = ctx.resources.get_texture_desc(self.src_texture.as_ref().get()?, world).get()?;
            let new_texture = Texture::create_by_desc(desc_info);
            let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
            let h_texture = textures.add(new_texture);
            Texture::to_gpu(&h_texture, world, ctx)?;
            let res_id = RenderResourceId::Texture(h_texture);
            self.cache_bind_group = Some(self.create_bind_group(&res_id,ctx).get()?);
            self.cache_texture_id = Some(res_id);
        }

        if is_dst_update {
            let dst_res_id = unsafe { &*(dst_dynamic.get_value().cast_userdata().get()? as *mut RenderResourceId) };
            self.dst_texture = Some(dst_res_id.clone());
            self.dst_format = Some(ctx.resources.get_texture_format(dst_res_id, world).get()?);
            self.dst_version = dst_dynamic.get_version() as i32;
        }
        Ok(())
    }

    fn create_bind_group(&self,res_id:&RenderResourceId,ctx:&RenderContext) -> Option<wgpu::BindGroup> {
        if let RenderResourceId::Texture(h_texture) = res_id {
            let mut builder = BindGroupBuilder::new();
            builder.add_texture(h_texture.clone_weak());
            let layout = ctx.ubo_ctx.get_layout_(self.post_effect_index.as_ref()?);
            let group = builder.build(layout, &ctx.device, &ctx.resources);
           
            return Some(group);
        };
        None
    }

    fn update_cache_quads(&mut self,world:&mut World) -> Result<()> {
        let camera_entity = world.get_entity(self.camera_entity).get()?;
        let post_stack = camera_entity.get::<PostEffectStack>();
        if post_stack.is_none() { return Ok(()); }
        let post_stack = post_stack.get()?;
        let mut new_lst:Vec<Handle<Material>> = vec![];
        for effect in post_stack.items.iter() {
            if !self.cache_quads.contains_key(&effect.material) {
                new_lst.push(effect.material.clone_weak());
            }
        }

        for h_material in new_lst.drain(..) {
            let mut new_quad = world.spawn();
            new_quad.insert(h_material.clone_weak());
            let e_new_quad = new_quad.id();
            self.cache_quads.insert(h_material, e_new_quad);
        }
        //TODO 卸载不用的
        Ok(())
    }

    fn draw(&mut self,world:&mut World,ctx:&mut RenderContext,command:&mut CommandEncoder) -> Result<bool> {
        self.last_state = LastTextureState::SrcToCache;
        let camera_entity = world.get_entity(self.camera_entity).get()?;
        let post_stack =  camera_entity.get::<PostEffectStack>();
        if post_stack.is_none() { return Ok(true); }
        let post_stack = post_stack.get()?;
        
        let materials = world.get_resource::<Assets<Material>>().get()?;
        let meshs = world.get_resource::<Assets<Mesh>>().get()?;
        let quad_mesh_id = self.quad_mesh.as_ref().get()?.id.clone();
        let quad_mesh = meshs.get(&quad_mesh_id).get()?;
        let uniform_index = self.post_effect_index.as_ref().get()?.index;
        for (index,effect_item) in post_stack.items.iter().enumerate() {
            let material = materials.get(&effect_item.material.id).get()?;
            if !material.is_ready(&ctx.resources) { continue }
            for pass_index in 0..material.def.pass_list.len() {
                let mut color_attachments:Vec<Option<wgpu::RenderPassColorAttachment>> = vec![];
                let is_last = pass_index == material.def.pass_list.len() - 1 && index == post_stack.items.len() - 1;
                let target_format = if is_last { self.dst_format.get()? } else { self.src_format.get()? };
                self.cache_pass_format[0] = target_format;
                ctx.build_pipeine(&material.def, quad_mesh, &self.cache_pass_format, None,pass_index);
                let dst_res_id = self.cur_target_texture(is_last)?;
                let dst_texture_view = ctx.resources.get_texture_view_by_resid(dst_res_id).get()?;
                color_attachments.push(Some(wgpu::RenderPassColorAttachment { 
                    view: dst_texture_view, 
                    resolve_target: None, 
                    ops:self.operations 
                }));
                let pass_desc = wgpu::RenderPassDescriptor {
                    label:None,
                    color_attachments:color_attachments.as_slice(),
                    depth_stencil_attachment:None
                };
                
                let mut render_pass = command.begin_render_pass(&pass_desc);
                if let Some(pipeline) = ctx.pipeline_cache.get_pipeline(&material.def.name,&quad_mesh,&self.cache_pass_format,None, pass_index) {
                    if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&quad_mesh_id, 0) {
                        
                        let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                        
                        for (index,ubo_name_index) in pipeline.ubos.iter().enumerate() {
                            match ubo_name_index.apply_type {
                             UBOApplyType::Camera => {
                                 if ubo_name_index.index == uniform_index {
                                    let src_bind_group = self.cur_src_bind_group()?;
                                    render_pass.set_bind_group(index as u32,src_bind_group, &[]);
                                 } else {
                                    let bind_group = ctx.ubo_ctx.get_bind_group(ubo_name_index, Some( self.camera_entity.id() )).get()?;
                                    render_pass.set_bind_group(index as u32, &bind_group, &[]);
                                 }
                              },
                             UBOApplyType::RenderObject => { log::error!("post effect shader has object backend"); },
                             UBOApplyType::Frame => {
                                let bind_group = ctx.ubo_ctx.get_bind_group(ubo_name_index, None).get()?;
                                render_pass.set_bind_group(index as u32, bind_group, &[]);
                             }
                            }
                        }
                        let mut set_index = pipeline.ubos.len() as u32;
                            if material.props.def.infos.len() > 0 {
                                if let Some(bind_group) = material.bind_group.as_ref() {
                                    render_pass.set_bind_group(set_index, bind_group, &[]);
                                    set_index += 1;
                                } else {
                                    continue;
                                }
                            }
                            if material.texture_props.textures.len() > 0  {
                                if let Some(bind_group) = material.texture_props.bind_group.as_ref() {
                                    render_pass.set_bind_group(set_index, bind_group, &[]);
                                } else {
                                    continue;
                                }
                            }

                            render_pass.set_vertex_buffer(0, vert_buffer.slice(0..));
                            if let Some(idx_id) = ctx.resources.get_render_resource(&quad_mesh_id, 1) {
                               
                                let idx_buffer = ctx.resources.get_buffer_by_resid(&idx_id).unwrap();
                                render_pass.set_index_buffer(idx_buffer.slice(0..), quad_mesh.index_format().unwrap());
                                render_pass.set_pipeline(&pipeline.pipeline);
                                render_pass.draw_indexed(quad_mesh.indices_range().unwrap(),0, 0..1);
                                    
                            } else {
                               
                                render_pass.set_pipeline(&pipeline.pipeline);
                                render_pass.draw(0..quad_mesh.count_vertices() as u32, 0..1);
                            }
                            drop(render_pass);
                            match self.last_state {
                                LastTextureState::SrcToCache => {
                                    self.last_state = LastTextureState::CacheToSrc;    
                                },
                                LastTextureState::CacheToSrc => {
                                    self.last_state = LastTextureState::SrcToCache;   
                                }
                            }

                    }
                }
            }
        }
        Ok(true)
    }

    fn cur_target_texture(&self,is_last:bool) -> Result<&RenderResourceId> {
        if is_last {  return Ok(self.dst_texture.as_ref().get()?)  }
        match self.last_state {
             LastTextureState::SrcToCache => {
                return Ok(self.cache_texture_id.as_ref().get()?)
            },
            _ => { return Ok(self.src_texture.as_ref().get()?)}
        }
    }

    fn cur_src_bind_group(&self) -> Result<&wgpu::BindGroup> {
        match self.last_state {
            LastTextureState::SrcToCache => {
                return Ok(self.src_bind_group.as_ref().get()?)
            },
            _ => { 
                return Ok(self.cache_bind_group.as_ref().get()?)
            }
        }
    }
    
}

impl IUpdateNode for PostStackNode {
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let index = ctx.ubo_ctx.get_index("PostEffect").get()?;
        self.post_effect_index = Some(index);
        let server = world.get_resource::<AssetServer>().get()?;
        self.quad_mesh = Some(server.get_asset("mesh:quad2").get()?.make_weak_handle().typed());
        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_system:&mut FRPSystem) -> Result<()> {
        self.update_textures(frp_system, world, ctx)?;
        self.update_cache_quads(world)?;
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_system:&mut FRPSystem) -> Result<()> {
        
        self.update_textures(frp_system, world, ctx)?;
        self.update_cache_quads(world)?;
        let mut command = ctx.command_encoder.take().get()?;
        if let Err(err) = self.draw(world, ctx, &mut command) {
            log::error!("post stack node draw error:{:?}",err);
        }
        ctx.command_encoder = Some(command);
        Ok(())
    }
}
