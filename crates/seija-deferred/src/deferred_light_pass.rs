use bevy_ecs::prelude::{World, Entity};

use seija_asset::Handle;
use seija_render::{graph::INode, RenderContext, resource::{RenderResourceId, Texture}, 
material::{MaterialStorage, Material}, errors::RenderErrors};
use anyhow::{Result,anyhow};
use crate::DeferredQuad;

pub struct DeferredLightPass {
    tex_count:usize,
    is_set_texs:bool
}

impl DeferredLightPass {
    pub fn new(tex_count:usize) -> Self {
        DeferredLightPass {
            tex_count,
            is_set_texs:false
        }
    }


    pub fn set_quad_texs(world:&World,textures:Vec<Handle<Texture>>,e_quad:Entity) -> Result<()> {
        let e_ref = world.entity(e_quad);
        let h_mat = e_ref.get::<Handle<Material>>().ok_or(anyhow!("not found Handle<Material>"))?;
        let mats = world.get_resource::<MaterialStorage>().ok_or(RenderErrors::NotFoundMaterialStorage)?;
        
        mats.material_mut(&h_mat.id, |mat| {
          mat.texture_props.set("positionTex", textures[0].clone());
          mat.texture_props.set("normalTex", textures[1].clone());
        });
        Ok(())
    }
    
    pub fn collect_textures(inputs:&Vec<Option<RenderResourceId>>) -> Option<Vec<Handle<Texture>>> {
        let mut ret = vec![];
        for input in inputs.iter() {
            if let Some(h_tex) = input.as_ref().and_then(|v| v.into_texture()) {
                ret.push(h_tex);
            } else { return None; }
        }
        Some(ret)
    }
}

impl INode for DeferredLightPass {
    fn input_count(&self) -> usize { self.tex_count + 1 }
    fn init(&mut self, _: &mut World, _ctx:&mut RenderContext) {}

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>) {
        if self.is_set_texs { return; }
        if let Some(e_quad) = world.get_resource::<DeferredQuad>().map(|v| v.0) {
            if let Some(textures) = Self::collect_textures(inputs) {
                if let Err(err) = Self::set_quad_texs(world,textures,e_quad) {
                    log::error!("{:?}",err);
                }
                self.is_set_texs = true;
            }
            
        }
        
    }
}