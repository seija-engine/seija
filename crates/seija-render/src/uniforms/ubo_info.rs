use std::{convert::{TryFrom, TryInto}, sync::Arc};

use serde_json::{Value};
use crate::memory::{PropInfo, PropInfoList, UniformBufferDef};
#[derive(Debug)]
pub enum UBOType {
    PerCamera,
    PerObject,
    PerFrame
}

impl TryFrom<&Value> for UBOType {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
       let str = value.as_str().ok_or(format!("{:?}",value))?;
       match str {
           ":PerCamera" => Ok(UBOType::PerCamera),
           ":PerObject" => Ok(UBOType::PerObject),
           ":PerFrame" => Ok(UBOType::PerFrame),
           _ => Err(str.to_string())
       }
    }
}

#[derive(Debug)]
pub struct UBOInfo {
    pub typ:UBOType,
    pub name:String,
    pub props:Arc<UniformBufferDef>,
    pub backends:Vec<String>
}


impl TryFrom<&Value> for UBOInfo {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let object = value.as_object().ok_or("root".to_string())?;
        let typ:UBOType = object.get(":type").ok_or(":type".to_string())?.try_into()?;
        let name = object.get(":name").and_then(Value::as_str).ok_or(":name".to_string())?;
        let backends = object.get(":backends")
                                        .and_then(|v| v.as_array())
                                        .map(|lst| lst.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<String>>())
                                        .ok_or(":backends")?;
        let prop_json = object.get(":props").ok_or(":props".to_string())?;
        let props:PropInfoList = prop_json.try_into().map_err(|_| ":props".to_string())?;
        let udf = UniformBufferDef::try_from(&props).map_err(|_| ":props".to_string() )?;
        Ok(UBOInfo {
            typ,
            name:name.to_string(),
            props:Arc::new(udf),
            backends
        })
    }
}