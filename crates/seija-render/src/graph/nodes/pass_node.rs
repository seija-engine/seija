use crate::{RenderContext, camera::camera::Camera, graph::node::INode, material::{Material, MaterialStorage}, pipeline::{PipelineCache}, resource::Mesh};
use bevy_ecs::prelude::*;

use seija_asset::{Assets, Handle};
use seija_transform::Transform;
use wgpu::{Color, Operations, CommandEncoder, RenderPass};
use crate::resource::RenderResourceId;
pub struct PassNode {
    operations:Operations<Color>,
}



impl INode for PassNode {
    fn input_count(&self) -> usize { 2 }
    
    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,_outputs:&mut Vec<Option<RenderResourceId>>) {
            let mut command = ctx.command_encoder.take().unwrap();
            if let Err(err) = self.draw(world,&mut command,inputs,ctx) {
                log::error!("pass node error:{:?}",err);
            }
            ctx.command_encoder = Some(command);

        /* 
        let target_view = &inputs[0];
        let depth_view = &inputs[1];
        if depth_view.is_none() {
            return;
        }
     
       
        
        if let Some(view_id) = target_view.as_ref() {   
            let mut camera_query = world.query::<(Entity, &Camera)>();
            let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
            let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
            let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
            let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
            let mats = mat_storages.mateials.read();
            
            let view = ctx.resources.get_texture_view_by_resid(view_id);
            if view.is_none() {
                ctx.command_encoder = Some(command);
                return; 
            }
            let view = view.unwrap();
            
            let mut render_pass = command.begin_render_pass(&wgpu::RenderPassDescriptor {
                label:None,
                color_attachments:&[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target:None,
                    ops:self.operations
                }],
                depth_stencil_attachment:if let Some(depth_view) = depth_view {
                    Some(wgpu::RenderPassDepthStencilAttachment {
                        view:ctx.resources.get_texture_view_by_resid(depth_view).unwrap(),
                        stencil_ops: None,
                        depth_ops: Some(Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                    })
                } else {
                   None
                },
            });
            
            for (e,camera) in camera_query.iter(world) {
                if let Some(camera_buffer)  = ctx.camera_state.cameras_buffer.buffers.get(&e.id()) {
                    for view_entites in camera.view_list.values.iter() {
                        for ve in view_entites.value.iter() {
                            if let Ok((re,hmesh,hmat))  = render_query.get(world, ve.entity) {
                                let maybe_mesh = meshs.get(&hmesh.id);
                                let mat = mats.get(&hmat.id).unwrap();
                                if !mat.is_ready(&ctx.resources) || maybe_mesh.is_none() {
                                    continue;
                                }
                                let mesh = maybe_mesh.unwrap();
                                if let Some(pipes) = pipeline_cahce.get_pipeline(&mat.def.name, mesh) {
                                    if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&hmesh.id, 0) {
                                        for pipe in pipes.pipelines.iter() {
                                            let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                                            if let Some(trans_info) = ctx.transform_buffer.get_info(&re.id()) {
                                                render_pass.set_bind_group(0, &camera_buffer.bind_group, &[]);
                                                render_pass.set_bind_group(1, &trans_info.bind_group, &[]);
                                                render_pass.set_bind_group(2, mat.bind_group.as_ref().unwrap(), &[]);
                                                if let Some(texture_bind_group) = mat.texture_props.bind_group.as_ref() {
                                                    render_pass.set_bind_group(3, texture_bind_group, &[]);
                                                }
                                                if mat.def.is_light {
                                                    render_pass.set_bind_group(4, ctx.light_state.bind_group.as_ref().unwrap(), &[]);
                                                }
                                                
                                                render_pass.set_vertex_buffer(0, vert_buffer.slice(0..));
                                                if let Some(idx_id) = ctx.resources.get_render_resource(&hmesh.id, 1) {
                                                    let idx_buffer = ctx.resources.get_buffer_by_resid(&idx_id).unwrap();
                                                    render_pass.set_index_buffer(idx_buffer.slice(0..), mesh.index_format().unwrap());
                                                    render_pass.set_pipeline(pipe);
                                                   
                                                    render_pass.draw_indexed(mesh.indices_range().unwrap(),0, 0..1);
                                                } else {
                                                    render_pass.set_pipeline(pipe);
                                                    
                                                    render_pass.draw(0..mesh.count_vertices() as u32, 0..1);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }   
            }
        }
        ctx.command_encoder = Some(command);*/
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

        let mut render_pass = command.begin_render_pass(&wgpu::RenderPassDescriptor {
            label:None,
            color_attachments:&[wgpu::RenderPassColorAttachment {
                view:target_view,
                resolve_target:None,
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

        let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
        let mut camera_query = world.query::<(Entity,&Transform,&Camera)>();
        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();


        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        let mats = mat_storages.mateials.read();
        
        for (_,_,camera) in camera_query.iter(world) {
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
                                //这里设置uniform

                                render_pass.set_vertex_buffer(0, vert_buffer.slice(0..));
                                if let Some(idx_id) = ctx.resources.get_render_resource(&hmesh.id, 1) {
                                    let idx_buffer = ctx.resources.get_buffer_by_resid(&idx_id).unwrap();
                                    render_pass.set_index_buffer(idx_buffer.slice(0..), mesh.index_format().unwrap());
                                    render_pass.set_pipeline(pipeline);
                    
                                    render_pass.draw_indexed(mesh.indices_range().unwrap(),0, 0..1);
                                } else {
                                    render_pass.set_pipeline(pipeline);
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
    MissMaterial
}