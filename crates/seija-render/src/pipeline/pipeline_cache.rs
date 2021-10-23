use core::slice;

use std::io;
use std::hash::{Hash, Hasher};
use std::fs;
use fnv::{FnvHashMap, FnvHasher};
use wgpu::{BindGroupLayout, DepthStencilState, Device, 
          FragmentState, MultisampleState, PipelineLayout, 
          PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, 
          RenderPipelineDescriptor, ShaderModule, 
          ShaderModuleDescriptor, ShaderStage, StencilState, VertexState};
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

       let vert_shader = Self::read_shader_module(&pass.vs_path,device)?;
       let frag_shader = Self::read_shader_module(&pass.fs_path,device)?;

       
      let pipeline_layout = Self::create_pipeline_layout(device);

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
       let render_pipeline = device.create_render_pipeline(&render_pipeline_desc);
       Some(render_pipeline)
    }

    fn create_camera_bind_group_layout(device:&Device) -> BindGroupLayout {
        let camera_view_proj = wgpu::BindGroupLayoutEntry {
            binding:0,
            visibility:ShaderStage::VERTEX,
            ty:wgpu::BindingType::Buffer {
                ty:wgpu::BufferBindingType::Uniform,
                has_dynamic_offset:false,
                min_binding_size:None
            },
            count:None
        };

        let bind_group_layout_desc = &wgpu::BindGroupLayoutDescriptor {
            label:None,
            entries:&[camera_view_proj]
        };

        device.create_bind_group_layout(bind_group_layout_desc)
    }

    fn create_pipeline_layout(device:&Device) -> PipelineLayout {
        let bind_group_layout = Self::create_camera_bind_group_layout(device);
        let layout_desc = PipelineLayoutDescriptor {
            label:None,
            bind_group_layouts:&[],
            push_constant_ranges:&[],
        };
        device.create_pipeline_layout(&layout_desc)
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