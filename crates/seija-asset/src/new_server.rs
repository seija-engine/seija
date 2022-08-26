use std::{collections::{HashMap, VecDeque},  sync::{Arc, atomic::{AtomicU8, Ordering}}, future::Future, pin::Pin};
use bevy_ecs::world::World;
use downcast_rs::DowncastSync;
use parking_lot::RwLock;
use seija_core::uuid::Uuid;
use seija_core::{smol_str::SmolStr,anyhow::Result};
use crate::{HandleId, Asset, AssetDynamic, loading_queue::AssetLoadingQueue};
use thiserror::Error;

#[derive(Debug,Error)]
pub enum AssetError {
    #[error("not found loader")]
    NotFoundLoader
}

struct AssetInfo {
    handle_id:HandleId,
    state:AtomicU8
}

impl AssetInfo {
    pub(crate) fn new<T:Asset>() -> Self {
        let id = HandleId::random::<T>();
        AssetInfo { handle_id:id, state: AtomicU8::new(0) }
    }

    pub(crate) fn set_finish(&self) {
        self.state.store(1, Ordering::SeqCst);
    }
}

struct AssetRequest {
    asset:Arc<AssetInfo>
}

impl AssetRequest {
    pub(crate) fn new(asset:Arc<AssetInfo>) -> Self {
        AssetRequest { asset }
    }
}

pub type AssetTouchFn = fn(server:AssetServer) -> Pin<Box<dyn Future<Output = Box<dyn DowncastSync>> + Send>>;
pub type AssetPerpareFn = fn(world:&mut World,touch_data:Option<&mut Box<dyn DowncastSync>>) -> Option<Box<dyn DowncastSync>>;
pub type AssetAsyncLoadFn = fn(server:AssetServer,touch_data:Option<&mut Box<dyn DowncastSync>>) -> Pin<Box<dyn Future<Output = Result<Box<dyn AssetDynamic>>> + Send>>;
pub struct TypeLoader {
    pub typ:Uuid,
    pub sync_loader:fn(w:&mut World) -> Result<Box<dyn AssetDynamic>>,
    /*
        async fn run(_self: &Type) { }
        Box::pin(run(self))
    */
    pub async_touch:Option<AssetTouchFn>,
    pub perpare:Option<AssetPerpareFn>,
    pub async_load:AssetAsyncLoadFn
}



#[derive(Clone)]
pub struct AssetServer {
   inner:Arc<AssetServerInner>
}
struct AssetServerInner {
    asset_infos:RwLock<HashMap<SmolStr,Arc<AssetInfo>>>,
    loaders:RwLock<HashMap<Uuid,Arc<TypeLoader>>>,
    request_list:RwLock<VecDeque<(SmolStr,HandleId,Arc<TypeLoader>)>>
}


impl AssetServer {
    pub fn register_loader<T:Asset>(&self,loader:TypeLoader) {
        self.inner.loaders.write().insert(T::TYPE_UUID, loader.into());
    }

    pub fn get_loader(&self,uuid:&Uuid) -> Result<Arc<TypeLoader>> {
        self.inner.loaders.read().get(&uuid).cloned().ok_or(AssetError::NotFoundLoader.into())
    }

    pub fn load_async<T:Asset>(&self,path:&str) -> Result<AssetRequest> {
        if let Some(info) = self.inner.asset_infos.read().get(path) {
            return Ok(AssetRequest::new(info.clone()));
        }
        let asset_info = Arc::new(AssetInfo::new::<T>());
        self.inner.asset_infos.write().insert(path.into(), asset_info.clone());

        let loader = self.get_loader(&T::TYPE_UUID)?;
        self.inner.request_list.write().push_back((SmolStr::new(path),asset_info.handle_id,loader));
        Ok(AssetRequest::new(asset_info))
    }

    pub(crate) fn add_dyn_asset(&self,typ:&Uuid,hid:HandleId,asset:Box<dyn AssetDynamic>) {

    }
}


pub(crate) fn update_asset_system(server:&AssetServer,loading_queue:&mut AssetLoadingQueue,world:&mut World) {
    let mut mut_req_list = server.inner.request_list.write();
    while let Some((uri,hid,loader)) = mut_req_list.pop_front() {
        loading_queue.push_uri(uri,hid,server.clone(),loader,world);
    }

    loading_queue.update(server,world);
}