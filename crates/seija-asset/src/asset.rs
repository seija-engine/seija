
use bevy_ecs::prelude::World;
use downcast_rs::{DowncastSync,impl_downcast, Downcast};
use async_trait::async_trait;
use seija_core::type_uuid::{TypeUuid, TypeUuidDynamic};
use seija_core::{anyhow::{Result},smol};

use crate::{AssetServer};
use crate::loader::LoadingTrack;
pub trait Asset : TypeUuid + AssetDynamic { }

pub trait AssetDynamic: TypeUuidDynamic + Send + Sync + Downcast + 'static {}
impl_downcast!(AssetDynamic);
impl<T> Asset for T where T: TypeUuid + AssetDynamic + TypeUuidDynamic {}

impl<T> AssetDynamic for T where T: Send + Sync + 'static + TypeUuidDynamic {}

#[async_trait]
pub trait AssetLoader : Send + Sync + 'static {
  
   async fn load(&self,server:AssetServer,track:Option<LoadingTrack>,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>>;
   
   fn load_sync(&self,_:&mut World,path:&str,asset_server:AssetServer,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> { 
      smol::block_on(async move {
         self.load(asset_server, None, path, params).await
      })
   }

   
}

pub trait AssetLoaderParams:DowncastSync {}
impl_downcast!(AssetLoaderParams);