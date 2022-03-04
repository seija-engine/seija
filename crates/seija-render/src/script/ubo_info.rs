use std::convert::{TryFrom, TryInto};

use serde_json::{Value, json};
use crate::memory::{PropInfo};

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

pub struct UBOInfo {
    typ:UBOType,
    name:String,
    props:Vec<PropInfo>,
    backends:Vec<String>
}


impl TryFrom<&Value> for UBOInfo {
    type Error = String;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let object = value.as_object().ok_or("root".to_string())?;
        let typ:UBOType = object.get(":type").ok_or(":type".to_string())?.try_into()?;
        let name = object.get(":name").ok_or(":name".to_string())?;
        let backends = object.get(":backends")
                                      .and_then(|v| v.as_array())
                                      .map(|lst| lst.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<String>>())
                                      .ok_or(":backends")?;
                    
        todo!()
    }
}