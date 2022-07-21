use bevy_ecs::prelude::{World, Entity, Changed, Or};
use lite_clojure_eval::Variable;
use anyhow::{Result,anyhow};
use seija_transform::Transform;
use crate::{IUpdateNode, RenderContext, UniformIndex, camera::camera::Orthographic};
use seija_core::bytes::AsBytes;
use super::ShadowLight;

#[derive(Default)]
pub struct ShadowNode {
    uniform_name:String,
    proj_view_index:usize,
    name_index:UniformIndex
}

impl IUpdateNode for ShadowNode {
    fn update_params(&mut self,params:Vec<Variable>) {
        if let Some(s) = params.get(0).and_then(Variable::cast_string) {
            self.uniform_name = s.borrow().clone();
        } else {
            log::error!("shadow node params error");
        }
    }

    fn init(&mut self,_:&World,ctx:&mut RenderContext) -> Result<()> {
        let info = ctx.ubo_ctx.info.get_info(&self.uniform_name).ok_or(anyhow!("not found info {}",&self.uniform_name))?;
        let proj_view_index = info.props.get_offset("projView", 0).ok_or(anyhow!("not found projView"))?;
        self.proj_view_index = proj_view_index;
        self.name_index = ctx.ubo_ctx.get_index(self.uniform_name.as_str()).ok_or(anyhow!("err ubo name {}",&self.uniform_name))?;
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        let mut shadow_query = world.query_filtered::<(Entity,&Transform,&ShadowLight),Or<(Changed<Transform>,Changed<ShadowLight>)>>();
        if let Some((e,t,_)) = shadow_query.iter(world).next() {
            let mut orth = Orthographic::default();
            orth.left = -100f32;
            orth.right = 100f32;
            orth.bottom = -100f32;
            orth.top = 100f32;
            let proj_view = orth.proj_matrix() * t.global().matrix().inverse();
            ctx.ubo_ctx.set_buffer(&self.name_index, Some(e.id()), |buffer| {
                buffer.buffer.write_bytes_(self.proj_view_index, proj_view.to_cols_array().as_bytes());
            });
        }
    }
}