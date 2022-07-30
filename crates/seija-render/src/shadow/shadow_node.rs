use bevy_ecs::prelude::{World, Entity, Changed, Or};
use glam::{Mat4, Vec3, Vec4};
use lite_clojure_eval::Variable;
use anyhow::{Result,anyhow};
use seija_geometry::{calc_bound_sphere, proj_view_corners};
use seija_transform::{Transform};
use smol_str::SmolStr;
use crate::{IUpdateNode, RenderContext, UniformIndex, camera::camera::{Orthographic, Camera}};
use seija_core::bytes::AsBytes;
use super::{ShadowLight, ShadowCamera, recv_backend::ShadowRecvBackend};

#[derive(Default)]
pub struct ShadowNode {
    ubo_cast_name:SmolStr,
    ubo_recv_name:SmolStr,
    recv_backend:ShadowRecvBackend,

    proj_view_index:usize,
    name_index:UniformIndex
}

impl IUpdateNode for ShadowNode {
    fn update_params(&mut self,params:Vec<Variable>) {
        if let Some(s) = params.get(0).and_then(Variable::cast_string) {
            self.ubo_cast_name = SmolStr::new(s.borrow().as_str());
        } else {
            log::error!("shadow node params 0 error");
        }

        if let Some(s) = params.get(1).and_then(Variable::cast_string) {
            self.ubo_recv_name = SmolStr::new(s.borrow().as_str());
        } else {
            log::error!("shadow node params 1 error");
        }
    }

    fn init(&mut self,_:&World,ctx:&mut RenderContext) -> Result<()> {
        //cast
        let info = ctx.ubo_ctx.info.get_info(&self.ubo_cast_name).ok_or(anyhow!("not found info {}",&self.ubo_cast_name))?;
        let proj_view_index = info.props.get_offset("projView", 0).ok_or(anyhow!("not found projView"))?;
        self.proj_view_index = proj_view_index;
        self.name_index = ctx.ubo_ctx.get_index(self.ubo_cast_name.as_str()).ok_or(anyhow!("err ubo name {}",&self.ubo_cast_name))?;

        //recv
        self.recv_backend = ShadowRecvBackend::from_name(&self.ubo_recv_name,&ctx.ubo_ctx)?;
        
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        let mut shadow_query = world.query_filtered::<(Entity,&Transform,&ShadowLight),Or<(Changed<Transform>,Changed<ShadowLight>)>>();
        let mut shadow_camera = world.query::<(&Camera,&Transform,&ShadowCamera)>();
        if let Some((c,camera_t,_)) = shadow_camera.iter(world).next() {
            let proj = c.projection.matrix();
            let view = camera_t.global().matrix().inverse();
            let proj_view = proj * view;
            let frustum_pts = proj_view_corners(&proj_view);
            
            let sphere = calc_bound_sphere(frustum_pts);
           
            let mut orth = Orthographic::default();
            orth.left = -sphere.radius;
            orth.right = sphere.radius;
            orth.top = sphere.radius;
            orth.bottom = -sphere.radius;
            orth.far = sphere.radius;
            orth.near = 0.01f32;
            
           

            if let Some((e,t,shadow_light)) = shadow_query.iter(world).next() {
                let p = t.global().rotation * Vec3::Z;
                //dbg!(p);
                let view = Mat4::look_at_rh(-p * (orth.far - orth.near) * 0.5f32,Vec3::ZERO, Vec3::Y);
                let light_proj_view = orth.proj_matrix() * view;
                //debug
                dbg!(-p * 5f32);
                let (s,r,p) = view.to_scale_rotation_translation();
                let rr = r.to_euler(glam::EulerRot::XYZ);
                log::error!("r:{:?} p:{:?}",(rr.0.to_degrees(),rr.1.to_degrees(),rr.2.to_degrees()),p);
                dbg!(&proj_view_corners(&orth.proj_matrix()));
                log::error!("shadow debug {:?} {:?} {}",&orth,&sphere,&light_proj_view);
                
                self.recv_backend.set_bias(&mut ctx.ubo_ctx, shadow_light.bias);
                self.recv_backend.set_strength(&mut ctx.ubo_ctx, shadow_light.strength);
                ctx.ubo_ctx.set_buffer(&self.name_index, Some(e.id()), |buffer| {
                    buffer.buffer.write_bytes_(self.proj_view_index, light_proj_view.to_cols_array().as_bytes());
                });
            }
        }
    }
}