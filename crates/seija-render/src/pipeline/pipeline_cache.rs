use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::fs;
use bevy_glsl_to_spirv::ShaderType;
use fnv::{FnvHashMap, FnvHasher};
use wgpu::{DepthStencilState, Device, FragmentState, MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, StencilState, VertexState};

use crate::{material::{MaterialDef, PassDef}, resource::Mesh};

#[derive(Hash,PartialEq, Eq,Debug)]
pub struct PipelineKey<'a>(&'a String,u64);

pub struct RenderPipelines {
   pub pipelines:Vec<RenderPipeline>
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

    pub fn get_pipeline(&self,def_name:&String,mesh:&Mesh) -> Option<&RenderPipelines> {
        let mut hasher = FnvHasher::default();
        PipelineKey(def_name,mesh.layout_hash_u64()).hash(&mut hasher);
        let key = hasher.finish();
        self.cache_pipelines.get(&key)
    }

    pub fn check_build(&mut self,mesh:&Mesh,mat_def:&MaterialDef,device:&Device) {
        let mut hasher = FnvHasher::default();
        PipelineKey(&mat_def.name,mesh.layout_hash_u64()).hash(&mut hasher);
        let key = hasher.finish();
        if !self.cache_pipelines.contains_key(&key) {
            let pipes = self.compile_pipelines(mesh, mat_def, device);
            self.cache_pipelines.insert(key, pipes);
        }
    }

    fn compile_pipelines(&self,mesh:&Mesh,mat_def:&MaterialDef,device:&Device) -> RenderPipelines {
        let prim_state = mesh.primitive_state();
        let mut pipes:Vec<RenderPipeline> = Vec::new();
        for pass in  mat_def.pass_list.iter() {
           if let Some(pipe) = self.compile_pipeline(mesh,pass, &prim_state,device) {
               dbg!(&pipe);
               pipes.push(pipe);
           }
        }
        RenderPipelines::new(pipes)
    }

    fn compile_pipeline(&self,mesh:&Mesh,pass:&PassDef,mesh_prim_state:&PrimitiveState,device:&Device) -> Option<RenderPipeline> {
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

       let vert_shader = Self::create_shader_module(&pass.vs_path,device,ShaderType::Vertex)?;
       let frag_shader = Self::create_shader_module(&pass.fs_path,device,ShaderType::Fragment)?;

       
      let pipeline_layout = Self::create_pipeline_layout(device);

       let render_pipeline_desc = RenderPipelineDescriptor {
           label:None,
           layout:Some(&pipeline_layout),
           vertex:VertexState {  module:&vert_shader, entry_point:"main", buffers:&[mesh.vert_layout()] },
           primitive:cur_primstate,
           depth_stencil:Some(depth_stencil),
           multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
           fragment:Some(FragmentState { module:&frag_shader, entry_point:"main", targets:&[] })
       };
       let render_pipeline = device.create_render_pipeline(&render_pipeline_desc);
       Some(render_pipeline)
    }

    fn create_pipeline_layout(device:&Device) -> PipelineLayout {
        let layout_desc = PipelineLayoutDescriptor {
            label:None,
            bind_group_layouts:&[],
            push_constant_ranges:&[],
        };
        device.create_pipeline_layout(&layout_desc)
    }

    fn create_shader_module(path:&str,device:&Device,ty:ShaderType) -> Option<ShaderModule> {
       let code_string = fs::read_to_string(path).ok()?;
       match bevy_glsl_to_spirv::compile(&code_string, ty, None) {
           Ok(spirv_bytes) => {
               let spirv: Cow<[u32]> = spirv_bytes.into();
               let shader_module = device.create_shader_module(&ShaderModuleDescriptor {
                  label:None,
                  source:wgpu::ShaderSource::SpirV(spirv),
                  flags:Default::default()
               });
              return Some(shader_module);
           },
           Err(err) => {
               eprintln!("path:{} err:{}",path,&err);
               return None;
           }
       }
    }

    fn compile_frag_shader<'a>(&self,pass:&PassDef) -> FragmentState<'a> {
        todo!()
    }
}