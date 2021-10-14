use std::hash::{Hash, Hasher};

use fnv::{FnvHashMap, FnvHasher};
use wgpu::{DepthStencilState, Device, FragmentState, MultisampleState, PrimitiveState, RenderPipeline, StencilState};

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
    cache_pipelines:FnvHashMap<u64,RenderPipelines>
}

impl PipelineCache {
    pub fn check_build(&mut self,mesh:&Mesh,mat_def:&MaterialDef,device:&Device) {
        let mut hasher = FnvHasher::default();
        PipelineKey(&mat_def.name,mesh.layout_hash_u64()).hash(&mut hasher);
        let key = hasher.finish();
        if !self.cache_pipelines.contains_key(&key) {
            self.compile_pipelines(mesh, mat_def, device);
            self.cache_pipelines.insert(key, RenderPipelines::new(vec![]));
        }
    }

    fn compile_pipelines(&self,mesh:&Mesh,mat_def:&MaterialDef,device:&Device) {
        let prim_state = mesh.primitive_state();
        for pass in  mat_def.pass_list.iter() {
            self.compile_pipeline(pass, &prim_state,device);
        }
    }

    fn compile_pipeline(&self,pass:&PassDef,mesh_prim_state:&PrimitiveState,device:&Device) {
        let mut cur_primstate = mesh_prim_state.clone();
        cur_primstate.cull_mode = (&pass.cull).into();
        cur_primstate.front_face = pass.front_face.0;
        cur_primstate.clamp_depth = pass.clamp_depth;
        cur_primstate.polygon_mode = pass.polygon_mode.0;
        cur_primstate.conservative = pass.conservative;

        let depth_stencil = DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: pass.z_write,
            depth_compare: (&pass.z_test).into(),
            stencil: StencilState {
                front: wgpu::StencilFaceState::IGNORE,
                back: wgpu::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: wgpu::DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            }
        };

        let multisample = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };
    

        Self::create_shader_module(&pass.vs_path);
       /*
     
        pub layout: Option<&'a PipelineLayout>,
  
        pub vertex: VertexState<'a>,

        pub fragment: Option<FragmentState<'a>>,
       */

    }

    fn create_shader_module(path:&str) {
        
    }

    fn compile_frag_shader<'a>(&self,pass:&PassDef) -> FragmentState<'a> {
        todo!()
    }
}