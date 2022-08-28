use bevy_ecs::prelude::World;
use seija_asset::IAssetLoader;
use seija_asset::async_trait::async_trait;
use smol_str::SmolStr;
use seija_asset::{AssetLoaderParams, AssetServer, AssetDynamic, downcast_rs::DowncastSync};
use seija_core::TypeUuid;
use seija_core::anyhow::Result;
use crate::resource::Texture;
use seija_core::smol;
use super::TextureDescInfo;

#[derive(Default)]
pub(crate) struct  TextureLoader;
#[async_trait]
impl IAssetLoader for TextureLoader {
    fn typ(&self) -> uuid::Uuid { Texture::TYPE_UUID }

    fn sync_load(&self,_:&mut World,path:&str,server:&AssetServer,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path)?;
        let bytes = std::fs::read(full_path)?;
        let texture = Texture::from_image_bytes(&bytes, read_desc(params))?;
        Ok(Box::new(texture))
    }

    async fn async_load(&self,server:AssetServer,path:SmolStr,
                        _:Option<Box<dyn DowncastSync>>,
                        params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path.as_str())?;
        let bytes = smol::fs::read(full_path).await?;
        let texture = Texture::from_image_bytes(&bytes, read_desc(params))?;
        Ok(Box::new(texture))
    }
}

impl AssetLoaderParams for TextureDescInfo {}

fn read_desc(params:Option<Box<dyn AssetLoaderParams>>) -> TextureDescInfo {
    params.and_then(|v| v.downcast::<TextureDescInfo>().ok())
    .map(|v| *v)
    .unwrap_or(Default::default())
}
