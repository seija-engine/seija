use std::{convert::{TryFrom, TryInto}, sync::Arc};

use serde_json::{Value};
use crate::memory::{PropInfoList, UniformBufferDef};
#[derive(Debug,Clone, Copy)]
pub enum UBOType {
    ComponentBuffer,
    GlobalBuffer
}

impl TryFrom<&Value> for UBOType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
       let str = value.as_str().ok_or(format!("{:?}",value))?;
       match str {
           ":ComponentBuffer" => Ok(UBOType::ComponentBuffer),
           ":GlobalBuffer" => Ok(UBOType::GlobalBuffer),
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
pub struct UBOInfo {
    pub typ:UBOType,
    pub apply:UBOApplyType,
    pub name:Arc<String>,
    pub index:usize,
    pub props:Arc<UniformBufferDef>,
    pub backends:Vec<String>,
    pub shader_stage:wgpu::ShaderStage
}


impl TryFrom<&Value> for UBOInfo {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let object = value.as_object().ok_or("root".to_string())?;
        let typ:UBOType = object.get(":type").ok_or(":type".to_string())?.try_into()?;
        let apply:UBOApplyType = object.get(":apply").ok_or(":apply".to_string())?.try_into()?;
        let name = object.get(":name").and_then(Value::as_str).ok_or(":name".to_string())?;
        let backends = object.get(":backends")
                                        .and_then(|v| v.as_array())
                                        .map(|lst| lst.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<String>>())
                                        .ok_or(":backends")?;
        let prop_json = object.get(":props").ok_or(":props".to_string())?;
        let props:PropInfoList = prop_json.try_into().map_err(|_| ":props".to_string())?;
        let prop_index = object.get(":index").and_then(|v| v.as_i64()).unwrap_or(0);
        let udf = UniformBufferDef::try_from(&props).map_err(|_| ":props".to_string() )?;
        let shader_stage = object.get(":shader-stage").and_then(Value::as_i64).unwrap_or(
            wgpu::ShaderStage::VERTEX_FRAGMENT.bits() as i64
        ) as u32;
        Ok(UBOInfo {
            typ,
            apply,
            name:Arc::new(name.to_string()),
            props:Arc::new(udf),
            backends,
            index:prop_index as usize,
            shader_stage:wgpu::ShaderStage::from_bits(shader_stage)
                               .unwrap_or(wgpu::ShaderStage::VERTEX_FRAGMENT) 
        })
    }
}