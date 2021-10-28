use core::slice;

use std::io;
use std::hash::{Hash, Hasher};
use std::fs;
use std::sync::Arc;
use fnv::{FnvHashMap, FnvHasher};
use wgpu::{BindGroupLayout, DepthStencilState, Device, 
          FragmentState, MultisampleState, PipelineLayout, 
          PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, 
          RenderPipelineDescriptor, ShaderModule, 
          ShaderModuleDescriptor, ShaderStage, StencilState, VertexState};
use crate::RenderContext;
use crate::{material::{MaterialDef, PassDef}, resource::Mesh};

use super::render_bindings::RenderBindGroupLayout;



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

    pub fn check_build(&mut self,mesh:&Mesh,mat_def:&MaterialDef,ctx:&RenderContext) {
        let mut hasher = FnvHasher::default();
        PipelineKey(&mat_def.name,mesh.layout_hash_u64()).hash(&mut hasher);
        let key = hasher.finish();
        if !self.cache_pipelines.contains_key(&key) {
            let pipes = self.compile_pipelines(mesh, mat_def,ctx);
            self.cache_pipelines.insert(key, pipes);
        }
    }

    fn compile_pipelines(&mut self,mesh:&Mesh,mat_def:&MaterialDef,ctx:&RenderContext) -> RenderPipelines {
        let prim_state = mesh.primitive_state();
        let mut pipes:Vec<RenderPipeline> = Vec::new();
        for pass in  mat_def.pass_list.iter() {
           if let Some(pipe) = self.compile_pipeline(mesh,pass, &prim_state,ctx) {
               pipes.push(pipe);
           }
        }
        RenderPipelines::new(pipes)
    }

    fn compile_pipeline(&mut self,mesh:&Mesh,pass:&PassDef,mesh_prim_state:&PrimitiveState,ctx:&RenderContext) -> Option<RenderPipeline> {
        let mut cur_primstate = mesh_prim_state.clone();
        cur_primstate.cull_mode = (&pass.cull).into();
        cur_primstate.front_face = pass.front_face.0;
        cur_primstate.clamp_depth = pass.clamp_depth;
        cur_primstate.polygon_mode = pass.polygon_mode.0;
        cur_primstate.conservative = pass.conservative;
        /* 
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
        };*/

       let vert_shader = Self::read_shader_module(&pass.vs_path,&ctx.device)?;
       let frag_shader = Self::read_shader_module(&pass.fs_path,&ctx.device)?;

       
      let pipeline_layout = self.create_pipeline_layout(ctx);

      let targets = vec![wgpu::ColorTargetState {
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        }),
        write_mask: wgpu::ColorWrite::ALL,
    }];

       let render_pipeline_desc = RenderPipelineDescriptor {
           label:None,
           layout:Some(&pipeline_layout),
           vertex:VertexState {  module:&vert_shader, entry_point:"main", buffers:&[mesh.vert_layout()] },
           primitive:cur_primstate,
           depth_stencil:None,
           multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
           fragment:Some(FragmentState { module:&frag_shader, entry_point:"main", targets:&targets })
       };
       let render_pipeline = ctx.device.create_render_pipeline(&render_pipeline_desc);
       Some(render_pipeline)
    }

    fn create_pipeline_layout(&mut self,ctx:&RenderContext) -> PipelineLayout {
        let camer_layout = ctx.camera_state.camera_layout.layout.as_ref().unwrap();;
        let layout_desc = PipelineLayoutDescriptor {
            label:None,
            bind_group_layouts:&[camer_layout],
            push_constant_ranges:&[],
        };
        ctx.device.create_pipeline_layout(&layout_desc)
    }

    fn read_shader_module(path:&str,device:&Device) -> Option<ShaderModule> {
       let code_bytes = fs::read(path).ok()?;
       let bytes = read_spirv(std::io::Cursor::new(&code_bytes)).unwrap();
       
       let shader_module = device.create_shader_module(&ShaderModuleDescriptor {
        label:None,
        source:wgpu::ShaderSource::SpirV(bytes.into()),
        flags:Default::default()
       });
       Some(shader_module)
    }

   
}

pub fn read_spirv<R: io::Read + io::Seek>(mut x: R) -> io::Result<Vec<u32>> {
    let size = x.seek(io::SeekFrom::End(0))?;
    if size % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input length not divisible by 4",
        ));
    }
    if size > usize::max_value() as u64 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "input too long"));
    }
    let words = (size / 4) as usize;
    let mut result = Vec::<u32>::with_capacity(words);
    x.seek(io::SeekFrom::Start(0))?;
    unsafe {
        // Writing all bytes through a pointer with less strict alignment when our type has no
        // invalid bitpatterns is safe.
        x.read_exact(slice::from_raw_parts_mut(
            result.as_mut_ptr() as *mut u8,
            words * 4,
        ))?;
        result.set_len(words);
    }
    const MAGIC_NUMBER: u32 = 0x07230203;
    if result.len() > 0 && result[0] == MAGIC_NUMBER.swap_bytes() {
        for word in &mut result {
            *word = word.swap_bytes();
        }
    }
    if result.len() == 0 || result[0] != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input missing SPIR-V magic number",
        ));
    }
    Ok(result)
}