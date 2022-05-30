use bevy_ecs::prelude::{World, Entity};
use glam::{Vec3, Mat4, Quat};
use seija_core::bytes::AsBytes;
use anyhow::{Result};
use seija_core::LogOption;
use crate::{graph::INode, resource::RenderResourceId, camera::camera::{Orthographic, Projection}, UBONameIndex};

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

    fn prepare(&mut self, world: &mut World, ctx:&mut crate::RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,
              ctx:&mut crate::RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
       if let Err(err) = self.draw(world,ctx) {
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

    pub fn draw(&mut self,world:&mut World,ctx:&mut crate::RenderContext) -> Result<()> {
        if let Some(shadow_light) = world.get_resource::<ShadowLight>() {
            if self.last_dir != shadow_light.directon {
                self.last_dir = shadow_light.directon.clone();
                self.draw_inner(world,shadow_light.directon,ctx)?;
               
            }
        }
        Ok(())
    }

    pub fn draw_inner(&mut self,world:&mut World,dir:Vec3,ctx:&mut crate::RenderContext) -> Result<()> {
        self.set_ubo(dir,ctx);
        
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