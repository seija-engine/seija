use bevy_ecs::{world::World, prelude::Entity, query::{Added, With}};
use glam::{Vec4, Vec3};
use lite_clojure_eval::Variable;
use lite_clojure_frp::FRPSystem;
use seija_transform::Transform;
use smol_str::SmolStr;
use crate::{RenderContext, uniforms::backends::Camera3DBackend, UniformIndex, camera::camera::Camera, memory::TypedUniformBuffer};
use anyhow::{Result,anyhow};
use super::{super::errors::Errors, IUpdateNode};

pub struct CameraNode {
    ubo_name:SmolStr,
    backend:Option<Camera3DBackend>,
    name_index:Option<UniformIndex>,
}

impl CameraNode {
    pub fn from_args(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let name = args.get(0).and_then(Variable::cast_string)
                              .ok_or(Errors::TypeCastError("string"))?;
        let br_names = name.borrow();
        Ok(Box::new(CameraNode { ubo_name:br_names.clone().into(),backend:None,name_index:None }))
    }
}

impl IUpdateNode for CameraNode {
    fn init(&mut self,_:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let info = ctx.ubo_ctx.info.get_info(&self.ubo_name).ok_or(Errors::NotFoundUBO(self.ubo_name.clone()))?;
        let backend = Camera3DBackend::from_def(&info.props).map_err(|v| anyhow!("camera3d backend err:{}",v.as_str()))?;
        self.backend = Some(backend);
       
        Ok(())
    }

    fn active(&mut self,_:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let name_index = ctx.ubo_ctx.get_index(self.ubo_name.as_str()).ok_or(anyhow!("err ubo name"))?;
        self.name_index = Some(name_index);
        Ok(())
    }

    fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        if let Some(name_index) = self.name_index.as_ref() {
            let mut added_cameras = world.query_filtered::<Entity,(Added<Camera>,With<Transform>)>(); 
            for v in added_cameras.iter(&world) {
                ctx.ubo_ctx.add_component(name_index, v, &mut ctx.resources);
            }
            for rm_e in world.removed::<Camera>() {
                ctx.ubo_ctx.remove_component(name_index, rm_e);
            }
        }

        let mut cameras = world.query::<(Entity,&Transform,&Camera)>();
        for (e,t,camera) in cameras.iter(world) {
            if let Some(key) = self.name_index {
                ctx.ubo_ctx.set_buffer(&key, Some(e), |buffer| {
                    self.update_camera_buffer(buffer, t, camera);
                })
            }
        }
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
      
        
        Ok(())
    }

   
}

impl CameraNode {
    

    fn update_camera_buffer(&self,buffer:&mut TypedUniformBuffer,t:&Transform,camera:&Camera) {
        if let Some(backend) = self.backend.as_ref() {
            let mut clone_global = t.global().clone();
            clone_global.scale = Vec3::ONE;
            let inv_global =  clone_global.matrix().inverse();
            //log::error!("camera clone_global: {:?}",clone_global);
            let proj = camera.projection.matrix();
            //log::error!("camera.projection: {:?}",camera.projection);
            let proj_view = proj * inv_global;
            let view = inv_global;
            let v3 = t.global().position;
            let pos = Vec4::new(v3.x,v3.y,v3.z,1f32);
            let buffer = &mut buffer.buffer;

            backend.set_view(buffer, &view);
            backend.set_proj(buffer, &proj);
            backend.set_projview(buffer, &proj_view);
            backend.set_position(buffer, pos);
        }
    }
}