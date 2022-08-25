use std::{collections::{HashMap, VecDeque}, any::Any, sync::Arc, future::Future, pin::Pin};
use bevy_ecs::world::World;
use downcast_rs::DowncastSync;
use parking_lot::RwLock;
use seija_core::uuid::Uuid;
use seija_core::{smol_str::SmolStr,anyhow::Result,smol};
use crate::{HandleId, Asset, AssetDynamic};
use thiserror::Error;

#[derive(Debug,Error)]
pub enum AssetError {
    #[error("not found loader")]
    NotFoundLoader
}

struct AssetInfo {
    handle_id:HandleId
}

pub struct TypeLoader {
    sync_loader:fn(w:&mut World) -> Result<Box<dyn AssetDynamic>>,
    /*
        async fn run(_self: &Type) { }
        Box::pin(run(self))
    */
    async_touch:Option<fn(server:AssetServer) -> Pin<Box<dyn Future<Output = Box<dyn DowncastSync>> + Send>>>,
    perpare:Option<fn(world:&mut World,touch_data:&mut Box<dyn DowncastSync>)>,
}

struct LoadContext {
    touch_data: Box<dyn DowncastSync>
}

impl LoadContext {
    pub fn new(data:Box<dyn DowncastSync>) -> Self {
        LoadContext {
            touch_data:data
        }
    }
}

#[derive(Clone)]
pub struct AssetServer {
   inner:Arc<AssetServerInner>
}
struct AssetServerInner {
    asset_infos:RwLock<HashMap<SmolStr,AssetInfo>>,
    loaders:RwLock<HashMap<Uuid,Arc<TypeLoader>>>,
    loadings:VecDeque<LoadContext>
}


impl AssetServer {
    
    pub fn register_loader<T:Asset>(&self,loader:TypeLoader) {
        self.inner.loaders.write().insert(T::TYPE_UUID, loader.into());
    }

    pub fn get_loader(&self,uuid:&Uuid) -> Result<Arc<TypeLoader>> {
        self.inner.loaders.read().get(&uuid).cloned().ok_or(AssetError::NotFoundLoader.into())
    }

    pub fn load_async<T:Asset>(&self,path:&str) -> Result<()> {
        if let Some(info) = self.inner.asset_infos.read().get(path) {
            return Ok(());
        }
        let loader = self.get_loader(&T::TYPE_UUID)?;
        if let Some(touch_fn) = loader.async_touch {
            let server:AssetServer = self.clone();
            smol::spawn(async move {
                let touch_data = touch_fn(server).await;
                
            }).detach();
        }
        Ok(())
    }
    

    pub fn update(&mut self,world:&mut World) {
    }

}
