use core::slice;

use std::collections::HashSet;
use std::io;
use std::hash::{Hash, Hasher};
use std::fs;
use std::path::Path;
use bevy_ecs::prelude::Entity;
use seija_core::{LogOption};
use std::sync::{Arc};
use fnv::{FnvHashMap, FnvHasher};
use glsl_pack_rtbase::MacroGroup;
use wgpu::{DepthStencilState, Device, 
          FragmentState, MultisampleState, PipelineLayout, 
          PipelineLayoutDescriptor, 
          RenderPipelineDescriptor, ShaderModule, 
          ShaderModuleDescriptor, StencilState, VertexState};
use crate::rt_shaders::RuntimeShaderInfo;
use crate::uniforms::{UBOApplyType, UniformContext};
use crate::{RenderContext, RenderConfig, GraphSetting, UniformIndex};
use crate::material::ShaderInfoDef;
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

pub struct RenderPipeline {
    //TODO 优化
    pub tag:Option<String>,
    pub ubos:Vec<UniformIndex>,
    pub pipeline:wgpu::RenderPipeline
}

impl RenderPipeline {
    pub fn set_binds<'b:'a,'a>(&self,camera_e:Entity,ve:&Entity,pass:&'a mut wgpu::RenderPass<'b>,buf_ctx:&'b UniformContext) -> Option<u32> {
        for (index,ubo_name_index) in self.ubos.iter().enumerate() {
            match ubo_name_index.apply_type {
             UBOApplyType::Camera => {
                let bind_group = buf_ctx.get_bind_group(ubo_name_index, Some(camera_e.id()))?;
                pass.set_bind_group(index as u32, &bind_group, &[]);
             },
             UBOApplyType::RenderObject => {
                let bind_group = buf_ctx.get_bind_group(ubo_name_index, Some(ve.id()))?;
                pass.set_bind_group(index as u32, &bind_group, &[]);
             },
             UBOApplyType::Frame => {
                let bind_group = buf_ctx.get_bind_group(ubo_name_index, None)?;
                pass.set_bind_group(index as u32, &bind_group, &[]);
             }
            }
        }
        
        Some(self.ubos.len() as u32)
    }
}

#[derive(Default)]
pub struct PipelineCache {
    cfg:Arc<RenderConfig>,
    cache_pipelines:FnvHashMap<u64,RenderPipelines>
}

impl PipelineCache {
    pub fn new(cfg:Arc<RenderConfig>) -> Self {
        PipelineCache { cache_pipelines: Default::default(),cfg }
    }
}


impl PipelineCache {


    pub fn get_pipeline(&self,def_name:&String,mesh:&Mesh) -> Option<&RenderPipelines> {
        let mut hasher = FnvHasher::default();
        PipelineKey(def_name,mesh.layout_hash_u64()).hash(&mut hasher);
        let key = hasher.finish();
        self.cache_pipelines.get(&key)
    }

    pub fn update(&mut self,mesh:&Mesh,mat_def:&MaterialDef,ctx:&RenderContext) {
        let mut hasher = FnvHasher::default();
        PipelineKey(&mat_def.name,mesh.layout_hash_u64()).hash(&mut hasher);
        let key = hasher.finish();
        if !self.cache_pipelines.contains_key(&key) {
            let pipes = self.compile_pipelines(mesh, mat_def,ctx);
            log::info!("create pipeline success {}",&mat_def.name);
            self.cache_pipelines.insert(key, pipes);
        }
    }

    

    fn compile_pipelines<'m>(&mut self,mesh:&Mesh,mat_def:&'m MaterialDef,ctx:&RenderContext) -> RenderPipelines {
        let mut pipes:Vec<RenderPipeline> = Vec::new();
      
        for pass in  mat_def.pass_list.iter() {
           if let Some(pipe) = self.compile_pipeline(mesh,pass,ctx,mat_def) {
               pipes.push(pipe);
           } else {
               log::error!("material compile pipeline fail {}",&mat_def.name);
           }
        }
        RenderPipelines::new(pipes)
    }

    fn compile_pipeline(&mut self,
                        mesh:&Mesh,pass:&PassDef,
                        ctx:&RenderContext,
                        mat_def:&MaterialDef) -> Option<RenderPipeline> {
        let mut cur_primstate = mesh.primitive_state().clone();
        cur_primstate.cull_mode = (&pass.cull).into();
        cur_primstate.front_face = pass.front_face.0;
        cur_primstate.clamp_depth = pass.clamp_depth;
        cur_primstate.polygon_mode = pass.polygon_mode.0;
        cur_primstate.conservative = pass.conservative;
        
       let depth_stencil = Some(DepthStencilState {
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
       });

       let shader_name_prefix = get_shader_name_prefix(mesh, &pass.shader_info,&ctx.shaders)
                                         .log_err(&format!("gen shader name prefix err:{}",&pass.shader_info.name))?;
       let vs_path = self.cfg.config_path.join(".render").join("shaders").join(&format!("{}.vert.spv",shader_name_prefix));
       let fs_path = self.cfg.config_path.join(".render").join("shaders").join(&format!("{}.frag.spv",shader_name_prefix));

       let vert_shader = Self::read_shader_module(&vs_path,&ctx.device)?;
       let frag_shader = Self::read_shader_module(fs_path,&ctx.device)?;
       
      let pipeline_layout = self.create_pipeline_layout(ctx,pass,mat_def).log_err("create pipeline layout fail")?;

      let targets = pass.get_color_targets();

       let render_pipeline_desc = RenderPipelineDescriptor {
           label:None,
           layout:Some(&pipeline_layout),
           vertex:VertexState {  module:&vert_shader, entry_point:"main", buffers:&[mesh.vert_layout()] },
           primitive:cur_primstate,
           depth_stencil,
           multisample: Self::get_multisample_state(&ctx.setting),
           fragment:Some(FragmentState { module:&frag_shader, entry_point:"main", targets:&targets })
       };
       let gpu_pipeline = ctx.device.create_render_pipeline(&render_pipeline_desc);

       let rt_shader = ctx.shaders.find_shader(&pass.shader_info.name)?;
       let ubo_names = ctx.ubo_ctx.info.get_ubos_by_backends(&rt_shader.backends);
       let mut ubos:Vec<UniformIndex> = vec![];
       for (ubo_name,_) in ubo_names.iter() {
           let name_index = ctx.ubo_ctx.get_index(ubo_name).log_err(&format!("not found ubo: {}",ubo_name))?;
           ubos.push(name_index);
       }
      
       let render_pipeline = RenderPipeline {
           tag:pass.tag.clone(),
           ubos,
           pipeline:gpu_pipeline
       };
       Some(render_pipeline)
    }

    fn get_multisample_state(setting:&GraphSetting) -> MultisampleState {
        if setting.msaa_samples > 1 {
            MultisampleState {
                count: setting.msaa_samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        } else {
            MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        }
    }

    fn create_pipeline_layout(&mut self,ctx:&RenderContext,pass_def:&PassDef,mat_def:&MaterialDef) -> Option<PipelineLayout> {
       
        let mut layouts = ctx.create_bind_group_layouts(pass_def)?;
        if mat_def.prop_def.infos.len() > 0 {
            layouts.push(&ctx.material_sys.layout);
        }
        if mat_def.tex_prop_def.indexs.len() > 0 {
            if let Some(texture_layout) = ctx.material_sys.texture_layouts.get(&mat_def.name) {
                layouts.push(texture_layout);
            }
        }
       
        let layout_desc = PipelineLayoutDescriptor {
            label:None,
            bind_group_layouts:&layouts,
            push_constant_ranges:&[],
        };
        Some(ctx.device.create_pipeline_layout(&layout_desc)) 
    }

    fn read_shader_module<P:AsRef<Path>>(path:P,device:&Device) -> Option<ShaderModule> {
       let code_bytes = fs::read(path.as_ref()).ok()?;
       let bytes = read_spirv(std::io::Cursor::new(&code_bytes)).unwrap();
       let shader_module = device.create_shader_module(&ShaderModuleDescriptor {
        label:None,
        source:wgpu::ShaderSource::SpirV(bytes.into()),
        flags:Default::default()
       });
       log::info!("create shader module {:?}",path.as_ref());
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


fn get_shader_name_prefix(mesh:&Mesh,shader:&ShaderInfoDef,shaders:&RuntimeShaderInfo) -> Option<String> {
    let shader_info = shaders.find_shader(&shader.name).log_err(&format!("not find shader in rt.json:{}",&shader.name))?;
    let mesh_types = mesh.mesh_attr_types().iter().map(|v| v.name()).collect::<HashSet<_>>();
    
    let mut macros:Vec<String> = vec![];
    for (s,is_require) in shader_info.verts.iter() {
        if mesh_types.contains(s.as_str()) {
            macros.push(format!("VERTEX_{}",s.clone()) );
        } else if *is_require {
            log::error!("mesh attrs:{:?}",&mesh_types);
            return None;
        }
    }
    
    let macro_group = MacroGroup::new(macros);
    let macro_string = macro_group.hash_base64();
    let sname = shader.name.clone().replace('.', "#");
    Some(format!("{}_{}",&sname,&macro_string))
}
