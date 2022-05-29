use bevy_ecs::prelude::{World, Entity};
use glam::{Vec3, Mat4, Quat};
use anyhow::{Result};
use crate::{graph::INode, resource::RenderResourceId, camera::camera::{Orthographic, Projection}, UBONameIndex};

pub struct ShadowMapNode {
    pub ubo_name:String,
    name_index:Option<UBONameIndex>,
}

impl INode for ShadowMapNode {
    fn prepare(&mut self, world: &mut World, ctx:&mut crate::RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,
              ctx:&mut crate::RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
       if let Err(err) = self.draw(world) {
           log::error!("shadow map error:{}",err);
       }
    }
}


impl ShadowMapNode {
    pub fn draw(&self,world:&mut World) -> Result<()> {
        if let Some(shadow_light) = world.get_resource::<ShadowLight>() {
            self.draw_inner(world,shadow_light.directon)?;
        }
        Ok(())
    }

    pub fn draw_inner(&self,world:&mut World,dir:Vec3) -> Result<()> {
        let mat4 = Self::create_orth_mat(dir);

        Ok(())
    }

    fn create_orth_mat(dir:Vec3) -> Mat4 {
        let mat = Mat4::orthographic_rh(-1000f32,1000f32,-1000f32,1000f32,0.1f32,1000f32);
        let view = Mat4::look_at_rh(Vec3::ZERO, dir, Vec3::Y);
        return mat * view;
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