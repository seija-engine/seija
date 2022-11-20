use anyhow::{Result};
use bevy_ecs::prelude::{Changed, Entity, Or, World};
use glam::{Mat4, Vec3};
use lite_clojure_eval::Variable;
use lite_clojure_frp::FRPSystem;
use seija_core::{OptionExt, bytes::AsBytes};
use seija_geometry::{calc_bound_sphere, proj_view_corners};
use seija_transform::Transform;
use smol_str::SmolStr;

use crate::{dsl_frp::IUpdateNode, UniformIndex, camera::camera::{Camera, Orthographic}};

use super::{recv_backend::ShadowRecvBackend, ShadowLight, ShadowCamera};

#[derive(Default)]
pub struct ShadowNode {
    ubo_cast_name: SmolStr,
    ubo_recv_name: SmolStr,
    recv_backend: ShadowRecvBackend,

    proj_view_index: usize,
    name_index: UniformIndex,
}

impl ShadowNode {
    pub fn from_args(args: Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let cast_name = args.get(0).and_then(Variable::cast_string).get()?;
        let ubo_cast_name: SmolStr = cast_name.borrow().as_str().into();
        let recv_name = args.get(1).and_then(Variable::cast_string).get()?;
        let ubo_recv_name: SmolStr = recv_name.borrow().as_str().into();

        let shadow_node = ShadowNode {
            ubo_cast_name,
            ubo_recv_name,
            ..Default::default()
        };

        Ok(Box::new(shadow_node))
    }
}

impl IUpdateNode for ShadowNode {
    fn active(
        &mut self,
        _world: &mut World,
        ctx: &mut crate::RenderContext,
        _: &mut FRPSystem,
    ) -> Result<()> {
        let info = ctx.ubo_ctx.info.get_info(&self.ubo_cast_name).get()?;
        self.proj_view_index = info.props.get_offset("projView", 0).get()?;
        self.name_index = ctx.ubo_ctx.get_index(self.ubo_cast_name.as_str()).get()?;
        self.recv_backend = ShadowRecvBackend::from_name(&self.ubo_recv_name, &ctx.ubo_ctx)?;
        Ok(())
    }

    fn update(
        &mut self,
        world: &mut World,
        ctx: &mut crate::RenderContext,
        _: &mut FRPSystem,
    ) -> Result<()> {
        let mut shadow_query = world.query_filtered::<(Entity,&Transform,&ShadowLight),
                                                                            Or<(Changed<Transform>,Changed<ShadowLight>)>>();
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
                let view = Mat4::look_at_rh(-p * (orth.far - orth.near) * 0.5f32,Vec3::ZERO, Vec3::Y);
                let light_proj_view = orth.proj_matrix() * view;

                self.recv_backend.set_bias(&mut ctx.ubo_ctx, shadow_light.bias);
                self.recv_backend.set_strength(&mut ctx.ubo_ctx, shadow_light.strength);
                ctx.ubo_ctx.set_buffer(&self.name_index, Some(e.id()), |buffer| {
                    buffer.buffer.write_bytes_(self.proj_view_index, light_proj_view.to_cols_array().as_bytes());
                });
            }
        }
        Ok(())
    }
}
