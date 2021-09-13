use std::convert::{TryFrom};
use seija_core::{TypeUuid};
use super::{RenderOrder, errors::MaterialDefReadError, types::{Cull, ZTest}};
use lite_clojure_eval::EvalRT;
use serde_json::{Value};
use uuid::Uuid;

#[derive(Debug,TypeUuid)]
#[uuid = "58ee0320-a01e-4a1b-9d07-ade19767853b"]
pub struct MaterialDef {
    pub name:String,
    pub order:RenderOrder,
    pub pass_list:Vec<PassDef>,
    pub prop_defs:Vec<PropDef>
}
#[derive(Debug)]
pub struct  PropDef {
    name:String,
    value:PropValueDef
}

#[derive(Debug)]
pub enum PropValueDef {
    Int(i32),
    Float(f32)
}

#[derive(Debug)]
pub struct PassDef {
    z_write:bool,
    z_test:ZTest,
    cull:Cull,
    vs_path:String,
    fs_path:String
}

impl Default for PassDef {
    fn default() -> Self {
        Self { 
            z_write:true,
            z_test:ZTest::Less,
            cull: Cull::Back,
            vs_path:String::default(),
            fs_path:String::default() 
        }
    }
}

pub fn read_material_def(vm:&mut EvalRT,file_string:&str) -> Result<MaterialDef,MaterialDefReadError>  {
    let value:Value = vm.eval_string(String::default(), file_string).ok_or(MaterialDefReadError::LanguageError)?.into();
    let value_object = value.as_object().ok_or(MaterialDefReadError::FormatError)?;
   
    //name
    let def_name = value_object.get(":name").and_then(|v| v.as_str()).ok_or(MaterialDefReadError::InvalidName)?;
    //order
    let order_str = value.get(":order").and_then(|v| v.as_str()).unwrap_or(&"Opaque");
    let order = RenderOrder::try_from(order_str).map_err(|s| MaterialDefReadError::InvalidOrder(s))?;
    //pass
    let json_pass = value.get(":pass").ok_or(MaterialDefReadError::InvalidPass)?;
    let mut pass_list:Vec<PassDef> = Vec::new();
    match json_pass {
        Value::Array(arr) => {
            for v in arr {
                pass_list.push(read_pass(v)?);
            }
        },
        Value::Object(_) => { pass_list.push(read_pass(json_pass)?); },
        _ => return Err(MaterialDefReadError::InvalidPass)
    }

    let mut prop_defs:Vec<PropDef> = vec![];
    if let Some(prop_arr) = value_object.get(":props").and_then(|v|v.as_array()) {
        for json_prop in prop_arr {
            prop_defs.push(read_prop(json_prop)?);
        }
    }

    Ok(MaterialDef {
        name:def_name.to_string(),
        order,
        pass_list,
        prop_defs
    })
}

fn read_pass(json_value:&Value) -> Result<PassDef,MaterialDefReadError> {
    let map = json_value.as_object().ok_or(MaterialDefReadError::InvalidPass)?;
    let mut pass_def = PassDef::default();
    if let Some(z_write) = map.get(":z-write").and_then(|v| v.as_bool()) {
        pass_def.z_write = z_write;
    }
    if let Some(z_test) = map.get(":z-test").and_then(|v| v.as_str()) {
        pass_def.z_test = ZTest::try_from(z_test).map_err(|_| MaterialDefReadError::InvalidPass)?;
    }
    if let Some(cull) = map.get(":cull").and_then(|v| v.as_str()) {
        pass_def.cull = Cull::try_from(cull).map_err(|_| MaterialDefReadError::InvalidPass)?;
    }
    pass_def.vs_path = map.get(":vs").and_then(|v|v.as_str()).ok_or( MaterialDefReadError::InvalidPass)?.to_string();
    pass_def.fs_path = map.get(":fs").and_then(|v|v.as_str()).ok_or( MaterialDefReadError::InvalidPass)?.to_string();
    Ok(pass_def)
}

fn read_prop(json_value:&Value) -> Result<PropDef,MaterialDefReadError> {
    let map = json_value.as_object().ok_or(MaterialDefReadError::InvalidProp)?;
    let name = map.get(":name").and_then(|v|v.as_str()).ok_or( MaterialDefReadError::InvalidProp)?.to_string();
    let typ = map.get(":type").and_then(|v|v.as_str()).ok_or( MaterialDefReadError::InvalidProp)?;
    let value = match typ {
        "Int" => {
            let default = map.get(":default").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            PropValueDef::Int(default)
         },
         "Float" => {
            let default = map.get(":default").and_then(|v| v.as_f64()).unwrap_or(0f64) as f32;
            PropValueDef::Float(default)
         },
         _ => return Err(MaterialDefReadError::InvalidProp)
    };
    Ok(PropDef{name,value})
}