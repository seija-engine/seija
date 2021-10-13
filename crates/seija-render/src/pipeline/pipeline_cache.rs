use std::hash::{Hash, Hasher};

use fnv::{FnvHashMap, FnvHasher};
use wgpu::{Device, PrimitiveState, RenderPipeline};

use crate::{material::{MaterialDef, PassDef}, resource::Mesh};

#[derive(Hash,PartialEq, Eq,Debug)]
pub struct PipelineKey<'a>(&'a String,u64);

pub struct RenderPipelines {
    pipelines:Vec<RenderPipeline>
}

impl RenderPipelines {
    pub fn new(pipelines:Vec<RenderPipeline>) -> RenderPipelines {
        RenderPipelines { pipelines }
    }
}

#[derive(Default)]
pub struct PipelineCache {
    cache_pipelines:FnvHashMap<u64,RenderPipeline>
}

impl PipelineCache {
    pub fn check_build(&mut self,mesh:&Mesh,mat_def:&MaterialDef,device:&Device) {
        let mut hasher = FnvHasher::default();
        PipelineKey(&mat_def.name,mesh.layout_hash_u64()).hash(&mut hasher);
        let key = hasher.finish();
        if !self.cache_pipelines.contains_key(&key) {
            self.compile_pipelines(mesh, mat_def, device);
        }
    }

    fn compile_pipelines(&self,mesh:&Mesh,mat_def:&MaterialDef,device:&Device) {
        let prim_state = mesh.primitive_state();
        for pass in  mat_def.pass_list.iter() {
            self.compile_pipeline(pass, &prim_state);
        }
    }

    fn compile_pipeline(&self,pass:&PassDef,mesh_prim_state:&PrimitiveState) {
        let mut cur_primstate = mesh_prim_state.clone();
        cur_primstate.cull_mode = (&pass.cull).into();
        cur_primstate.front_face = pass.front_face.0;
        cur_primstate.clamp_depth = pass.clamp_depth;

        /*
           
            polygon_mode:wgpu::PolygonMode::Fill
            conservative:false
        */
    }
}