use crate::{RenderContext, camera::camera::Camera, graph::node::INode, material::{Material, MaterialStorage}, pipeline::{PipelineCache}, resource::Mesh, uniforms::UBOApplyType};
use bevy_ecs::prelude::*;

use seija_asset::{Assets, Handle};
use seija_transform::Transform;
use wgpu::{Color, Operations, CommandEncoder, TextureView};
use crate::resource::RenderResourceId;
pub struct PassNode {
    operations:Operations<Color>,
}



impl INode for PassNode {
    fn input_count(&self) -> usize { 3 }
    
    fn prepare(&mut self, _world: &mut World, ctx:&mut RenderContext) {
    
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,_outputs:&mut Vec<Option<RenderResourceId>>) {
            let mut command = ctx.command_encoder.take().unwrap();
            if let Err(err) = self.draw(world,&mut command,inputs,ctx) {
                log::error!("pass node error:{:?}",err);
            }
            ctx.command_encoder = Some(command);
    }
}

impl PassNode {
    pub fn new() -> PassNode {
        PassNode {
            operations:Operations { load:wgpu::LoadOp::Clear(wgpu::Color{r:0.01,g:0.01,b:0.01,a:0.0}), store:true }
        }
    }
    
    fn draw(&mut self,world:&mut World,command:&mut CommandEncoder,inputs:&Vec<Option<RenderResourceId>>,ctx:&mut RenderContext) -> Result<(),PassError> {
        let target_resid= inputs[0].as_ref().ok_or(PassError::ErrInput(0))?;
        let depth_resid =  inputs[1].as_ref().ok_or(PassError::ErrInput(1))?;
       
        let target_view = ctx.resources.get_texture_view_by_resid(target_resid).ok_or(PassError::ErrTargetView)?;
        let depth_view = ctx.resources.get_texture_view_by_resid( depth_resid).ok_or(PassError::ErrTargetView)?;
        let msaa_view =  inputs[2].as_ref().and_then(|id| ctx.resources.get_texture_view_by_resid(id));
       
        let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
        let mut camera_query = world.query::<(Entity,&Transform,&Camera)>();
        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
      

        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        let mats = mat_storages.mateials.read();
        
        
        let mut render_pass = command.begin_render_pass(&wgpu::RenderPassDescriptor {
            label:None,
            color_attachments:&[wgpu::RenderPassColorAttachment {
                view:if let Some(msaa) = msaa_view {msaa } else { target_view },
                resolve_target:if msaa_view.is_some() { Some(target_view) } else { None },
                ops:self.operations
            }],
            depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {
                view:depth_view,
                stencil_ops: None,
                depth_ops: Some(Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
            }),
        });

        for (camera_e,_,camera) in camera_query.iter(world) {
            for ves in camera.view_list.values.iter() {
                for view_entity in ves.value.iter() {
                    if let Ok((_,hmesh,hmat))  = render_query.get(world, view_entity.entity) {
                       let mesh = meshs.get(&hmesh.id).ok_or(PassError::MissMesh)?;
                       let material = mats.get(&hmat.id).ok_or(PassError::MissMaterial)?;
                       if !material.is_ready(&ctx.resources) { continue }
                       if let Some(pipelines)  = pipeline_cahce.get_pipeline(&material.def.name, mesh) {
                         if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&hmesh.id, 0) {
                            for pipeline in pipelines.pipelines.iter() {
                                let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                                let mut set_index = 0;
                                for (index,ubo_name_index) in pipeline.ubos.iter().enumerate() {
                                    match ubo_name_index.2 {
                                        UBOApplyType::Camera => {
                                            let bind_group = ctx.ubo_ctx.buffers.get_bind_group(ubo_name_index, Some(camera_e.id())).ok_or(PassError::ErrUBOIndex)?;
                                            render_pass.set_bind_group(index as u32, bind_group, &[]);
                                        },
                                        UBOApplyType::RenderObject => {
                                            let bind_group = ctx.ubo_ctx.buffers.get_bind_group(ubo_name_index, Some(view_entity.entity.id())).ok_or(PassError::ErrUBOIndex)?;
                                            render_pass.set_bind_group(index as u32, bind_group, &[]);
                                        },
                                        UBOApplyType::Frame => {
                                            let bind_group = ctx.ubo_ctx.buffers.get_bind_group(ubo_name_index, None).ok_or(PassError::ErrUBOIndex)?;
                                            render_pass.set_bind_group(index as u32, bind_group, &[]);
                                        }    
                                    };
                                    set_index += 1;
                                }
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


    
    
}


#[derive(Debug)] 
enum PassError {
    ErrInput(usize),
    ErrTargetView,
    MissMesh,
    MissMaterial,
    ErrUBOIndex
}