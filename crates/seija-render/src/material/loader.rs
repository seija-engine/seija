use std::{sync::Arc};

use anyhow::Context;
use bevy_ecs::prelude::World;
use lite_clojure_eval::EvalRT;
use relative_path::RelativePath;
use seija_asset::{AssetServer,async_trait::async_trait ,AssetLoaderParams, AssetDynamic, Assets, downcast_rs::*,IAssetLoader, AsyncLoadMode, HandleUntyped, add_to_asset_type, this_asset_path};
use seija_core::{anyhow::{Result,anyhow},smol, bytes::AsBytes, OptionExt};
use seija_core::TypeUuid;
use serde_json::Value;
use smol_str::SmolStr;


use crate::{MemUniformInfo,material::Material, UniformType, RawUniformInfo, resource::Texture};

use super::{read_material_def, material_def::MaterialDefineAsset, MaterialDef};


#[derive(Default)]
pub(crate) struct MaterialDefineLoader;

#[async_trait]
impl IAssetLoader for MaterialDefineLoader {
    fn typ(&self) -> uuid::Uuid { MaterialDefineAsset::TYPE_UUID }

    fn add_to_asset(&self, world:&mut World, res:Box<dyn AssetDynamic>) -> Result<HandleUntyped> {
        add_to_asset_type::<MaterialDefineAsset>(world, res)
    }

    fn sync_load(&self,_:&mut World,path:&str,server:&AssetServer,_:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path)?;
        log::info!("loader material define full_path: {:?}",&full_path);
        let code_string = std::fs::read_to_string(full_path)?;
        load_material_def(&code_string)
    }

    async fn async_load(&self,server:AssetServer,path:SmolStr,
                        _:Option<Box<dyn DowncastSync>>,
                        _:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(&path)?;
        let code_string = smol::fs::read_to_string(full_path).await?;
        load_material_def(&code_string)
    }
}

fn load_material_def(code_string:&str) -> Result<Box<dyn AssetDynamic>> {
    let mut vm = EvalRT::new();
    
    let define = read_material_def(&mut vm, &code_string, false)?;
    let asset = MaterialDefineAsset { define:Arc::new(define) };
   
    Ok(Box::new(asset))
}


struct MaterialTouch {
    json:Value,
    define_handle:HandleUntyped,
    rc_define:Option<Arc<MaterialDef>>
}

#[derive(Default)]
pub(crate) struct MaterialLoader;

#[async_trait]
impl IAssetLoader for MaterialLoader {
    fn typ(&self) -> uuid::Uuid { Material::TYPE_UUID }
    fn mode(&self) -> AsyncLoadMode { AsyncLoadMode::Touch }
    
    fn add_to_asset(&self, world:&mut World, res:Box<dyn AssetDynamic>) -> Result<HandleUntyped> {
        add_to_asset_type::<Material>(world, res)
    }

    fn sync_load(&self,w:&mut World,path:&str,server:&AssetServer,_:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let file_path = RelativePath::new(path);
        let cur_dir = file_path.parent().get()?;
        
        let full_path = server.full_path(path)?;
        log::info!("loader material full_path: {:?}",&full_path);
        let bytes = std::fs::read(full_path)?;
        let json_value:Value = serde_json::from_slice(&bytes)?;
        let json_map = json_value.as_object().get()?;
        let material_def_path = json_map.get("material").and_then(Value::as_str).get()?;
        let asset_material_def_path = this_asset_path(cur_dir, material_def_path);
        
        let h_def = server.load_sync::<MaterialDefineAsset>(w, asset_material_def_path.as_str(), None)?;
        let defs = w.get_resource::<Assets<MaterialDefineAsset>>().get()?;
        let def_asset = defs.get(&h_def.id).get()?;
        let mut material = Material::from_def(def_asset.define.clone(), &server).get()?;
        let json_props = json_map.get("props").get()?;
        set_material_props(&mut material,json_props)?;
        set_material_textures_sync(w,&mut material,json_props,&server,cur_dir)?;
        Ok(Box::new(material))
    }

    async fn async_touch(&self, server:AssetServer, path:SmolStr) -> Result<Box<dyn DowncastSync>> {
        let full_path = server.full_path(&path)?;
        let bytes = std::fs::read(full_path)?;
        let json:Value = serde_json::from_slice(&bytes)?;
        let json_map = json.as_object().context(0)?;
        let material_def_path = json_map.get("material").and_then(Value::as_str).context(1)?;
        let req = server.load_async::<MaterialDefineAsset>(material_def_path,None)?;
        let define_handle = req.wait_handle().await.ok_or(anyhow!("load material define error"))?;
        let touch = MaterialTouch { json,define_handle,rc_define:None };
        let ret:Box<dyn DowncastSync> = Box::new(touch);
        Ok(ret)  
    }

    fn perpare(&self, world:&mut World, touch_data:Option<Box<dyn DowncastSync>>) -> Option<Box<dyn DowncastSync>> {
        if let Some(Ok(mut touch)) = touch_data.map(|v|v.into_any().downcast::<MaterialTouch>()) {
            let mut_touch:&mut MaterialTouch = touch.as_mut();
            let defines = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
            if let Some(define) = defines.get(&mut_touch.define_handle.id) {
                mut_touch.rc_define = Some(define.define.clone());
            }
            return Some(touch);
        }
        None
    }

    async fn async_load(&self,server:AssetServer,_:SmolStr,
        touch:Option<Box<dyn DowncastSync>>,
                        _:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
       let touch = touch.ok_or(anyhow!("touch error"))?.into_any()
                                                   .downcast::<MaterialTouch>()
                                                   .map_err(|_|anyhow!("cast error"))?;

        let define = touch.rc_define.ok_or(anyhow!("rc_define err"))?;
        let mut material = Material::from_def(define, &server).context(5)?;
        material.define = Some(touch.define_handle.typed());
        let json_map = touch.json.as_object().context(0)?;
        let json_props = json_map.get("props").context(6)?;
        set_material_props(&mut material,json_props)?;
        set_material_textures(&mut material,json_props,&server).await?;
        let ret:Box<dyn AssetDynamic> = Box::new(material);
        Ok(ret)
    }
}
async fn set_material_textures(material:&mut Material,value:&Value,server:&AssetServer) -> Result<()> {
    let props = value.as_object().context(1)?;
    let define = material.def.clone();
    for (k,v) in props.iter() {
        if let Some(_) = define.tex_prop_def.get_info(k) {
            let texture_path = v.as_str().context(2)?;
            let req = server.load_async::<Texture>(texture_path, None)?;
            let h_tex = req.wait_handle().await.context(3)?;
            material.texture_props.set(k, h_tex.typed());
        }
    }
    Ok(())
}

fn set_material_textures_sync(world:&mut World,material:&mut Material,value:&Value,server:&AssetServer,cur_dir:&RelativePath) -> Result<()> {
    let props = value.as_object().get()?;
    let define = material.def.clone();
    for (k,v) in props.iter() {
        if let Some(_) = define.tex_prop_def.get_info(k) {
            let texture_path = v.as_str().get()?;
            let asset_texture_path = this_asset_path(cur_dir, texture_path);
            let handle = server.load_sync::<Texture>(world,asset_texture_path.as_str(), None)?;
            material.texture_props.set(k, handle);
        }
    }
    Ok(())
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
