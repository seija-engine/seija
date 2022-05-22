use bevy_ecs::prelude::{World, Entity};
use seija_asset::Assets;
use seija_render::{graph::INode, RenderContext, resource::{RenderResourceId, shape::{Quad}, Mesh}, 
material::{MaterialStorage}, errors::RenderErrors};
use seija_transform::Transform;
use anyhow::{Result,anyhow};

pub struct DeferredLightPass {
    tex_count:usize
}

impl DeferredLightPass {
    pub fn new(tex_count:usize) -> Self {
        DeferredLightPass {
            tex_count
        }
    }

    pub fn create_quad(&self,world:&mut World) -> Result<Entity> {
        let mats = world.get_resource::<MaterialStorage>()
                                       .ok_or(RenderErrors::NotFoundMaterialStorage)?;
        let h_mat = mats.create_material("DeferredLightPass")
                                                .ok_or(anyhow!("create deferred mat error"))?;
        
        let quad_mesh:Mesh = Quad::new(2f32).into();
        let mut meshs = world.get_resource_mut::<Assets<Mesh>>()
                                             .ok_or(RenderErrors::NotFoundAssetsMesh)?;
        let h_quad = meshs.add(quad_mesh);
        let eid = world.spawn().insert(h_quad).insert(Transform::default()).insert(h_mat).id();
        
        Ok(eid)
    }
}

impl INode for DeferredLightPass {
    fn input_count(&self) -> usize { self.tex_count + 1 }
    fn init(&mut self, world: &mut World, _ctx:&mut RenderContext) {
       if let Err(err) = self.create_quad(world) {
           log::error!("{}",err);
       }
    }

    fn prepare(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
       
    }
}