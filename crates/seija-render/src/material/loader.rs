use std::sync::Arc;

use anyhow::Context;
use bevy_ecs::prelude::World;
use lite_clojure_eval::EvalRT;
use seija_asset::{AssetLoader, AssetServer, LoadingTrack, AssetLoaderParams, AssetDynamic, Assets};
use seija_core::{anyhow::{Result},smol};
use async_trait::{async_trait};
use serde_json::Value;

use crate::material::Material;

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
        let h_def = server.load_sync::<MaterialDefineAsset>(world, material_def_path, None).context(2)?;
        let defs = world.get_resource::<Assets<MaterialDefineAsset>>().context(3)?;
        let def_asset = defs.get(&h_def.id).context(4)?;
        let material = Material::from_def_new(def_asset.define.clone(), &server).context(5)?;
        Ok(Box::new(material))
    }
}