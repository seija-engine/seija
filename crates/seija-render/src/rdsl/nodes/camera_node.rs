use bevy_ecs::prelude::{World, Entity, Added, With};
use glam::Vec4;
use lite_clojure_eval::Variable;
use seija_transform::Transform;

use crate::{rdsl::node::IUpdateNode, UniformIndex, uniforms::backends::Camera3DBackend, RenderContext, camera::camera::Camera, memory::TypedUniformBuffer};
#[derive(Default)]
pub struct CameraNode {
    ubo_name:String,
    name_index:Option<UniformIndex>,
    backend:Option<Camera3DBackend>
}

impl IUpdateNode for CameraNode {
    fn update_params(&mut self,params:Vec<Variable>) {
        if let Some(string) = params.get(0).and_then(Variable::cast_string) {
            self.ubo_name = string.borrow().clone();
        }
    }

    fn init(&mut self,_:&mut World,ctx:&mut crate::RenderContext) {
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            match Camera3DBackend::from_def(&info.props) {
                Ok(backend) => {
                    self.backend = Some(backend)
                },
                Err(err) => {
                    log::error!("Camera3DBackend backend error :{}",err);
                }
            }
            self.name_index = Some(ctx.ubo_ctx.get_index(self.ubo_name.as_str()).unwrap())
         }
    }

    fn prepare(&mut self, world: &mut World,ctx:&mut RenderContext) {
        if let Some(name_index) = self.name_index.as_ref() {
            let mut added_cameras = world.query_filtered::<Entity,(Added<Camera>,With<Transform>)>(); 
            for v in added_cameras.iter(&world) {
                ctx.ubo_ctx.add_component(name_index, v.id(), &mut ctx.resources);
            }
    
            for rm_e in world.removed::<Camera>() {
                ctx.ubo_ctx.remove_component(name_index, rm_e.id());
            }
        }   
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext) {
        let mut cameras = world.query::<(Entity,&Transform,&Camera)>();
        for (e,t,camera) in cameras.iter(world) {
            if let Some(key) = self.name_index {
                ctx.ubo_ctx.set_buffer(&key, Some(e.id()), |buffer| {
                    self.update_camera_buffer(buffer, t, camera);
                })
            }
        }
    }
}

impl CameraNode {
    fn update_camera_buffer(&self,buffer:&mut TypedUniformBuffer,t:&Transform,camera:&Camera) {
        if let Some(backend) = self.backend.as_ref() {
            let proj = camera.projection.matrix();
            let proj_view = proj * t.global().matrix().inverse();
            let view = t.global().matrix().inverse();
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