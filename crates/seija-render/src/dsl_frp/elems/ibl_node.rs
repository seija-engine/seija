use bevy_ecs::{world::World, system::Resource};
use lite_clojure_eval::Variable;
use lite_clojure_frp::FRPSystem;
use seija_asset::Handle;
use seija_core::OptionExt;
use smol_str::SmolStr;
use anyhow::Result;
use crate::{dsl_frp::errors::Errors, RenderContext, UniformIndex, resource::Texture};

use super::IUpdateNode;
#[derive(Resource)]
pub struct IBLEnv {
   pub irradiance_map:Option<Handle<Texture>>,
   pub specular_map:Option<Handle<Texture>>,
   pub brdf_lut:Option<Handle<Texture>>
}

pub struct IBLNode {
    ubo_name:SmolStr,
    index:Option<UniformIndex>
}

impl IBLNode {
    pub fn from_args(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let name = args.get(0)
                                        .and_then(Variable::cast_string)
                                        .ok_or(Errors::TypeCastError("string"))?;
        let br_name = name.borrow();
        Ok(Box::new(IBLNode {
            ubo_name:br_name.as_str().into(),
            index:None
        }))
    }
}

impl IUpdateNode for IBLNode {
    fn active(&mut self,_world:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let env_index = ctx.ubo_ctx.get_index(&self.ubo_name).get()?;
        self.index = Some(env_index);
        Ok(())
    }

    fn update(&mut self,_world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        Ok(())
    }

    fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let is_ibl_dirty = world.is_resource_changed::<IBLEnv>();
        if !is_ibl_dirty { return Ok(()); }
        if let Some(env) = world.get_resource::<IBLEnv>() {
            if let Some(diff_map) = env.irradiance_map.as_ref() {
                ctx.ubo_ctx.set_texture_byindex(None, self.index.as_ref().get()?, "irradianceMap", diff_map.clone_weak())?;
            }
            if let Some(specular_map) = env.specular_map.as_ref() {
                ctx.ubo_ctx.set_texture_byindex(None, self.index.as_ref().get()?, "prefilterMap", specular_map.clone_weak())?;
            }
            if let Some(brdf_lut) = env.brdf_lut.as_ref() {
                ctx.ubo_ctx.set_texture_byindex(None, self.index.as_ref().get()?, "brdfLUT", brdf_lut.clone_weak())?;
            }
        }
        Ok(())
    }
}