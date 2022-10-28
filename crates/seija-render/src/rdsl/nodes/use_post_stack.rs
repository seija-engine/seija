use bevy_ecs::{entity::Entity, prelude::World};
use lite_clojure_eval::Variable;
use anyhow::{anyhow,Result};
use crate::{IUpdateNode, RenderContext, rdsl::{atom::Atom, nodes::CommonError}, resource::RenderResourceId, PostEffectStack};

#[derive(Default)]
pub struct UsePostStack {
    camera_id:Option<Entity>,
    src_atom:Option<*mut Atom<RenderResourceId>>,
    dst_atom:Option<*mut Atom<RenderResourceId>>,
    param_dirty:bool
}

impl UsePostStack {
    pub fn build_params(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let camera_entity = self.camera_id.ok_or(anyhow!("camera_id is nil"))?;
        let maybe_posteffect = world.entity(camera_entity).get::<PostEffectStack>();
        if let Some(posteffect) = maybe_posteffect {
            
        }
        let src_resid:&RenderResourceId = unsafe { &*self.src_atom.unwrap() }.inner();
        let src_format = ctx.resources.get_texture_format(src_resid, world)
                                                     .ok_or(anyhow!("format err"))?;

        let dst_resid:&RenderResourceId = unsafe { &*self.dst_atom.unwrap() }.inner();
        let dst_format = ctx.resources.get_texture_format(dst_resid, world)
                                                     .ok_or(anyhow!("format err"))?;
        
        log::error!("src:{:?} dst:{:?}",src_format,dst_format);
        
        Ok(())
    }
}

impl IUpdateNode for UsePostStack {
    fn update_params(&mut self,params:Vec<Variable>) -> Result<()> {
        let camera_id = params[0].cast_int().ok_or(anyhow!("type cast error"))?;
        self.camera_id = Some(Entity::from_raw(camera_id as u32));
        let src_atom = params[1].cast_userdata()
                                                             .map(|v|v as *mut Atom<RenderResourceId>)
                                                             .ok_or(anyhow!("type cast error"))?;
        let dst_atom = params[2].cast_userdata()
                                                             .map(|v|v as *mut Atom<RenderResourceId>)
                                                             .ok_or(anyhow!("type cast error"))?;
        self.src_atom = Some(src_atom);
        self.dst_atom = Some(dst_atom);
        self.param_dirty = true;
        Ok(())
    }

    fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) {
        if !self.param_dirty { return; }
        self.param_dirty = false;
        if let Err(err) = self.build_params(world, ctx) {
            log::error!("build_params error:{:?}",err);
        }
    }

    fn update(&mut self, _world:&mut World, _ctx:&mut RenderContext) {
        
    }
}