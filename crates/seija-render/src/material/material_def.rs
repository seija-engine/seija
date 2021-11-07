use std::{collections::HashMap, convert::{TryFrom}, sync::{Arc}};
use seija_core::{TypeUuid};
use wgpu::{FrontFace, PolygonMode};
use super::{RenderOrder, errors::MaterialDefReadError, types::{Cull, SFrontFace, SPolygonMode, ZTest}};
use lite_clojure_eval::EvalRT;
use serde_json::{Value};
use uuid::Uuid;
use crate::{memory::UniformBufferDef, pipeline::render_bindings::BindGroupLayoutBuilder};

#[derive(Debug,TypeUuid)]
#[uuid = "58ee0320-a01e-4a1b-9d07-ade19767853b"]
pub struct MaterialDef {
    pub name:String,
    pub order:RenderOrder,
    pub pass_list:Vec<PassDef>,
    pub prop_def:Arc<UniformBufferDef>,
    pub texture_idxs:HashMap<String,usize>,
    pub texture_layout_builder:BindGroupLayoutBuilder
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
    pub vs_path:String,
    pub fs_path:String
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

    let prop_value = value.get(":props").ok_or(MaterialDefReadError::InvalidProp)?;
    let buffer_def = UniformBufferDef::try_from(prop_value).map_err(|_| MaterialDefReadError::InvalidProp)?;
    let (texture_layout_builder,texture_idxs) = 
            read_texture_prop(prop_value).map_err(|_| MaterialDefReadError::InvalidProp)?;
    Ok(MaterialDef {
        name:def_name.to_string(),
        order,
        pass_list,
        prop_def:Arc::new(buffer_def),
        texture_layout_builder,
        texture_idxs
    })
}

fn read_texture_prop(json_value:&Value) -> Result<(BindGroupLayoutBuilder,HashMap<String,usize>),()> {
    let arr = json_value.as_array().ok_or( ())?;
    let mut texture_layout_builder = BindGroupLayoutBuilder::new();
    let mut texture_idxs:HashMap<String,usize> = HashMap::default();
    let mut texture_index:usize = 0;
    for item in arr {
        if let Some(map) = item.as_object() {
            let prop_type = map.get(":type").and_then(|v| v.as_str()).ok_or(())?;
            let prop_name = map.get(":name").and_then(|v| v.as_str()).ok_or(())?;
            match prop_type {
                "Texture" => {
                    texture_layout_builder.add_texture(false);
                    texture_layout_builder.add_sampler();
                    texture_idxs.insert(prop_name.to_string(), texture_index);
                    texture_index += 1;
                },
                "CubeMap" => {
                    texture_layout_builder.add_texture(true);
                    texture_layout_builder.add_sampler();
                    texture_idxs.insert(prop_name.to_string(), texture_index);
                    texture_index += 1;
                },
                _ => {}
            }
        }
    }
    Ok((texture_layout_builder,texture_idxs))
}

fn read_pass(json_value:&Value) -> Result<PassDef,MaterialDefReadError> {
    let map = json_value.as_object().ok_or(MaterialDefReadError::InvalidPass)?;
    let mut pass_def = PassDef::default();
    if let Some(z_write) = map.get(":z-write").and_then(|v| v.as_bool()) {
        pass_def.z_write = z_write;
    }
    if let Some(s) = map.get(":front-face").and_then(|v| v.as_str()) {
        pass_def.front_face = SFrontFace::try_from(s).map_err(|_| MaterialDefReadError::InvalidPass )?
    }
    //if let Some(b) = map.get(":clamp-depth").and_then(|v| v.as_bool()) {
    //    pass_def.clamp_depth = b;
    //}
    if let Some(z_test) = map.get(":z-test").and_then(|v| v.as_str()) {
        pass_def.z_test = ZTest::try_from(z_test).map_err(|_| MaterialDefReadError::InvalidPass)?;
    }
    if let Some(cull) = map.get(":cull").and_then(|v| v.as_str()) {
        pass_def.cull = Cull::try_from(cull).map_err(|_| MaterialDefReadError::InvalidPass)?;
    }
    if let Some(poly) = map.get(":polygon-mode").and_then(|v| v.as_str()) {
        pass_def.polygon_mode = SPolygonMode::try_from(poly).map_err(|_| MaterialDefReadError::InvalidPass)?;
    }
    if let Some(b) = map.get(":conservative").and_then(|v| v.as_bool()) {
        pass_def.conservative = b;
    }

    pass_def.vs_path = map.get(":vs").and_then(|v|v.as_str()).ok_or( MaterialDefReadError::InvalidPass)?.to_string();
    pass_def.fs_path = map.get(":fs").and_then(|v|v.as_str()).ok_or( MaterialDefReadError::InvalidPass)?.to_string();
    Ok(pass_def)
}

