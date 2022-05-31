use bevy_ecs::prelude::{World, Entity};
use glam::{Vec3, Mat4, Quat};
use seija_asset::{Handle, Assets};
use seija_core::bytes::AsBytes;
use anyhow::{Result};
use seija_core::LogOption;
use seija_transform::Transform;
use crate::{graph::{INode, GraphError}, resource::{RenderResourceId, Mesh}, camera::camera::{Orthographic, Projection, Camera}, UBONameIndex, material::{Material, MaterialStorage}, pipeline::PipelineCache};

pub struct ShadowMapNode {
    pub ubo_name:String,
    name_index:Option<UBONameIndex>,
    proj_view_index:usize,
    orth_mat:Mat4,
    last_dir:Vec3
}


impl INode for ShadowMapNode {
    fn init(&mut self, _: &mut World, ctx:&mut crate::RenderContext) {
        if let Some(info) = ctx.ubo_ctx.info.get_info(self.ubo_name.as_str()) {
            self.proj_view_index = info.props.get_offset("lightProjView", 0)
                                       .log_err("not found lightProjView").unwrap_or(0);
            self.name_index = ctx.ubo_ctx.buffers.get_name_index(self.ubo_name.as_str());
        }
    }

    fn input_count(&self)  -> usize { 1 }
    fn output_count(&self) -> usize { 1 }

    fn prepare(&mut self, world: &mut World, ctx:&mut crate::RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,
              ctx:&mut crate::RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
        outputs[0] = inputs[0].clone();
       
       if let Err(err) = self.draw(world,ctx,inputs) {
           log::error!("shadow map error:{}",err);
       }
    }
}


impl ShadowMapNode {
    pub fn new(name:String) -> Self {
        ShadowMapNode {
            ubo_name:name,
            name_index:None,
            proj_view_index:0,
            last_dir:Vec3::ZERO,
            orth_mat:Mat4::orthographic_rh(-1000f32,1000f32,-1000f32,1000f32,0.1f32,1000f32)
        }
    }

    pub fn draw(&mut self,world:&mut World,ctx:&mut crate::RenderContext,inputs:&Vec<Option<RenderResourceId>>) -> Result<()> {
        
        if let Some(shadow_light) = world.get_resource::<ShadowLight>() {
            if self.last_dir != shadow_light.directon {
                self.last_dir = shadow_light.directon.clone();
                self.draw_inner(world,shadow_light.directon,ctx,inputs)?;
               
            }
        }
        Ok(())
    }

    pub fn draw_inner(&mut self,world:&mut World,dir:Vec3,ctx:&mut crate::RenderContext,inputs:&Vec<Option<RenderResourceId>>) -> Result<()> {
        self.set_ubo(dir,ctx);
        let texture_id = inputs[0].as_ref().ok_or(GraphError::ErrInput(0))?;
        if !ctx.resources.is_ready(texture_id) { return Ok(()); }
        let texture_view = ctx.resources.get_texture_view_by_resid(texture_id).ok_or(GraphError::ErrTargetView)?;

        let mut command = ctx.command_encoder.take().unwrap();
        let pass_desc = wgpu::RenderPassDescriptor {
            label:None,
            color_attachments:&[],
            depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {
                view:texture_view,
                stencil_ops:None,
                depth_ops:Some(wgpu::Operations {
                    load:  wgpu::LoadOp::Clear(1.0),
                    store: true,
                })
            })
        };
       
        let pass = command.begin_render_pass(&pass_desc);
        let mut camera_query = world.query::<(Entity,&Transform,&Camera)>();
        let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        let mats = mat_storages.mateials.read();

        for (_,camera_t,camera) in camera_query.iter(world) {
            for entity in camera.iter() {
                if let Ok((_,hmesh,hmat))  = render_query.get(world, *entity) {
                    let mesh = meshs.get(&hmesh.id).ok_or(GraphError::MissMesh)?;
                    let material = mats.get(&hmat.id).ok_or(GraphError::MissMaterial)?;
                    if !material.is_ready(&ctx.resources) { continue }
                    if let Some(pipelines)  = pipeline_cahce.get_pipeline(&material.def.name, mesh) {
                        for pipe in pipelines.pipelines.iter() {
                            if pipe.tag.as_ref().map(|v| v.as_str()) != Some("Shadow") { continue; }
                        }
                    }
                }
            }
        }
        Ok(())
    }


    fn set_ubo(&self,dir:Vec3,ctx:&mut crate::RenderContext) {
        //set ubo
        let mat4 = self.create_orth_mat(dir);
        if let Some(name_index) = self.name_index.as_ref() {
           let buffer = ctx.ubo_ctx.buffers.get_buffer_mut(name_index, None);
           if let Some(buffer) = buffer {
                buffer.buffer.write_bytes_(self.proj_view_index,  mat4.to_cols_array().as_bytes());
           }
        }
    }

    fn create_orth_mat(&self,dir:Vec3) -> Mat4 {
        let view = Mat4::look_at_rh(Vec3::ZERO, dir, Vec3::Y);
        return self.orth_mat * view;
    }
}


pub struct ShadowLight {
    directon:Vec3
}

impl ShadowLight {
    pub fn new(dir:Vec3) -> Self {
        ShadowLight {
            directon:dir.normalize()
        }
    }
}