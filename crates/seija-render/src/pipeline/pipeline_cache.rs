use std::hash::{Hash, Hasher};

use fnv::{FnvHashMap, FnvHasher};
use wgpu::{Device, RenderPipeline};

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
        
        for pass in  mat_def.pass_list.iter() {
            
        }
    }

    fn compile_pipeline(&self,pass:&PassDef) {

    }
}