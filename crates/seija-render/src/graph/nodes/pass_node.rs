use crate::{camera::camera::Camera, graph::node::INode, material::{Material, MaterialStorage}, pipeline::{PipelineCache, RenderPipelines}, render::{RenderContext}, resource::Mesh};
use bevy_ecs::prelude::*;

use seija_asset::{Assets, Handle};
use wgpu::{Color, CommandEncoder, Device, Operations, TextureView};
use crate::resource::RenderResourceId;
pub struct PassNode {
    operations:Operations<Color>
}

impl INode for PassNode {
    fn input_count(&self) -> usize { 1 }
    
    fn update(&mut self,world: &mut World,
              ctx:&mut RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              _outputs:&mut Vec<Option<RenderResourceId>>) {
        
        let target_view = &inputs[0];
        let mut command = ctx.command_encoder.take().unwrap();
        if let Some(view_id) = target_view.as_ref() {   
            let mut camera_query = world.query::<(Entity, &Camera)>();
            let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
            let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
            let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
            let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
            let mats = mat_storages.mateials.read();
            
            
            let view = ctx.resources.get_texture_view(view_id).unwrap();
            let mut render_pass = command.begin_render_pass(&wgpu::RenderPassDescriptor {
                label:None,
                color_attachments:&[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target:None,
                    ops:self.operations
                }],
                depth_stencil_attachment:None,
            });
            
            for (_,camera) in camera_query.iter(world) {
                for view_entites in camera.view_list.values.iter() {
                    for ve in view_entites.value.iter() {
                        if let Ok((_,hmesh,hmat))  = render_query.get(world, ve.entity) {
                            let mesh = meshs.get(&hmesh.id).unwrap();
                            let mat = mats.get(&hmat.id).unwrap();
                            if let Some(pipes) = pipeline_cahce.get_pipeline(&mat.def.name, mesh) {
                                if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(hmesh.clone_weak_untyped(), 0) {
                                    for pipe in pipes.pipelines.iter() {
                                        
                                        let idx_id = ctx.resources.get_render_resource(hmesh.clone_weak_untyped(), 1).unwrap();
                                        let vert_buffer = ctx.resources.get_buffer(&mesh_buffer_id).unwrap();
                                        let idx_buffer = ctx.resources.get_buffer(&idx_id).unwrap();

                                        
                                        
                                        render_pass.set_vertex_buffer(0, vert_buffer.slice(0..));
                                        render_pass.set_index_buffer(idx_buffer.slice(0..), mesh.index_format().unwrap());
                                        render_pass.set_pipeline(pipe);

                                        render_pass.draw_indexed(mesh.indices_range().unwrap(),0, 0..1)
                                    }
                                }
                            }
                        }
                    }
                }   
            }
        }
        ctx.command_encoder = Some(command);
    }
}

impl PassNode {
    pub fn new() -> PassNode {
        PassNode {
            operations:Operations { load:wgpu::LoadOp::Clear(wgpu::Color{r:0.5,g:0.1,b:0.1,a:0.0}), store:true }
        }
    }
    

    
}