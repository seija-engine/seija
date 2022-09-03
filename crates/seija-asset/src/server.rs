use crate::{
    Asset, Assets, lifecycle::AssetLifeCycle, HandleId, RefEvent,
    AssetDynamic, Handle, errors::AssetError, HandleUntyped, LifecycleEvent, AssetLoaderParams, IAssetLoader,
};
use bevy_ecs::{prelude::{Res, World}};
use parking_lot::RwLock;
use relative_path::RelativePath;
use seija_core::{anyhow::{Result,anyhow}, smol_str::SmolStr, smol::channel::Sender};
use uuid::Uuid;
use std::{
    path::{PathBuf},
    sync::{Arc, atomic::{AtomicU8,Ordering}}, collections::{HashMap, VecDeque}, future::Future, task::{Poll}};


pub struct AssetInfo {
    handle_id:HandleId,
    state:AtomicU8,
    sender:Sender<RefEvent>,
}

pub struct ArcAssetInfo(pub Arc<AssetInfo>);

impl AssetInfo {
    //#[warn(dead_code)]
    //pub(crate) fn new<T:Asset>(sender:Sender<RefEvent>) -> Self {
    //    let id = HandleId::random::<T>();
    //    AssetInfo { handle_id:id, state: AtomicU8::new(0),sender,waker:Default::default() }
    //}

    pub(crate) fn new_untyped(typ:&Uuid,sender:Sender<RefEvent>) -> Self {
        let id = HandleId::new(typ.clone(), rand::random());
        AssetInfo { handle_id:id, state: AtomicU8::new(0),sender}
    }

    pub(crate) fn new_id(id:HandleId,sender:Sender<RefEvent>) -> Self {
        AssetInfo { handle_id: id, state: AtomicU8::new(0),sender }
    }

    pub(crate) fn set_finish(&self) {
        self.state.store(1, Ordering::SeqCst);
    }

    pub(crate) fn set_fail(&self) {
        self.state.store(2, Ordering::SeqCst);
    }

    pub(crate) fn is_finish(&self) -> bool {
        self.state.load(Ordering::SeqCst) == 1
    }

    pub(crate) fn is_fail(&self) -> bool {
        self.state.load(Ordering::SeqCst) == 2
    }

    pub fn make_handle(&self) -> HandleUntyped {
        HandleUntyped::strong(self.handle_id, self.sender.clone())
    }

    pub fn make_weak_handle(&self) -> HandleUntyped {
        HandleUntyped::weak(self.handle_id)
    }
}

pub struct AssetRequest {
    asset:ArcAssetInfo
}

impl AssetRequest {
    pub(crate) fn new(asset:Arc<AssetInfo>) -> Self {
        AssetRequest { asset:ArcAssetInfo(asset) }
    }

    pub fn is_finish(&self) -> bool {
        self.asset.0.is_finish()
    }

    pub fn make_handle(&self) -> HandleUntyped {
        self.asset.0.make_handle()
    }

    pub fn make_weak_handle(&self) -> HandleUntyped {
        
        self.asset.0.make_weak_handle()
    }

    pub async fn wait_id(self) -> Option<HandleId> { self.asset.await }

    pub async fn wait_handle(self) -> Option<HandleUntyped> {
        let sender = self.asset.0.sender.clone();
        let id = self.wait_id().await;
        if let Some(id) = id {
            return Some(HandleUntyped::strong(id, sender));
        }
        None
    }
}

impl Future for ArcAssetInfo {
    type Output = Option<HandleId>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        //let waker = cx.waker().clone();
        //{
        //    let mut waker_lock = self.0.waker.lock().unwrap();
        //    *waker_lock = Some(waker);
        //};
       
        cx.waker().clone().wake();
        if self.0.is_finish() {
            Poll::Ready(Some(self.0.handle_id))
        } else if self.0.is_fail() {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
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
    loaders:RwLock<HashMap<Uuid,Arc<dyn IAssetLoader>>>,
    pub(crate) request_list:Arc<RwLock<VecDeque<(SmolStr,HandleId,Option<Box<dyn AssetLoaderParams>>,Arc<dyn IAssetLoader>)>>>
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

    pub fn register_loader<T:Asset,F:IAssetLoader>(&self,loader:F) {
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
        let read_info = self.inner.assets.read().get(path).cloned();
        let info = if let Some(info) = read_info {
            info
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

    pub fn load_sync<T:Asset>(&self,world:&mut World,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Handle<T>> {
      
        let info = self.inner.assets.read().get(path).cloned();
        if let Some(info) = info {
           
            if info.is_finish() {
                return Ok(info.make_handle().typed::<T>());
            }
            
            if !info.is_fail() {
                let mut wait = parking_lot_core::SpinWait::default();
                loop {
                    if info.is_finish() {
                        return Ok(info.make_handle().typed::<T>());
                    } else if info.is_fail() {
                        return Err(anyhow!("load fail"));
                    }
                    wait.spin();
                }
            }
        }
        let loader = self.inner.loaders.read().get(&T::TYPE_UUID).ok_or(AssetError::NotFoundLoader)?.clone();
        let load_asset = loader.sync_load(world,path,self,params)?;
        let boxed_asset = load_asset.downcast::<T>().map_err(|_| AssetError::TypeCastError)?;
        let mut assets = world.get_resource_mut::<Assets<T>>().ok_or(AssetError::TypeCastError)?;
        let handle = assets.add(*boxed_asset);
        let info = Arc::new( AssetInfo::new_id(handle.id, self.inner.life_cycle.sender()));
        info.set_finish();
        self.inner.assets.write().insert(SmolStr::new(path), info);
        Ok(handle)
    }

    pub fn load_async<T:Asset>(&self,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Result<AssetRequest> {
        self.load_async_untyped(&T::TYPE_UUID, path, params)
    }

    pub fn load_async_untyped(&self,typ:&Uuid,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Result<AssetRequest> {
        let read_info = self.inner.assets.read().get(path).cloned();
        if let Some(info) = read_info {
            if !info.is_fail() {
                log::info!("load_async_untyped cache:{}",path);
                return Ok(AssetRequest::new(info.clone()))
            }  
        }
       
        let asset_info = Arc::new(AssetInfo::new_untyped(typ,self.inner.life_cycle.sender()));
        self.inner.assets.write().insert(path.into(), asset_info.clone());
        log::info!("load_async_untyped:{}",path);

        let loader = self.inner.loaders.read().get(typ).ok_or(AssetError::NotFoundLoader)?.clone();
        self.inner.request_list.write().push_back((SmolStr::new(path),asset_info.handle_id,params,loader));
        Ok(AssetRequest::new(asset_info))
    }
}



pub fn free_unused_assets_system(asset_server: Res<AssetServer>) {
    asset_server.inner.life_cycle.free_unused_assets();
}
