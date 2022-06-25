use std::{convert::{TryFrom, TryInto}, sync::Arc};

use serde_json::{Value};
use crate::{memory::{PropInfoList, UniformBufferDef}, pipeline::render_bindings::{BindGroupBuilder, BindGroupLayoutBuilder}};

use super::texture_def::UniformTextureDef;
#[derive(Debug,Clone, Copy)]
pub enum UniformType {
    Component,
    Global
}

impl TryFrom<&Value> for UniformType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
       let str = value.as_str().ok_or(format!("{:?}",value))?;
       match str {
           ":Component" => Ok(UniformType::Component),
           ":Global" => Ok(UniformType::Global),
           _ => Err(str.to_string())
       }
    }
}

#[derive(Debug,Clone, Copy)]
pub enum UBOApplyType {
    Camera,
    RenderObject,
    Frame
}

impl TryFrom<&Value> for UBOApplyType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
       let str = value.as_str().ok_or(format!("{:?}",value))?;
       match str {
           ":Camera" => Ok(UBOApplyType::Camera),
           ":RenderObject" => Ok(UBOApplyType::RenderObject),
           ":Frame" => Ok(UBOApplyType::Frame),
           _ => Err(str.to_string())
       }
    }
}

#[derive(Debug)]
pub struct UniformInfo {
    pub typ:UniformType,
    pub apply:UBOApplyType,
    pub name:Arc<String>,
    pub sort:usize,
    pub props:Arc<UniformBufferDef>,
    pub textures:Arc<Vec<UniformTextureDef>>,
    pub backends:Vec<String>,
    pub shader_stage:wgpu::ShaderStage
}


impl TryFrom<&Value> for UniformInfo {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let object = value.as_object().ok_or("root".to_string())?;
        let typ:UniformType = object.get(":type").ok_or(":type".to_string())?.try_into()?;
        let apply:UBOApplyType = object.get(":apply").ok_or(":apply".to_string())?.try_into()?;
        let name = object.get(":name").and_then(Value::as_str).unwrap_or_default();
        let backends = object.get(":backends")
                                        .and_then(|v| v.as_array())
                                        .map(|lst| lst.iter()
                                        .filter_map(|v| v.as_str().map(String::from)).collect::<Vec<String>>())
                                        .ok_or(":backends")?;
        let prop_json = object.get(":props").ok_or(":props".to_string())?;
        let props:PropInfoList = prop_json.try_into().map_err(|_| ":props".to_string())?;
        let prop_index = object.get(":index").and_then(|v| v.as_i64()).unwrap_or(0);
        let udf = UniformBufferDef::try_from(&props).map_err(|_| ":props".to_string() )?;
        let shader_stage = object.get(":shader-stage").and_then(Value::as_i64).unwrap_or(
            wgpu::ShaderStage::VERTEX_FRAGMENT.bits() as i64
        ) as u32;

        let mut textures:Vec<UniformTextureDef> = vec![];
        if let Some(json_textures) = object.get(":textures").and_then(Value::as_array) {
            for json_item in json_textures.iter() {
                if let Ok(texture_def) = UniformTextureDef::try_from(json_item) {
                    textures.push(texture_def);
                } else {
                    log::warn!("read UniformTextureDef error:{}",json_item.to_string());
                }
            }
        }

        Ok(UniformInfo {
            typ,
            apply,
            name:Arc::new(name.to_string()),
            props:Arc::new(udf),
            backends,
            sort:prop_index as usize,
            textures:Arc::new(textures),
            shader_stage:wgpu::ShaderStage::from_bits(shader_stage)
                               .unwrap_or(wgpu::ShaderStage::VERTEX_FRAGMENT) 
        })
    }
}


impl UniformInfo {
    pub fn create_layout(&self,device:&wgpu::Device) -> wgpu::BindGroupLayout {
        let mut builder = BindGroupLayoutBuilder::new();
        if self.props.infos.len() > 0 {
            builder.add_uniform(self.shader_stage);
        }
        for texture_desc in self.textures.iter() {
            builder.add_texture(false, Some(texture_desc.sample_type));
            builder.add_sampler(texture_desc.is_filterable());
        }
        let layout = builder.build(device);
        layout
    }
}