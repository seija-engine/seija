use std::convert::TryFrom;
use serde_json::Value;
use wgpu::TextureSampleType;

#[derive(Debug)]
pub struct UniformTextureDef {
    pub is_cubemap:bool,
    pub name:String,
    pub str_type:String,
    pub sample_type:TextureSampleType
}

impl TryFrom<&Value> for UniformTextureDef {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut is_cubemap = false;
        let json_map = value.as_object().ok_or(())?;
        let name = json_map.get(":name").and_then(Value::as_str).ok_or(())?.to_string();
        let type_str = json_map.get(":type").and_then(Value::as_str).ok_or(())?;
        let sample_type = match type_str {
            "cubeMap" => {
                let filterable = json_map.get(":filterable").and_then(Value::as_bool).unwrap_or(true);
                is_cubemap = true;
                wgpu::TextureSampleType::Float { filterable }
            },
            "texture2D" | "texture2DArray" => {
                let filterable = json_map.get(":filterable").and_then(Value::as_bool).unwrap_or(true);
                wgpu::TextureSampleType::Float { filterable }
            },
            "texture2DShadow" => wgpu::TextureSampleType::Depth,
            "itexture2D" => wgpu::TextureSampleType::Sint,
            "utexture2D" => wgpu::TextureSampleType::Uint,
            _ => { 
                log::error!("error texture type:{}",type_str);
                return Err(())
            }
        };
        Ok(UniformTextureDef {
            is_cubemap,
            name,
            sample_type,
            str_type:type_str.to_string()
        })
    }
}

impl UniformTextureDef {
    pub fn is_filterable(&self) -> bool {
        if let wgpu::TextureSampleType::Float {filterable } = self.sample_type {
            filterable
        } else { false }
    }
}