use anyhow::{Result,anyhow};
use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_eval::Variable;
use lite_clojure_frp::{DynamicID, FRPSystem};
use seija_asset::{Handle, AssetServer, Assets};
use crate::{dsl_frp::{errors::Errors, PostEffectStack}, RenderContext, resource::{RenderResourceId, Mesh, Texture}, material::Material};
use super::IUpdateNode;
use wgpu::{Operations,Color, CommandEncoder};

pub struct PostStackNode {
    camera_entity: Entity,
    src_texture_id: DynamicID,
    dst_texture_id: DynamicID,

    src_texture:Option<RenderResourceId>,
    src_format:Option<wgpu::TextureFormat>,
    dst_format:Option<wgpu::TextureFormat>,
    dst_texture:Option<RenderResourceId>,
    cache_texture:Option<RenderResourceId>,
    src_version:Option<u32>,
    dst_version:Option<u32>,
    quad_mesh:Option<Handle<Mesh>>,
    cache_pass_format:Vec<wgpu::TextureFormat>,
    last_state:LastTextureState,

    operations:Operations<Color>,
}

#[derive(PartialEq, Eq)]
enum LastTextureState {
    None,
    SrcToCache,
    CacheToSrc
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
            src_format:None,
            dst_format:None,
            src_version:None,
            dst_version:None,
            quad_mesh:None,
            cache_texture:None,
            last_state:LastTextureState::None,
            cache_pass_format:vec![wgpu::TextureFormat::Rgba8Unorm],
            operations:wgpu::Operations {
                load:wgpu::LoadOp::Clear(Color {r:0f64,g:0f64,b:0f64,a:1f64 }),
                store:true  
            }
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
            self.update_textures(frp_sys,ctx,world)?;
            if stack_effect_count > 1 {
                self.update_cache_texture(ctx, world)?;
            }
            return Ok(());
        }
        let dst_dynamic = frp_sys.dynamics.get(&self.dst_texture_id).ok_or(Errors::NotFoundDynamic)?;
        if Some(dst_dynamic.get_version()) != self.dst_version {
            self.dst_version = Some(dst_dynamic.get_version());
            self.update_textures(frp_sys,ctx,world)?;
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
        Ok(())
    }

    pub fn update_textures(&mut self,frp_sys:&mut FRPSystem,ctx:&mut RenderContext,world:&World) -> Result<()> {
        let src_dyn_id = frp_sys.dynamics.get(&self.src_texture_id).ok_or(Errors::NotFoundDynamic)?;
        let res_ptr = src_dyn_id.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
        let res_id = unsafe { &*(res_ptr as *mut RenderResourceId)}.clone();
        self.src_format = Some(ctx.resources.get_texture_format(&res_id, world).ok_or(Errors::GetResTextureFormatError)?);
        self.src_texture = Some(res_id);
       
        let dst_dyn_id = frp_sys.dynamics.get(&self.dst_texture_id).ok_or(Errors::NotFoundDynamic)?;
        let res_ptr = dst_dyn_id.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
        let res_id = unsafe { &*(res_ptr as *mut RenderResourceId)}.clone();
        self.dst_format = Some(ctx.resources.get_texture_format(&res_id, world).ok_or(Errors::GetResTextureFormatError)?);
        self.dst_texture = Some(res_id);
        Ok(())
    }

    fn get_target_format(&self,is_last:bool) -> wgpu::TextureFormat {
        if is_last {  return self.dst_format.unwrap(); }
        self.src_format.unwrap()
    }

    fn calc_from_to_texture(&mut self,is_last:bool,from_id:&mut RenderResourceId,to_id:&mut RenderResourceId) {
        if is_last {
            *to_id = self.dst_texture.clone().unwrap();   
        }
        match self.last_state {
            LastTextureState::SrcToCache => { 
                if is_last {
                    *from_id = self.cache_texture.clone().unwrap();
                } else {
                    *from_id = self.src_texture.clone().unwrap();
                    *to_id = self.cache_texture.clone().unwrap();   
                }
                self.last_state = LastTextureState::CacheToSrc;
            },
            LastTextureState::CacheToSrc => { 
                if is_last {
                    *from_id = self.src_texture.clone().unwrap();
                } else {
                    *from_id = self.cache_texture.clone().unwrap();
                    *to_id = self.src_texture.clone().unwrap();   
                }
                self.last_state = LastTextureState::SrcToCache;
            },
            LastTextureState::None => { 
                if is_last {
                    *from_id = self.src_texture.clone().unwrap();
                } else {
                    *from_id = self.src_texture.clone().unwrap();
                    *to_id = self.cache_texture.clone().unwrap();   
                }
                self.last_state = LastTextureState::SrcToCache;
            }
        }
    }

    fn draw(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem,command:&mut CommandEncoder) -> Result<()> {
        self.last_state = LastTextureState::None;
        let camera_entity = world.get_entity(self.camera_entity).ok_or(anyhow!("camera entity error"))?;
        if let Some(post_stack) = camera_entity.get::<PostEffectStack>() {
            let materials = world.get_resource::<Assets<Material>>().unwrap();
            let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
            let quad_mesh_id = self.quad_mesh.as_ref().unwrap().id.clone();
            let quad_mesh = meshs.get(&self.quad_mesh.as_ref().unwrap().id).unwrap();
            
            for (index,effect_item) in post_stack.items.iter().enumerate() {
                let material = materials.get(&effect_item.material.id).ok_or(Errors::MissMaterial)?;
                if !material.is_ready(&ctx.resources) { continue }
                for pass_index in 0..material.def.pass_list.len() {
                   let mut color_attachments = vec![];
                   let is_last = pass_index == material.def.pass_list.len() - 1 && index == post_stack.items.len() - 1;
                   let target_format = self.get_target_format(is_last);
                   self.cache_pass_format[0] = target_format;
                   ctx.build_pipeine(&material.def, quad_mesh, &self.cache_pass_format, pass_index);
                   
                   let mut from_res_id = RenderResourceId::MainSwap;
                   let mut dst_res_id = RenderResourceId::MainSwap;
                   self.calc_from_to_texture(is_last, &mut from_res_id, &mut dst_res_id);
                   //begin VKPass
                   if let Some(dst_texture_view) = ctx.resources.get_texture_view_by_resid(&dst_res_id) {
                    color_attachments.push(wgpu::RenderPassColorAttachment { 
                        view: dst_texture_view, 
                        resolve_target: None, 
                        ops:self.operations 
                     });
                     let pass_desc = wgpu::RenderPassDescriptor {
                      label:None,
                      color_attachments:color_attachments.as_slice(),
                      depth_stencil_attachment:None
                     };
                     let mut render_pass = command.begin_render_pass(&pass_desc);
                     if let Some(pipeline) = ctx.pipeline_cache.get_pipeline(&material.def.name, &quad_mesh, &self.cache_pass_format, pass_index) {
                        if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&quad_mesh_id, 0) {
                            let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                            let offset_index = pipeline.set_binds(Some(self.camera_entity), None, &mut render_pass, &ctx.ubo_ctx);
                            if offset_index.is_none()  { continue }
                           
                            let mut set_index = offset_index.unwrap();
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

                         }
                     }
                   }
                }
            }
        }
        Ok(())
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
        let mut command = ctx.command_encoder.take().unwrap();
        if let Err(err) = self.draw(world, ctx, frp_sys, &mut command) {
            log::error!("post_stack node error:{:?}",err);
        }
        ctx.command_encoder = Some(command);
        Ok(())
    }

    
}
