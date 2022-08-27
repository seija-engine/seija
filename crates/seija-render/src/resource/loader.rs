use std::pin::Pin;
use bevy_ecs::prelude::World;
use smol_str::SmolStr;
use std::future::Future;
use seija_asset::{AssetLoaderParams, AssetLoader, AssetServer, AssetDynamic, downcast_rs::DowncastSync};
use seija_core::TypeUuid;
use seija_core::anyhow::Result;
use crate::resource::Texture;
use seija_core::smol;

use super::TextureDescInfo;


impl AssetLoaderParams for TextureDescInfo {}

pub(crate) fn new_texture_loader() -> AssetLoader {
    AssetLoader {
        typ:Texture::TYPE_UUID,
        sync_load:texture_sync,
        async_touch:None,
        perpare:None,
        async_load:texture_async
    }
}

fn read_desc(params:Option<Box<dyn AssetLoaderParams>>) -> TextureDescInfo {
    params.and_then(|v| v.downcast::<TextureDescInfo>().ok())
    .map(|v| *v)
    .unwrap_or(Default::default())
}

fn texture_sync(_:&mut World,path:&str,server:&AssetServer,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
    let full_path = server.full_path(path)?;
    let bytes = std::fs::read(full_path)?;
    let texture = Texture::from_image_bytes(&bytes, read_desc(params))?;
    Ok(Box::new(texture))
}

async fn async_texture_async(server:AssetServer,path:SmolStr,_:Option<Box<dyn DowncastSync>>,params:Option<Box<dyn AssetLoaderParams>>) 
    -> Result<Box<dyn AssetDynamic>> {
    let full_path = server.full_path(path.as_str())?;
    let bytes = smol::fs::read(full_path).await?;
    let texture = Texture::from_image_bytes(&bytes, read_desc(params))?;
    Ok(Box::new(texture))
}

fn texture_async(server:AssetServer,path:SmolStr,t:Option<Box<dyn DowncastSync>>,params:Option<Box<dyn AssetLoaderParams>>) 
  -> Pin<Box<dyn Future<Output = Result<Box<dyn AssetDynamic>>> + Send>> {
    Box::pin(async_texture_async(server,path,t,params))
}
