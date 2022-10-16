use async_trait::{async_trait};
use bevy_ecs::prelude::World;
use downcast_rs::{DowncastSync,impl_downcast, Downcast};
use seija_core::smol_str::SmolStr;
use seija_core::type_uuid::{TypeUuid, TypeUuidDynamic};
use seija_core::{anyhow::{Result,anyhow}};
use uuid::Uuid;

use crate::errors::AssetError;
use crate::{AssetServer, HandleUntyped, Assets};
pub trait Asset : TypeUuid + AssetDynamic { }

pub trait AssetDynamic: TypeUuidDynamic + Send + Sync + Downcast + 'static {}
impl_downcast!(AssetDynamic);
impl<T> Asset for T where T: TypeUuid + AssetDynamic + TypeUuidDynamic {}

impl<T> AssetDynamic for T where T: Send + Sync + 'static + TypeUuidDynamic {}


pub trait AssetLoaderParams:DowncastSync {}
impl_downcast!(AssetLoaderParams);


#[derive(PartialEq, Eq)]
pub enum AsyncLoadMode {
    Touch,
    Perpare,
    OnlyLoad
}

pub fn add_to_asset_type<T:Asset>(world:&mut World,res:Box<dyn AssetDynamic>) -> Result<HandleUntyped> {
    let mut assets = world.get_resource_mut::<Assets<T>>().ok_or(AssetError::TypeCastError)?;
    let res = res.downcast::<T>().map_err(|_| AssetError::TypeCastError)?;
    let handle = assets.add(*res);
    Ok(handle.untyped())
}

#[async_trait]
pub trait IAssetLoader: Send + Sync + 'static {
    fn typ(&self) -> Uuid;
    fn add_to_asset(&self,world:&mut World,res:Box<dyn AssetDynamic>) -> Result<HandleUntyped>;
    fn mode(&self) -> AsyncLoadMode { AsyncLoadMode::OnlyLoad }
    fn sync_load(&self,w:&mut World,path:&str,server:&AssetServer,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>>;
    async fn async_touch(&self,_server:AssetServer,_path:SmolStr) -> Result<Box<dyn DowncastSync>> { Err(anyhow!("zero")) }
    fn perpare(&self,_world:&mut World,_touch_data:Option<Box<dyn DowncastSync>>) -> Option<Box<dyn DowncastSync>> { None }
    async fn async_load(&self,server:AssetServer,path:SmolStr,
                        touch_data:Option<Box<dyn DowncastSync>>,
                        params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>>;
}