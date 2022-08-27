use crate::{
    Asset, Assets, lifecycle::AssetLifeCycle, HandleId, RefEvent, 
    asset::TypeLoader, AssetDynamic, Handle, errors::AssetError, HandleUntyped, LifecycleEvent, AssetLoaderParams,
};
use bevy_ecs::{prelude::{Res, World}};
use parking_lot::RwLock;
use relative_path::RelativePath;
use seija_core::{anyhow::Result, smol_str::SmolStr, smol::channel::Sender};
use uuid::Uuid;
use std::{
    path::{PathBuf},
    sync::{Arc, atomic::{AtomicU8,Ordering}}, collections::{HashMap, VecDeque}};


pub struct AssetInfo {
    handle_id:HandleId,
    state:AtomicU8,
    sender:Sender<RefEvent>
}

impl AssetInfo {
    pub(crate) fn new<T:Asset>(sender:Sender<RefEvent>) -> Self {
        let id = HandleId::random::<T>();
        AssetInfo { handle_id:id, state: AtomicU8::new(0),sender }
    }

    pub(crate) fn new_id(id:HandleId,sender:Sender<RefEvent>) -> Self {
        AssetInfo { handle_id: id, state: AtomicU8::new(0),sender }
    }

    pub(crate) fn set_finish(&self) {
        self.state.store(1, Ordering::SeqCst);
    }

    pub(crate) fn is_finish(&self) -> bool {
        self.state.load(Ordering::SeqCst) == 1
    }

    pub fn make_handle(&self) -> HandleUntyped {
        HandleUntyped::strong(self.handle_id, self.sender.clone())
    }

    pub fn make_weak_handle(&self) -> HandleUntyped {
        HandleUntyped::weak(self.handle_id)
    }
}

pub struct AssetRequest {
    asset:Arc<AssetInfo>
}

impl AssetRequest {
    pub(crate) fn new(asset:Arc<AssetInfo>) -> Self {
        AssetRequest { asset }
    }

    pub fn is_finish(&self) -> bool {
        self.asset.is_finish()
    }

    pub fn make_handle(&self) -> HandleUntyped {
        self.asset.make_handle()
    }

    pub fn make_weak_handle(&self) -> HandleUntyped {
        self.asset.make_weak_handle()
    }
    
}

#[derive(Clone)]
pub struct AssetServer {
    pub inner: Arc<AssetServerInner>,
    
}


pub struct AssetServerInner {
    pub root_path: PathBuf,
    pub(crate) life_cycle:AssetLifeCycle,
    assets:RwLock<HashMap<SmolStr,Arc<AssetInfo>>>,
    loaders:RwLock<HashMap<Uuid,Arc<TypeLoader>>>,
    pub(crate) request_list:Arc<RwLock<VecDeque<(SmolStr,HandleId,Arc<TypeLoader>)>>>
}

impl AssetServer {
    pub fn new(root_path: PathBuf) -> AssetServer {
        log::info!("init asset server:{:?}", root_path.as_path());
        
        AssetServer {
            inner: Arc::new(AssetServerInner {
                root_path,
                life_cycle:Default::default(),
                assets:RwLock::new(HashMap::default()),
                loaders:Default::default(),
                request_list:Default::default()
            }),
        }
    }

    pub fn register_type<T: Asset>(&self) -> Assets<T> {
        self.inner.life_cycle.register(&T::TYPE_UUID);
        Assets::new(self.inner.life_cycle.sender())
    }

    pub fn register_loader<T:Asset>(&self,loader:TypeLoader) {
        self.inner.loaders.write().insert(T::TYPE_UUID.clone(), Arc::new(loader));
    }

    pub fn full_path(&self, path: &str) -> Result<PathBuf> {
        Ok(RelativePath::from_path(path)?.to_logical_path(&self.inner.root_path))
    }

    pub fn set_asset(&self,path:&str,id:HandleId) {
       let asset_info = AssetInfo::new_id(id,self.inner.life_cycle.sender());
       self.inner.assets.write().insert(SmolStr::new(path), Arc::new(asset_info));
    }

    pub fn get_asset(&self,path:&str) -> Option<Arc<AssetInfo>> {
        self.inner.assets.read().get(path).cloned()
    }

    pub fn add_dyn_asset(&self,path:&str,typ:&Uuid,hid:HandleId,asset:Box<dyn AssetDynamic>) {
        let info = if let Some(info) = self.inner.assets.read().get(path) {
            info.clone()
        } else {
            let info = Arc::new(AssetInfo::new_id(hid, self.inner.life_cycle.sender()));
            self.inner.assets.write().insert(SmolStr::new(path), info.clone());
            info
        };
        let events = self.inner.life_cycle.lifecycle_events.write();
        if let Some(event) = events.get(typ) {
            event.sender.try_send(LifecycleEvent::Create(asset,hid,info)).unwrap();
        }
    }

    pub fn create_asset<T:Asset>(&self,asset:T,path:&str) -> Handle<T> {
        let h = Handle::<T>::strong(HandleId::random::<T>(), self.inner.life_cycle.sender());
        self.add_dyn_asset(path,&T::TYPE_UUID,h.id,Box::new(asset));
        h
    }

    pub fn load_sync<T:Asset>(&self,world:&mut World,path:&str) -> Result<Handle<T>> {
        if let Some(info) = self.inner.assets.read().get(path) {
            if info.is_finish() {
                return Ok(info.make_handle().typed::<T>());
            }
            let mut wait = parking_lot_core::SpinWait::default();
            loop {
                if info.is_finish() {
                    return Ok(info.make_handle().typed::<T>());
                }
                wait.spin();
            }
        }
        let loader = self.inner.loaders.read().get(&T::TYPE_UUID).ok_or(AssetError::NotFoundLoader)?.clone();
        let func = loader.sync_load;
        let load_asset = func(world,path,self)?;
        let boxed_asset = load_asset.downcast::<T>().map_err(|_| AssetError::TypeCastError)?;
        let mut assets = world.get_resource_mut::<Assets<T>>().ok_or(AssetError::TypeCastError)?;
        let handle = assets.add(*boxed_asset);
        let info = Arc::new( AssetInfo::new_id(handle.id, self.inner.life_cycle.sender()));
        self.inner.assets.write().insert(SmolStr::new(path), info);
        Ok(handle)
    }

    pub fn load_async<T:Asset>(&self,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Result<AssetRequest> {
        if let Some(info) = self.inner.assets.read().get(path) {
            return Ok(AssetRequest::new(info.clone()));
        }
        let asset_info = Arc::new(AssetInfo::new::<T>(self.inner.life_cycle.sender()));
        self.inner.assets.write().insert(path.into(), asset_info.clone());

        let loader = self.inner.loaders.read().get(&T::TYPE_UUID).ok_or(AssetError::NotFoundLoader)?.clone();
        self.inner.request_list.write().push_back((SmolStr::new(path),asset_info.handle_id,loader));
        Ok(AssetRequest::new(asset_info))
    }
}



pub fn free_unused_assets_system(asset_server: Res<AssetServer>) {
    asset_server.inner.life_cycle.free_unused_assets();
}
