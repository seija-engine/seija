use std::{convert::{TryFrom}, sync::{Arc}, cell::Ref};
use seija_core::{TypeUuid};
use wgpu::{FrontFace, PolygonMode};
use super::{RenderOrder, errors::MaterialDefReadError, storage::DEFAULT_TEXTURES, texture_prop_def::TexturePropDef, types::{Cull, SFrontFace, SPolygonMode, ZTest}};
use lite_clojure_eval::EvalRT;
use serde_json::{Value};
use uuid::Uuid;
use crate::{memory::UniformBufferDef};

#[derive(Debug,TypeUuid)]
#[uuid = "58ee0320-a01e-4a1b-9d07-ade19767853b"]
pub struct MaterialDef {
    pub name:String,
    pub order:RenderOrder,
    pub is_light:bool,
    pub pass_list:Vec<PassDef>,
    pub prop_def:Arc<UniformBufferDef>,
    pub tex_prop_def:Arc<TexturePropDef>,
}

#[derive(Debug)]
pub struct PassDef {
    pub conservative:bool,
    pub polygon_mode:SPolygonMode,
    pub front_face:SFrontFace,
    pub z_write:bool,
    pub z_test:ZTest,
    pub cull:Cull,
    pub clamp_depth:bool,
    pub shader_info:ShaderInfoDef,
   
}

impl Default for PassDef {
    fn default() -> Self {
        Self {
            conservative:false,
            polygon_mode:SPolygonMode(PolygonMode::Fill),
            front_face:SFrontFace(FrontFace::Ccw),
            z_write:true,
            z_test:ZTest::Less,
            clamp_depth:false,
            cull: Cull::Back,
            shader_info:ShaderInfoDef::default()
        }
    }
}
#[derive(Debug,Default)]
pub struct ShaderInfoDef {
    pub name:String,
    pub macros:Arc<Vec<String>>
}

pub fn read_material_def(vm:&mut EvalRT,file_string:&str) -> Result<MaterialDef,MaterialDefReadError>  {
    let mut is_light = false;
    let value:Value = vm.eval_string(String::default(), file_string).ok_or(MaterialDefReadError::LanguageError)?.into();
    let value_object = value.as_object().ok_or(MaterialDefReadError::FormatError)?;
    
    //name
    let def_name = value_object.get(":name").and_then(|v| v.as_str()).ok_or(MaterialDefReadError::InvalidName)?;
    //order
    let order_str = value.get(":order").and_then(|v| v.as_str()).unwrap_or(&"Opaque");
    let order = RenderOrder::try_from(order_str).map_err(|s| MaterialDefReadError::InvalidOrder(s))?;
    //light
    if let Some(light) = value.get(":light").and_then(|v| v.as_bool()) {
        is_light = light;
    }
    
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

    let prop_value = value.get(":props").ok_or(MaterialDefReadError::InvalidProp)?;
    let buffer_def = UniformBufferDef::try_from(prop_value).map_err(|_| MaterialDefReadError::InvalidProp)?;
    let texture_prop_def = read_texture_prop(prop_value).map_err(|_| MaterialDefReadError::InvalidProp)?;
    Ok(MaterialDef {
        name:def_name.to_string(),
        order,
        pass_list,
        prop_def:Arc::new(buffer_def),
        tex_prop_def:Arc::new(texture_prop_def),
        is_light
    })
}

fn read_texture_prop(json_value:&Value) -> Result<TexturePropDef,()> {
    let arr = json_value.as_array().ok_or( ())?;
    let mut texture_props = TexturePropDef::default();
   
    let mut texture_index:usize = 0;
    for item in arr {
        if let Some(map) = item.as_object() {
            let prop_type = map.get(":type").and_then(|v| v.as_str()).ok_or(())?;
            let prop_name = map.get(":name").and_then(|v| v.as_str()).ok_or(())?;
            let def_name = map.get(":default").and_then(|v| v.as_str());
            let mut def_index = 0;
            if let Some(def_name) = def_name {
                let idx = DEFAULT_TEXTURES.get(def_name).map(|s| *s).unwrap_or(0);
                def_index = idx;
            }
            match prop_type {
                "Texture" => {
                    texture_props.layout_builder.add_texture(false);
                    texture_props.layout_builder.add_sampler();
                    texture_props.indexs.insert(prop_name.to_string(), (texture_index,def_index));
                   
                    texture_index += 1;
                },
                "CubeMap" => {
                    texture_props.layout_builder.add_texture(true);
                    texture_props.layout_builder.add_sampler();
                    texture_props.indexs.insert(prop_name.to_string(), (texture_index,def_index));
                    texture_index += 1;
                },
                _ => {}
            }
        }
    }
    Ok(texture_props)
}

fn read_pass(json_value:&Value) -> Result<PassDef,MaterialDefReadError> {
    let map = json_value.as_object().ok_or(MaterialDefReadError::InvalidPass)?;
    let mut pass_def = PassDef::default();
    if let Some(z_write) = map.get(":z-write").and_then(|v| v.as_bool()) {
        pass_def.z_write = z_write;
    }
    if let Some(s) = map.get(":front-face").and_then(|v| v.as_str()) {
        pass_def.front_face = SFrontFace::try_from(s).map_err(|_| MaterialDefReadError::InvalidPassProp(":front-face".into()) )?
    }
    //if let Some(b) = map.get(":clamp-depth").and_then(|v| v.as_bool()) {
    //    pass_def.clamp_depth = b;
    //}
    
    if let Some(z_test) = map.get(":z-test").and_then(|v| v.as_str()) {
        pass_def.z_test = ZTest::try_from(z_test).map_err(|_| MaterialDefReadError::InvalidPassProp(":z-test".into()))?;
    }
    if let Some(cull) = map.get(":cull").and_then(|v| v.as_str()) {
        pass_def.cull = Cull::try_from(cull).map_err(|_| MaterialDefReadError::InvalidPassProp(":cull".into()))?;
    }
    if let Some(poly) = map.get(":polygon-mode").and_then(|v| v.as_str()) {
        pass_def.polygon_mode = SPolygonMode::try_from(poly).map_err(|_| MaterialDefReadError::InvalidPassProp(":polygon-mode".into()))?;
    }
    if let Some(b) = map.get(":conservative").and_then(|v| v.as_bool()) {
        pass_def.conservative = b;
    }
    let shader_value =  map.get(":shader").ok_or(MaterialDefReadError::InvalidPassProp("shader".into()))?;
    pass_def.shader_info = ShaderInfoDef::try_from(shader_value)?;
    Ok(pass_def)
}

impl TryFrom<&Value> for ShaderInfoDef  {
    type Error = MaterialDefReadError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let name = value.get(":name").and_then(|v| v.as_str()).ok_or(MaterialDefReadError::InvalidPassProp("shader".into()))?;
        let mut macro_arr = vec![];
        if let Some(macros) = value.get(":macros") {
            macro_arr = macros.as_array().map(|arr| {
               arr.iter().filter_map(|v| v.as_str()).map(|v| v.to_string()).collect()
           }).unwrap_or(vec![]);
        }
        Ok(ShaderInfoDef {
            name:name.to_string(),
            macros:Arc::new(macro_arr)
        })
    }
}