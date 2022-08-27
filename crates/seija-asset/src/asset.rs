
use std::future::Future;
use std::pin::Pin;

use bevy_ecs::prelude::World;
use downcast_rs::{DowncastSync,impl_downcast, Downcast};
use seija_core::type_uuid::{TypeUuid, TypeUuidDynamic};
use seija_core::{anyhow::{Result},smol};
use uuid::Uuid;

use crate::{AssetServer};
use crate::loader::LoadingTrack;
pub trait Asset : TypeUuid + AssetDynamic { }

pub trait AssetDynamic: TypeUuidDynamic + Send + Sync + Downcast + 'static {}
impl_downcast!(AssetDynamic);
impl<T> Asset for T where T: TypeUuid + AssetDynamic + TypeUuidDynamic {}

impl<T> AssetDynamic for T where T: Send + Sync + 'static + TypeUuidDynamic {}


pub trait AssetLoaderParams:DowncastSync {}
impl_downcast!(AssetLoaderParams);


pub type AssetTouchFn = fn(server:AssetServer) -> Pin<Box<dyn Future<Output = Box<dyn DowncastSync>> + Send>>;
pub type AssetPerpareFn = fn(world:&mut World,touch_data:Option<&mut Box<dyn DowncastSync>>) -> Option<Box<dyn DowncastSync>>;
pub type AssetAsyncLoadFn = fn(server:AssetServer,touch_data:Option<&mut Box<dyn DowncastSync>>) 
                                 -> Pin<Box<dyn Future<Output = Result<Box<dyn AssetDynamic>>> + Send>>;
pub struct TypeLoader {
    pub typ:Uuid,
    pub sync_load:fn(w:&mut World,path:&str,server:&AssetServer) -> Result<Box<dyn AssetDynamic>>,
    /*
        async fn run(_self: &Type) { }
        Box::pin(run(self))
    */
    pub async_touch:Option<AssetTouchFn>,
    pub perpare:Option<AssetPerpareFn>,
    pub async_load:AssetAsyncLoadFn
}