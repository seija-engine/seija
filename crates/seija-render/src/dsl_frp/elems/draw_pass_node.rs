use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_eval::Variable;
use anyhow::{Result, anyhow};
use lite_clojure_frp::{DynamicID, FRPSystem};
use seija_asset::{Handle, Assets};
use seija_core::{time::Time, OptionExt};
use wgpu::{TextureFormat, CommandEncoder,Operations,Color};
use crate::{dsl_frp::errors::Errors, resource::{RenderResourceId, RenderResources, Mesh}, RenderContext, material::Material, query::QuerySystem};
use super::IUpdateNode;

#[derive(PartialEq,Debug)]
pub enum PassError {
    TextureNotReady,
    ErrTargetView,
    ErrDepthView,
    MissMesh,
    MissMaterial
}

pub struct DrawPassNode {
    query_index:usize,
    camera_entity:Option<Entity>,
    targets:Vec<DynamicID>,
    depth_texture_id:DynamicID,
    pass_name:String,

    cache_formats:Vec<TextureFormat>,
    cache_textures:Vec<RenderResourceId>,
    targets_version:Vec<u32>,
    depth_version:Option<u32>,
    depth_texture:Option<RenderResourceId>,

    operations:Operations<Color>,
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
        let depth_texture_id = params.get(3).and_then(Variable::cast_int).ok_or(Errors::TypeCastError("int"))? as DynamicID;
        let path_name = params.get(4).and_then(Variable::cast_string).ok_or(Errors::TypeCastError("string"))?.borrow().clone();
        Ok(Box::new(DrawPassNode {
            query_index,
            camera_entity,
            targets,
            depth_texture_id,
            pass_name: path_name,
            cache_textures:vec![],
            cache_formats:vec![],
            targets_version:vec![],
            depth_version:None,
            depth_texture:None,
            operations:wgpu::Operations {
                load:wgpu::LoadOp::Clear(Color {r:0f64,g:0f64,b:0f64,a:1f64 }),
                store:true  
            }
        }))
    }
}

impl DrawPassNode {
    pub fn check_update_textures(&mut self,frp_sys:&mut FRPSystem,ctx:&RenderContext,world:&World) -> Result<()> {
        let dynamic = frp_sys.dynamics.get(&self.depth_texture_id).ok_or(Errors::NotFoundDynamic)?;
        if self.depth_version != Some(dynamic.get_version()) {
            self.update_textures(frp_sys, ctx, world)?;
            return Ok(());
        }
        for (index,tex_dyn_id) in self.targets.iter().enumerate() {
            let dynamic = frp_sys.dynamics.get(&tex_dyn_id).ok_or(Errors::NotFoundDynamic)?;
            if dynamic.get_version() != self.targets_version[index] {
                self.update_textures(frp_sys, ctx, world)?;
                return Ok(());
            }
        }
        Ok(())
    }

    fn update_textures(&mut self,frp_sys:&mut FRPSystem,ctx:&RenderContext,world:&World) -> Result<()> {
        self.cache_formats.clear();
        self.cache_textures.clear();
        self.targets_version.clear();
        for tex_dyn_id in self.targets.iter() {
            let dynamic = frp_sys.dynamics.get(&tex_dyn_id).ok_or(Errors::NotFoundDynamic)?;
            let res_ptr = dynamic.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
            let res_id = unsafe { &*(res_ptr as *mut RenderResourceId)}.clone();
            let format = ctx.resources.get_texture_format(&res_id, world).ok_or(anyhow!("get texture format err"))?;
            self.cache_formats.push(format);
            self.cache_textures.push(res_id);
            self.targets_version.push(dynamic.get_version());
        }
        let dynamic = frp_sys.dynamics.get(&self.depth_texture_id).ok_or(Errors::NotFoundDynamic)?;
        let res_ptr = dynamic.get_value().cast_userdata().ok_or(Errors::NotFoundUserData("texture"))?;
        let res_id = unsafe { &*(res_ptr as *mut RenderResourceId)}.clone();
        self.depth_texture = Some(res_id);
        self.depth_version = Some(dynamic.get_version());
        Ok(())
    }

    pub fn create_render_pass<'a>(&self,res:&'a RenderResources,
        command:&'a mut CommandEncoder) -> Result<wgpu::RenderPass<'a>,PassError> {
        let mut color_attachments:Vec<wgpu::RenderPassColorAttachment> = vec![];
        for target in self.cache_textures.iter() {
            if !res.is_ready(target) {
                return Err(PassError::TextureNotReady);
            }
            let texture = res.get_texture_view_by_resid(target)
                                           .ok_or(PassError::ErrTargetView)?;
            color_attachments.push(wgpu::RenderPassColorAttachment {
                view:texture,
                resolve_target:None,
                ops:self.operations
            });
        }
        let mut depth_view:Option<wgpu::RenderPassDepthStencilAttachment> = None;
        if let Some(depth_res_id) = self.depth_texture.as_ref() {
            if !res.is_ready(depth_res_id) {
                return Err(PassError::TextureNotReady);
            }
            let texture_view = res.get_texture_view_by_resid(depth_res_id).ok_or(PassError::ErrDepthView)?;
            depth_view = Some(wgpu::RenderPassDepthStencilAttachment {
                view:texture_view,
                stencil_ops: None,
                depth_ops: Some(Operations {
                    load:  wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
            });
        }
        let pass_desc = wgpu::RenderPassDescriptor {
            label:None,
            color_attachments:color_attachments.as_slice(),
            depth_stencil_attachment:depth_view
        };
        let pass = command.begin_render_pass(&pass_desc);
        Ok(pass)
    }

    pub fn draw(&self,world:&mut World,ctx:&mut RenderContext,command:&mut CommandEncoder) -> Result<u32,PassError> {
        let ff = world.get_resource::<Time>().get().unwrap().frame();
      
        let mut render_query = world.query::<(&Handle<Mesh>,&Handle<Material>)>();
        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let materials = world.get_resource::<Assets<Material>>().unwrap();
       
        let query_system = world.get_resource::<QuerySystem>().unwrap();
        let view_query = &query_system.querys[self.query_index];
        //check build pipeline
        for entity in view_query.list.read().iter() {
            if let Ok((hmesh,hmat)) = render_query.get(world, *entity) { 
                let mesh = meshs.get(&hmesh.id).ok_or(PassError::MissMesh)?;
                let material = materials.get(&hmat.id).ok_or(PassError::MissMaterial)?;
                for pass_index in 0..material.def.pass_list.len() {
                    ctx.build_pipeine(&material.def, mesh,&self.cache_formats,Some(wgpu::TextureFormat::Depth32Float),pass_index);
                }
            }
        }
        let mut draw_count:u32 = 0;
        
        let mut render_pass = self.create_render_pass(&ctx.resources,  command)?;
       
        for entity in view_query.list.read().iter() {
           
            if let Ok((hmesh,hmat)) = render_query.get(world, *entity) { 
               
                let mesh = meshs.get(&hmesh.id).ok_or(PassError::MissMesh)?;
                let material = materials.get(&hmat.id).ok_or(PassError::MissMaterial)?;
                if !material.is_ready(&ctx.resources) { 
                    continue 
                }
                for pass_index in 0..material.def.pass_list.len() {
                    
                    let pipeline = ctx.pipeline_cache.get_pipeline(material.def.name.as_str(), 
                                                                   mesh,&self.cache_formats,
                                                                   Some(wgpu::TextureFormat::Depth32Float),pass_index);
                    if let Some(pipeline) = pipeline {
                       
                        if pipeline.tag != self.pass_name {  continue; }
                        if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&hmesh.id, 0) {

                            let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                            let oset_index = pipeline.set_binds(self.camera_entity,Some(entity.clone()), &mut render_pass, &ctx.ubo_ctx);
                            if oset_index.is_none()  {  continue }
                            let mut set_index = oset_index.unwrap();

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
                            if let Some(idx_id) = ctx.resources.get_render_resource(&hmesh.id, 1) {
                                let idx_buffer = ctx.resources.get_buffer_by_resid(&idx_id).unwrap();
                                render_pass.set_index_buffer(idx_buffer.slice(0..), mesh.index_format().unwrap());
                                render_pass.set_pipeline(&pipeline.pipeline);
                                render_pass.draw_indexed(mesh.indices_range().unwrap(),0, 0..1);
                                    
                            } else {
                                render_pass.set_pipeline(&pipeline.pipeline);
                                render_pass.draw(0..mesh.count_vertices() as u32, 0..1);
                            }
                            draw_count += 1;
                        }
                    }
                }
            }
        }
        Ok(draw_count)
    }
}

impl IUpdateNode for DrawPassNode {
    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.check_update_textures(frp_sys,ctx,world)?;
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.check_update_textures(frp_sys,ctx,world)?;

        let mut command = ctx.command_encoder.take().unwrap();
        match self.draw(world, ctx, &mut command) {
            Err(err) => {
                if err != PassError::TextureNotReady {
                    log::error!("draw pass error:{:?}",err);
                }
            },
            Ok(draw_count) => {
                if draw_count > 0 { ctx.frame_draw_pass += draw_count; }
            }
        }
        ctx.command_encoder = Some(command);
        Ok(())
    }
}