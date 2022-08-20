use std::sync::Arc;

use lite_clojure_eval::EvalRT;
use seija_asset::{AssetLoader, AssetServer, LoadingTrack, AssetLoaderParams, AssetDynamic};
use seija_core::{anyhow::{Result},smol};
use async_trait::{async_trait};

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
