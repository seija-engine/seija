use crate::{camera::camera::Camera, graph::node::INode, material::{Material, MaterialStorage}, pipeline::{PipelineCache, RenderPipelines}, render::{RenderContext}, resource::Mesh};
use bevy_ecs::prelude::*;
use seija_asset::{Assets, Handle};
use crate::resource::RenderResourceId;
pub struct PassNode;

impl INode for PassNode {
    fn input_count(&self) -> usize { 1 }
    
    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_inputs:&Vec<Option<RenderResourceId>>,_outputs:&mut Vec<Option<RenderResourceId>>) {
        let mut camera_query = world.query::<(Entity, &Camera)>();
        let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        let mats = mat_storages.mateials.read();
        for (_,camera) in camera_query.iter(world) {
            
            for view_entites in camera.view_list.values.iter() {
                for ve in view_entites.value.iter() {
                   if let Ok((_,hmesh,hmat))  = render_query.get(world, ve.entity) {
                        let mesh = meshs.get(&hmesh.id).unwrap();
                        let mat = mats.get(&hmat.id).unwrap();
                        if let Some(pipes) = pipeline_cahce.get_pipeline(&mat.def.name, mesh) {
                            self.draw_pipes(pipes,mesh,ctx);
                        }
                   }
                }
            }   
        }
    }
}

impl PassNode {
    pub fn new() -> PassNode {
        todo!()
    }
    fn draw_pipes(&self,pipes:&RenderPipelines,mesh:&Mesh,ctx:&mut RenderContext) {
       let command =  ctx.command_encoder.as_mut().unwrap();
       
    }
}