use std::convert::TryFrom;

use serde_json::Value;

use crate::material::STextureDescriptor;

#[derive(Debug)]
pub struct UniformTextureDef {
    pub name:String,
    pub desc:STextureDescriptor
}

impl TryFrom<&Value> for UniformTextureDef {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let json_map = value.as_object().ok_or(())?;
        let name = json_map.get(":name").and_then(Value::as_str).ok_or(())?.to_string();
        let desc = json_map.get(":desc").ok_or(()).and_then(STextureDescriptor::try_from)?;
        Ok(UniformTextureDef {
            name,
            desc
        })
    }
}