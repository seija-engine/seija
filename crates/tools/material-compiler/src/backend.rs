use std::{collections::{HashMap, hash_map::DefaultHasher}, fmt::Write, sync::Arc, hash::{Hash, Hasher}};
use glsl_pack_rtbase::shader::Shader;
use glsl_pkg::{IShaderBackend, backends::{BackendItem, Backends}};

use seija_render::{UniformInfo, RawUniformInfo,MemUniformInfo,UniformType, UniformBufferDef, material::TexturePropDef};
use smol_str::SmolStr;

use crate::render_info::RenderInfo;

pub struct SeijaShaderBackend {
    pub render_info:RenderInfo,
    vertexs:HashMap<SmolStr,(usize,SmolStr)>
}

impl SeijaShaderBackend {
    pub fn init(&mut self,config_path:&str) {
        self.render_info.run(config_path);
    }
    pub fn new() -> Self {
        let mut vertexs = HashMap::new();
        vertexs.insert("POSITION".into(), (0,"vec3".into()));
        vertexs.insert("UV0".into(), (1,"vec2".into()));
        vertexs.insert("UV1".into(), (2,"vec2".into()));
        vertexs.insert("NORMAL".into(), (3,"vec3".into()));
        vertexs.insert("TANGENT".into(), (4,"vec4".into()));
        vertexs.insert("COLOR".into(), (5,"vec4".into()));
        vertexs.insert("JOINTS".into(), (6,"uvec4".into()));
        vertexs.insert("WEIGHTS".into(), (7,"vec4".into()));
        vertexs.insert("INDEX0".into(), (8,"int".into()));
        SeijaShaderBackend { vertexs,render_info:RenderInfo::new() }
    }
}


#[derive(Debug)]
pub struct ShaderTask {
   pub pkg_name:String,
   pub shader_name:String,
   pub macros:Arc<Vec<SmolStr>>,
   pub backends:Vec<SmolStr>,
   pub prop_def:Arc<UniformBufferDef>,
   pub tex_prop_def:Arc<TexturePropDef>,
   pub slots:HashMap<String,String>
}

impl ShaderTask {
    pub fn hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.pkg_name.hash(&mut hasher);
        self.shader_name.hash(&mut hasher);
        self.macros.hash(&mut hasher);
        hasher.finish()
    }
}


impl IShaderBackend for SeijaShaderBackend {
    type ExData = ShaderTask;
    fn write_common_head<W:Write>(&self, writer:&mut W) {
        writer.write_str("#version 450\r\n").unwrap();
        writer.write_str("#extension GL_EXT_samplerless_texture_functions : enable\r\n").unwrap();
        
    }

    fn write_fs_head<W:Write>(&self, _writer:&mut W) {
        //writer.write_str("layout(location = 0) out vec4 _outColor;\r\n").unwrap();
      
    }

    fn vertex_names(&self) -> &HashMap<SmolStr,(usize,SmolStr)> {
       &self.vertexs
    }


    fn write_uniforms<W:Write>(&self, writer:&mut W,_:&Shader,ex_data:&ShaderTask) {
        let mut ubos:HashMap<String,Arc<UniformInfo>> = Default::default();
        for backend_name in ex_data.backends.iter() {
            if let Some(ubo_info) = self.render_info.backend2ubo.get(backend_name) {
                if !ubos.contains_key(ubo_info.name.as_str()) {
                    ubos.insert(ubo_info.name.to_string(),ubo_info.clone());
                }
            }
        }
        let mut ubo_list = ubos.values().collect::<Vec<_>>();
        ubo_list.sort_by(|a,b| a.sort.cmp(&b.sort));
        let mut index:usize = 0;
        for ubo in ubo_list.iter() {
            write_ubo_uniform(&ubo,writer,index);
            index += 1;
        }
        writer.write_str("\r\n").unwrap();
        if ex_data.prop_def.infos.len() > 0 {
            writer.write_str(&format!("layout(set = {}, binding = 0) uniform Material {{\r\n",index)).unwrap();
            for prop in ex_data.prop_def.infos.iter() {
                if let MemUniformInfo::Raw(raw) = prop {
                    write_ubo_uniform_prop(&raw, writer);
                }
            }
            writer.write_str("} material;\r\n").unwrap();
            index += 1;
        }
        
        if ex_data.tex_prop_def.indexs.len() > 0 {
            for (name,info) in ex_data.tex_prop_def.indexs.iter() {
                if !info.is_cube_map {
                    writer.write_str(&format!("layout(set = {}, binding = {}) uniform texture2D tex_{};\r\n",index,info.index * 2,name)).unwrap();
                    writer.write_str(&format!("layout(set = {}, binding = {}) uniform sampler tex_{}Sampler;\r\n",index,info.index * 2 + 1,name)).unwrap();
                } else {
                    writer.write_str(&format!("layout(set = {}, binding = {}) uniform textureCube tex_{};\r\n",index,info.index * 2,name)).unwrap();
                    writer.write_str(&format!("layout(set = {}, binding = {}) uniform sampler tex_{}Sampler;\r\n",index,info.index * 2 + 1,name)).unwrap();
                }
            }
        }

    }

    fn write_backend_trait<W:Write>(&self, write:&mut W, _:&Shader, backends:&Backends,task:&ShaderTask) {
        for backend_name in task.backends.iter() {
            if let Some(backend) = backends.values.get(backend_name.as_str()) {
                for fn_info in backend.fns.iter() {
                    self.write_backend_trait_fn(write,fn_info,&backend_name);                
                }
            } else {
                log::error!("not found backend {}",backend_name);
            }
        }
    }
 
    fn write_vs_slots<W:Write>(&self,write:&mut W,shader:&Shader,ex_data:&ShaderTask) {
        for slot in shader.slots.iter() {
            if slot.starts_with("slot_vs") {
                if let Some(slot_fn) = ex_data.slots.get(slot) {
                    write.write_str(&format!("\r\n{}\r\n",slot_fn)).unwrap();
                }
            }
        }
    }

    fn write_fs_slots<W:Write>(&self,write:&mut W,shader:&Shader,ex_data:&ShaderTask) {
        for slot in shader.slots.iter() {
            if slot.starts_with("slot_fs") {
                if let Some(slot_fn) = ex_data.slots.get(slot) {
                    write.write_str(&format!("\r\n{}\r\n",slot_fn)).unwrap();
                }
            }
        }
    }
}

impl SeijaShaderBackend {
    fn write_backend_trait_fn<W:Write>(&self,writer:&mut W,fn_info:&BackendItem,backend_name:&str) {
        writer.write_str("\r\n").unwrap();
        let mut new_name = fn_info.name.clone();
        if let Some(r) = new_name.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
        if let Some(array_name) = fn_info.array_name.as_ref() {
            if let Some(ubo_info) = self.render_info.backend2ubo.get(backend_name) {
                let mut new_array_name = array_name.clone();
                if let Some(r) = new_array_name.get_mut(0..1) {
                    r.make_ascii_uppercase();
                }
                writer.write_str(&format!("{} get{}{}(int index){{",fn_info.typ,new_array_name,new_name)).unwrap();
                writer.write_str(&format!("return _{}.{}[index].{};",&ubo_info.name,array_name,fn_info.name)).unwrap();
                writer.write_str("}\r\n").unwrap();
            }
        } else {
            if fn_info.typ == "texture2D" {
               
                writer.write_str(&format!("vec4 texture_{}(vec2 uv){{",new_name)).unwrap();
                if let Some(ubo_info) = self.render_info.backend2ubo.get(backend_name) {
                    let low_name = ubo_info.name.to_lowercase();
                    writer.write_str(&format!("return texture(sampler2D({}_{},{}_{}S),uv);",&low_name,fn_info.name,&low_name,fn_info.name)).unwrap();
                }
                writer.write_str("}\r\n").unwrap();
                
                writer.write_str(&format!("ivec2  textureSize_{}(){{",new_name)).unwrap();
                if let Some(ubo_info) = self.render_info.backend2ubo.get(backend_name) {
                    let low_name = ubo_info.name.to_lowercase();
                    writer.write_str(&format!("return textureSize({}_{},0);",&low_name,fn_info.name)).unwrap();
                }
                writer.write_str("}\r\n").unwrap();
               
            } else if  fn_info.typ == "cubeMap" {

            } else if fn_info.typ == "texture2DArray" {
                
            } else {
                writer.write_str(&format!("{} get{}(){{",fn_info.typ,new_name)).unwrap();
                if let Some(ubo_info) = self.render_info.backend2ubo.get(backend_name) {
                    writer.write_str(&format!("return _{}.{};",&ubo_info.name,fn_info.name)).unwrap();
                }
                writer.write_str("}\r\n").unwrap();
            }
            
        }
    }
}


fn write_ubo_uniform<W:Write>(info:&UniformInfo, writer:&mut W,index:usize) {

    for prop in info.props.infos.iter() {
        if let MemUniformInfo::Array(arr) = prop {
            writer.write_str("\r\n").unwrap();
            writer.write_str(&format!("struct {}{} {{\r\n",&info.name,arr.name)).unwrap();
            for prop in arr.elem_def.infos.iter() {
                if let MemUniformInfo::Raw(raw) = prop {
                    write_ubo_uniform_prop(&raw, writer);
                }
            }
            writer.write_str("};\r\n").unwrap();
        }
    }
    writer.write_str("\r\n").unwrap();
    let mut binding_index = 0;
    if info.props.infos.len() > 0 {
        writer.write_str(&format!("layout(set = {}, binding = 0) uniform {} {{\r\n",index,&info.name)).unwrap();
        binding_index += 1;
    }
    for prop in info.props.infos.iter() {
        match prop {
            MemUniformInfo::Raw(raw) => {
                write_ubo_uniform_prop(&raw, writer);
            },
            MemUniformInfo::Array(arr) => {
                writer.write_str(&format!("  {}{} {}[{}];\r\n",&info.name,&arr.name,&arr.name,arr.array_size)).unwrap();
            }
        }
    }
    if info.props.infos.len() > 0 {
        writer.write_str(&format!("}} _{};\r\n",&info.name)).unwrap();
    }

    
    for texture_prop in info.textures.iter() {
        let low_name = info.name.to_lowercase();
        if texture_prop.is_cubemap {
            writer.write_str(&format!("layout(set = {}, binding = {}) uniform textureCube {}_{};\r\n",index,binding_index,&low_name,&texture_prop.name)).unwrap();
        } else {
            if texture_prop.str_type.as_str() == "texture2DArray"  {
                writer.write_str("//skip texture array\r\n").unwrap();
            } else {
                writer.write_str(&format!("layout(set = {}, binding = {}) uniform {} {}_{};\r\n",index,binding_index,&texture_prop.str_type,&low_name,&texture_prop.name)).unwrap();
            }
        }
        binding_index += 1;
        writer.write_str(&format!("layout(set = {}, binding = {}) uniform sampler {}_{}S;\r\n",index,binding_index,&low_name,&texture_prop.name)).unwrap();
        binding_index += 1;
    }
}

fn write_ubo_uniform_prop<W:Write>(prop:&RawUniformInfo,writer:&mut W) {
    let typ_name = match prop.typ {
        UniformType::BOOL(_)   => "bool",
        UniformType::FLOAT(_)  => "float",
        UniformType::FLOAT3(_) => "vec3",
        UniformType::FLOAT4(_) => "vec4",
        UniformType::INT(_)    => "int",
        UniformType::UINT(_)   => "uint",
        UniformType::MAT3(_)   => "mat3",
        UniformType::MAT4(_)   => "mat4"
    };
    let full_type_name:String;
    if prop.size > 1 {
         full_type_name = format!("{}[{}]",typ_name,prop.size);
    } else {
         full_type_name = typ_name.to_string();
    }
    writer.write_str(&format!("  {} {};\r\n",full_type_name,prop.name)).unwrap();
}