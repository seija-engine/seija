use std::convert::TryFrom;
use serde_json::Value;
use wgpu::TextureSampleType;

#[derive(Debug)]
pub struct UniformTextureDef {
    pub name:String,
    pub sample_type:TextureSampleType
}

impl TryFrom<&Value> for UniformTextureDef {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let json_map = value.as_object().ok_or(())?;
        let name = json_map.get(":name").and_then(Value::as_str).ok_or(())?.to_string();
        let type_str = json_map.get(":type").and_then(Value::as_str).ok_or(())?;
        let sample_type = match type_str {
            "texture2D" => {
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
            name,
            sample_type
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