use std::{collections::{HashMap}, fmt::Write, sync::Arc};
use glsl_pack_rtbase::shader::Shader;
use glsl_pkg::{IShaderBackend, backends::BackendItem};

use seija_render::{UBOInfo, RawUniformInfo,UniformInfo,UniformType};

use crate::{render_info::RenderInfo, ShaderTask};

pub struct SeijaShaderBackend {
    pub render_info:RenderInfo,
    vertexs:HashMap<String,(usize,String)>
}

impl SeijaShaderBackend {
    pub fn init(&mut self,config_path:&str) {
        self.render_info.init();
        self.render_info.run(config_path);
    }
    pub fn new() -> Self {
        let mut vertexs = HashMap::new();
        vertexs.insert("POSITION".into(), (0,"vec3".into()));
        vertexs.insert("UV0".into(), (1,"vec2".into()));
        vertexs.insert("UV1".into(), (2,"vec2".into()));
        vertexs.insert("NORMAL".into(), (3,"vec3".into()));
        vertexs.insert("TANGENT".into(), (4,"vec3".into()));
        vertexs.insert("COLOR".into(), (5,"vec4".into()));
        SeijaShaderBackend { vertexs,render_info:RenderInfo::new() }
    }
}


impl IShaderBackend for SeijaShaderBackend {
    type ExData = ShaderTask;
    fn write_common_head<W:Write>(&self, writer:&mut W) {
        writer.write_str("#version 450\r\n").unwrap();
    }

    fn write_fs_head<W:Write>(&self, writer:&mut W) {
        writer.write_str("layout(location = 0) out vec4 _outColor;\r\n").unwrap();
      
    }

    fn vertex_names(&self) -> &HashMap<String,(usize,String)> {
       &self.vertexs
    }


    fn write_uniforms<W:Write>(&self, writer:&mut W,shader:&Shader,ex_data:&ShaderTask) {
        let mut ubos:HashMap<&String,Arc<UBOInfo>> = Default::default();
        for backend_name in shader.backend.iter() {
            if let Some(ubo_info) = self.render_info.backend2ubo.get(backend_name) {
                if !ubos.contains_key(&backend_name) {
                    ubos.insert(backend_name,ubo_info.clone());
                }
            }
        }
        let mut ubo_list = ubos.values().collect::<Vec<_>>();
        ubo_list.sort_by(|a,b| a.index.cmp(&b.index));
        let mut index:usize = 0;
        for ubo in ubo_list.iter() {
            write_ubo_uniform(&ubo,writer,index);
            index += 1;
        }
        writer.write_str("\r\n").unwrap();
        if ex_data.prop_def.infos.len() > 0 {
            writer.write_str(&format!("layout(set = {}, binding = 0) uniform Material {{\r\n",index)).unwrap();
            for prop in ex_data.prop_def.infos.iter() {
                if let UniformInfo::Raw(raw) = prop {
                    write_ubo_uniform_prop(raw, writer);
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

    fn write_backend_trait<W:Write>(&self, write:&mut W, shader:&Shader, backends:&glsl_pkg::backends::Backends) {
        for backend_name in shader.backend.iter() {
            if let Some(backend) = backends.values.get(backend_name) {
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
    fn write_backend_trait_fn<W:Write>(&self,writer:&mut W,fn_info:&BackendItem,backend_name:&String) {
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
                writer.write_str(&format!("{} get{}{}(int index) {{ \r\n",fn_info.typ,new_array_name,new_name)).unwrap();
                writer.write_str(&format!("  return _{}.{}[index].{};\r\n",&ubo_info.name,array_name,fn_info.name)).unwrap();
                writer.write_str("}\r\n").unwrap();
            }
        } else {
            writer.write_str(&format!("{} get{}() {{ \r\n",fn_info.typ,new_name)).unwrap();
            if let Some(ubo_info) = self.render_info.backend2ubo.get(backend_name) {
                writer.write_str(&format!("  return _{}.{};\r\n",&ubo_info.name,fn_info.name)).unwrap();
            }
            writer.write_str("}\r\n").unwrap();
        }
    }
}


fn write_ubo_uniform<W:Write>(info:&UBOInfo, writer:&mut W,index:usize) {

    for prop in info.props.infos.iter() {
        if let UniformInfo::Array(arr) = prop {
            writer.write_str("\r\n").unwrap();
            writer.write_str(&format!("struct {}{} {{\r\n",&info.name,arr.name)).unwrap();
            for prop in arr.elem_def.infos.iter() {
                if let UniformInfo::Raw(raw) = prop {
                    write_ubo_uniform_prop(raw, writer);
                }
            }
            writer.write_str("};\r\n").unwrap();
        }
    }
    writer.write_str("\r\n").unwrap();
    writer.write_str(&format!("layout(set = {}, binding = 0) uniform {} {{\r\n",index,&info.name)).unwrap();
    for prop in info.props.infos.iter() {
        match prop {
            UniformInfo::Raw(raw) => {
                write_ubo_uniform_prop(raw, writer);
            },
            UniformInfo::Array(arr) => {
                writer.write_str(&format!("  {}{} {}[{}];\r\n",&info.name,&arr.name,&arr.name,arr.array_size)).unwrap();
            }
        }
       
    }
    writer.write_str(&format!("}} _{};\r\n",&info.name)).unwrap();
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