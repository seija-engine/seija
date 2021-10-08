use fnv::FnvHashMap;
use wgpu::RenderPipeline;

use crate::{material::MaterialDef, resource::Mesh};

#[derive(Hash,PartialEq, Eq,Debug)]
pub struct  PipelineKey(String,u64);

pub struct PipelineCache {
    cache_pipelines:FnvHashMap<PipelineKey,RenderPipeline>
}

impl PipelineCache {
    pub fn get(&mut self,mesh:&Mesh,mat_def:&MaterialDef) {
        
    }
}