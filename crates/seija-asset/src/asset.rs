use downcast_rs::{DowncastSync,impl_downcast, Downcast};
use async_trait::async_trait;
use seija_core::type_uuid::{TypeUuid, TypeUuidDynamic};
use seija_core::{anyhow::{Result}};

use crate::AssetServer;
use crate::loader::LoadingTrack;
pub trait Asset : TypeUuid + AssetDynamic { }

pub trait AssetDynamic: TypeUuidDynamic + Send + Sync + Downcast + 'static {}
impl_downcast!(AssetDynamic);
impl<T> Asset for T where T: TypeUuid + AssetDynamic + TypeUuidDynamic {}

impl<T> AssetDynamic for T where T: Send + Sync + 'static + TypeUuidDynamic {}

#[async_trait]
pub trait AssetLoader : Send + Sync + 'static {
   async fn load(&self,server:AssetServer,track:Option<LoadingTrack>,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>>;
}

pub trait AssetLoaderParams:DowncastSync {}
impl_downcast!(AssetLoaderParams);