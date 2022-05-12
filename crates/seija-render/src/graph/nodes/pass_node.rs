use crate::{RenderContext, camera::camera::Camera, graph::node::INode, 
            material::{Material, MaterialStorage, RenderPath}, pipeline::{PipelineCache}, resource::{Mesh, RenderResources}};
use bevy_ecs::prelude::*;

use seija_asset::{Assets, Handle};
use seija_transform::Transform;
use wgpu::{Color, Operations, CommandEncoder};
use crate::resource::RenderResourceId;
pub struct PassNode {
    view_count:usize,
    is_depth:bool,
    arg_count:usize,
    path:RenderPath,
    operations:Operations<Color>,
    is_outinput:bool
}



impl INode for PassNode {
    fn input_count(&self) -> usize { self.arg_count }

    fn output_count(&self) -> usize {
        if self.is_outinput { 0 } else { self.arg_count }
    }
    
    fn prepare(&mut self, _world: &mut World, _:&mut RenderContext) { }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
             outputs:&mut Vec<Option<RenderResourceId>>) {
            
            let mut command = ctx.command_encoder.take().unwrap();
            if let Err(err) = self.draw(world,&mut command,inputs,ctx) {
                log::error!("pass node error:{:?}",err);
            }
            ctx.command_encoder = Some(command);

            if self.is_outinput {
                *outputs = inputs.clone();
            }
    }
}

impl PassNode {
    pub fn new(view_count:usize,is_depth:bool,is_outinput:bool,path:RenderPath) -> PassNode {
        let mut arg_count = view_count;
        if is_depth { arg_count += 1; }
        PassNode {
            view_count,
            is_depth,
            arg_count,
            path,
            is_outinput,
            operations:Operations { load:wgpu::LoadOp::Clear(wgpu::Color{r:0.01,g:0.01,b:0.01,a:0.0}), store:true }
        }
    }
    
    fn draw(&mut self,world:&mut World,command:&mut CommandEncoder,
            inputs:&Vec<Option<RenderResourceId>>,ctx:&mut RenderContext) -> Result<(),PassError> {
        let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
        let mut camera_query = world.query::<(Entity,&Transform,&Camera)>();
        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
      

        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        let mats = mat_storages.mateials.read();
        
        let mut render_pass = self.create_render_pass(inputs,&ctx.resources,command)?;
        
        for (camera_e,_,camera) in camera_query.iter(world) {
            for ves in camera.view_list.values.iter() {
                for view_entity in ves.value.iter() {
                    if let Ok((_,hmesh,hmat))  = render_query.get(world, view_entity.entity) {
                       let mesh = meshs.get(&hmesh.id).ok_or(PassError::MissMesh)?;
                       let material = mats.get(&hmat.id).ok_or(PassError::MissMaterial)?;
                      
                       if !material.is_ready(&ctx.resources) || material.def.path != self.path { continue }
                      
                       if let Some(pipelines)  = pipeline_cahce.get_pipeline(&material.def.name, mesh) {
                         if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&hmesh.id, 0) {
                            for pipeline in pipelines.pipelines.iter() {
                                let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                                let mut set_index = pipeline.set_binds(camera_e, &view_entity.entity, &mut render_pass, &ctx.ubo_ctx)
                                                                .ok_or(PassError::ErrUBOIndex)?;
                                if material.props.def.infos.len() > 0 {
                                   
                                    render_pass.set_bind_group(set_index, material.bind_group.as_ref().unwrap(), &[]);
                                    set_index += 1;
                                }
                                if material.texture_props.textures.len() > 0 {
                                    render_pass.set_bind_group(set_index, material.texture_props.bind_group.as_ref().unwrap(), &[]);
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
                            }
                         }
                       }
                    }
                }
            }
            
        }
        Ok(())
    }


    fn create_render_pass<'a>(&self,inputs:&Vec<Option<RenderResourceId>>,
                          res:&'a RenderResources,command:&'a mut CommandEncoder) -> Result<wgpu::RenderPass<'a>,PassError>  {
        let mut color_attachments:Vec<wgpu::RenderPassColorAttachment> = vec![];
        for idx in 0..self.view_count {
            let tex_id = inputs[idx].as_ref().ok_or(PassError::ErrArg)?;
            let texture = res.get_texture_view_by_resid(tex_id).ok_or(PassError::ErrTargetView)?;
            let color_attach = wgpu::RenderPassColorAttachment {
                view:texture,
                resolve_target:None,
                ops:self.operations
            };
            color_attachments.push(color_attach);
        }
        let mut depth_view:Option<wgpu::RenderPassDepthStencilAttachment> = None;
        if self.is_depth {
            let depth_id = inputs[self.view_count].as_ref().ok_or(PassError::ErrArg)?;
            let view = res.get_texture_view_by_resid(&depth_id).ok_or(PassError::ErrTargetView)?;
            depth_view = Some(wgpu::RenderPassDepthStencilAttachment {
                view,
                stencil_ops: None,
                depth_ops: Some(Operations {
                    load: wgpu::LoadOp::Clear(1.0),
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
    
    
}


#[derive(Debug)] 
enum PassError {
    ErrArg,
    ErrInput(usize),
    ErrTargetView,
    MissMesh,
    MissMaterial,
    ErrUBOIndex
}
