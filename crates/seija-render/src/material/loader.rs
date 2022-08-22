use std::sync::Arc;

use anyhow::Context;
use bevy_ecs::prelude::World;
use lite_clojure_eval::EvalRT;
use seija_asset::{AssetLoader, AssetServer, LoadingTrack, AssetLoaderParams, AssetDynamic, Assets};
use seija_core::{anyhow::{Result,anyhow},smol, bytes::AsBytes};
use async_trait::{async_trait};
use serde_json::Value;

use crate::{MemUniformInfo,material::Material, UniformType, RawUniformInfo};

use super::{read_material_def, material_def::MaterialDefineAsset};
pub(crate) struct MaterialDefineAssetLoader;

#[async_trait]
impl AssetLoader for MaterialDefineAssetLoader {
    async fn load(&self,server:AssetServer,_:Option<LoadingTrack>,path:&str,_:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path)?;
        let code_string = smol::fs::read_to_string(&full_path).await?;
        let mut vm = EvalRT::new();
        let define = read_material_def(&mut vm, &code_string, false)?;
        let asset = MaterialDefineAsset { define:Arc::new(define) };
        Ok(Box::new(asset))
    }
}


pub(crate) struct MaterialLoader;

#[async_trait]
impl AssetLoader for MaterialLoader {
    async fn load(&self,_:AssetServer,_:Option<LoadingTrack>,_:&str,_:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        unimplemented!()
    }

    fn load_sync(&self,world:&mut World, path:&str, server:AssetServer, _:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path)?;
        let bytes = std::fs::read(full_path)?;
        let json_value:Value = serde_json::from_slice(&bytes)?;
        let json_map = json_value.as_object().context(0)?;
        let material_def_path = json_map.get("material").and_then(Value::as_str).context(1)?;
        let h_def = server.load_sync::<MaterialDefineAsset>(world, material_def_path, None,false).context(2)?;
        let defs = world.get_resource::<Assets<MaterialDefineAsset>>().context(3)?;
        let def_asset = defs.get(&h_def.id).context(4)?;
        let mut material = Material::from_def(def_asset.define.clone(), &server).context(5)?;
        let json_props = json_map.get("props").context(6)?;
        set_material_props(&mut material,json_props)?;
       
        Ok(Box::new(material))
    }
}


fn set_material_props(material:&mut Material,value:&Value) -> Result<()> {
    let props = value.as_object().context(1)?;
    let define = material.def.clone();
    let bool_fn = |v:&Value| { v.as_bool().map(|b| { if b {1} else {0} } ) };
    let float3_fn = |v:&Value| -> Option<[f32;3]> {
        let arr = v.as_array()?;
        let x = arr.get(0).and_then(Value::as_f64).map(|f| f as f32)?;
        let y = arr.get(1).and_then(Value::as_f64).map(|f| f as f32)?;
        let z = arr.get(2).and_then(Value::as_f64).map(|f| f as f32)?;
        Some([x,y,z])
    };
    let float4_fn = |v:&Value| -> Option<[f32;4]> {
        let arr = v.as_array()?;
        let x = arr.get(0).and_then(Value::as_f64).map(|f| f as f32)?;
        let y = arr.get(1).and_then(Value::as_f64).map(|f| f as f32)?;
        let z = arr.get(2).and_then(Value::as_f64).map(|f| f as f32)?;
        let w = arr.get(3).and_then(Value::as_f64).map(|f| f as f32)?;
        Some([x,y,z,w])
    };

    for (k,v) in props.iter() {
        if let Some(MemUniformInfo::Raw(raw_info)) = define.prop_def.get_info(k) {
            let succ = match raw_info.typ {
                UniformType::BOOL(_) => { set_material_props_item(material, raw_info, v, bool_fn) },
                UniformType::FLOAT(_) => { set_material_props_item(material, raw_info, v, |vv| Value::as_f64(vv).map(|f| f as f32))},
                UniformType::INT(_) => { set_material_props_item(material, raw_info, v, |vv| Value::as_i64(vv).map(|f| f as i32)) },
                UniformType::UINT(_) => { set_material_props_item(material, raw_info, v, |vv| Value::as_u64(vv).map(|f| f as u32)) },
                UniformType::FLOAT3(_) => { set_material_props_item(material, raw_info, v, float3_fn) },
                UniformType::FLOAT4(_) => { set_material_props_item(material, raw_info, v, float4_fn) },
                _ => { unimplemented!() },
            };
            succ.ok_or(anyhow!(format!("material prop error:{}",k)))?
        }
    }
    Ok(())
}

fn set_material_props_item<T:AsBytes>(material:&mut Material,raw_info:&RawUniformInfo,value:&Value,v_to_t:fn(value:&Value) -> Option<T>) -> Option<()> {
    if raw_info.size > 1 {
        for (index,item) in value.as_array()?.iter().enumerate() {
            let v = v_to_t(item)?;
            let offset = raw_info.get_buffer_offset(index);
            material.props.buffer.write_bytes(offset, v);
        }
    } else {
        let v = v_to_t(value)?;
        let offset = raw_info.get_buffer_offset(0);
        material.props.buffer.write_bytes(offset, v);
    }
    Some(())
}
