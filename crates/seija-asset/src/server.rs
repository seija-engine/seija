use std::{collections::HashMap, sync::{Arc}, path::{PathBuf, Path}};
use bevy_ecs::prelude::Res;
use seija_core::smol::channel::{ Receiver, Sender, TryRecvError};
use parking_lot::RwLock;
use relative_path::{ RelativePath};
use uuid::Uuid;
use seija_core::{smol, smol_str::SmolStr};
use crate::{Asset, Assets, HandleId, asset::{AssetLoader, AssetLoaderParams}, loader::{LoadingTrack, TrackState},  AssetDynamic, Handle};
use seija_core::anyhow::Result;


#[derive(Clone)]
pub struct AssetServer {
   pub inner:Arc<AssetServerInner>
}

pub struct AssetMeta {
    track:Option<LoadingTrack>
}

impl AssetMeta {
    pub fn new(track:Option<LoadingTrack>) -> Self {
        AssetMeta { 
            track
        }
    }
}

pub struct AssetServerInner {
    pub root_path:PathBuf,
    pub ref_counter:AssetRefCounter,
    pub lifecycle_events:RwLock<HashMap<Uuid,LifecycleEventChannel>>,
    loaders:RwLock<HashMap<Uuid,Arc<dyn AssetLoader>>>,
    assets:RwLock<HashMap<SmolStr,AssetMeta>>,
    handle_to_path:RwLock<HashMap<HandleId,SmolStr>>
}

impl AssetServer {
    pub fn new(root_path:PathBuf) -> AssetServer {
        log::info!("init asset server:{:?}",root_path.as_path());
        AssetServer {
            inner:Arc::new(
                AssetServerInner {
                    root_path,
                    ref_counter:AssetRefCounter::default(),
                    lifecycle_events:Default::default(),
                    loaders:RwLock::new(HashMap::default()),
                    assets:RwLock::new(HashMap::default()),
                    handle_to_path:RwLock::new(HashMap::default()),
                }
            )
        }
    }
    
    pub fn inner(&self) -> &AssetServerInner {
        &self.inner
    }

    pub fn register_type<T:Asset>(&self) -> Assets<T> {
        self.inner.lifecycle_events.write().insert(T::TYPE_UUID, LifecycleEventChannel::default());
        Assets::new(self.get_ref_sender())
    }

    pub fn get_ref_sender(&self) -> Sender<RefEvent> {
        self.inner.ref_counter.channel.sender.clone()
    }

    pub fn register_loader(&self,uuid:Uuid,loader:impl AssetLoader) {
        self.inner.loaders.write().insert(uuid, Arc::new(loader));
    }

    pub async fn read_bytes<P:AsRef<Path>>(&self,path:P) -> Result<Vec<u8>> {
        if path.as_ref().is_relative() {
            let path = RelativePath::from_path(path.as_ref())?;
            let full_path = path.to_logical_path(&self.inner().root_path);
            smol::fs::read(full_path).await.map_err(|e| e.into())
        } else {
            smol::fs::read(path).await.map_err(|e| e.into())
        }
    }

    
    
    async fn _load_async<T:Asset,P:AsRef<Path>>(&self,path:P,loading_track:LoadingTrack,params:Option<Box<dyn AssetLoaderParams>>) {
        let loader = self.inner.loaders.read().get(&T::TYPE_UUID).cloned();
        if let Some(loader) = loader {
            loading_track.set_state(TrackState::Loading);
            match loader.load(self.clone(),Some(loading_track.clone()),path.as_ref().to_str().unwrap(), params).await {
                Ok(asset) => {
                    self.create_dyn_asset(path,&T::TYPE_UUID, asset, loading_track.handle_id().clone(),Some(loading_track));
                },
                Err(err) => {
                    loading_track.set_state(TrackState::Fail);
                    self.remove_asset_meta(loading_track.handle_id());
                    log::error!("load async {:?} error: {}",path.as_ref(),err); 
                },
            }
        }
    }

   

    pub fn load_async<T:Asset>(&self,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Option<LoadingTrack> {       
        if let Some(track) = self.inner.assets.read().get(path).and_then(|info|info.track.clone()) {
            return Some(track);
        }
        if !self.inner.loaders.read().contains_key(&T::TYPE_UUID) { return None; }
        let loading_track =  LoadingTrack::new(HandleId::random::<T>(),self.get_ref_sender());
        let clone_server = self.clone();
        let clone_track = loading_track.clone();
        let full_path = RelativePath::from_path(path).ok()?.to_logical_path(&self.inner().root_path);
        self.add_asset_meta(path, loading_track.clone());
        smol::spawn(async move {
            clone_server._load_async::<T,PathBuf>(full_path, loading_track, params).await;
        }).detach();
        return Some(clone_track);
    }

    fn create_dyn_asset<P:AsRef<Path>>(&self,p:P,uuid:&Uuid,asset:Box<dyn AssetDynamic>,hid:HandleId,track:Option<LoadingTrack>) {
        
        {
            let strip_path = SmolStr::from(p.as_ref().strip_prefix(&self.inner().root_path).unwrap().to_str().unwrap());

            let mut write_assets = self.inner().assets.write();
            write_assets.insert(strip_path.clone(), AssetMeta::new(track.clone()));
      
            self.inner().handle_to_path.write().insert(hid, strip_path);
        };
        let events = self.inner.lifecycle_events.write();
        if let Some(event) = events.get(uuid) {
            log::error!("send create:{:?}",p.as_ref());
            event.sender.try_send(LifecycleEvent::Create(asset,hid,track)).unwrap();
        }
    }

    pub fn create_asset<T:Asset,P:AsRef<Path>>(&self,asset:T,path:P,track:Option<LoadingTrack>) -> Handle<T> {
        let sender = self.inner.ref_counter.channel.sender.clone();
        let h = Handle::<T>::strong(HandleId::random::<T>(), sender);
        self.create_dyn_asset(path,&T::TYPE_UUID, Box::new(asset), h.id,track);
        h
    }

    fn add_asset_meta(&self,path:&str,track:LoadingTrack) {
        let hid = track.handle_id().clone();
        let mut write_assets = self.inner().assets.write();
        let path = SmolStr::new(path);
        write_assets.insert(path.clone(), AssetMeta::new(Some(track)));
        self.inner().handle_to_path.write().insert(hid, path);
    }

    fn remove_asset_meta(&self,id:&HandleId) {
        if let Some(path) = self.inner.handle_to_path.write().remove(id) {
            self.inner.assets.write().remove(path.as_str());
        }
    }

    pub fn free_unused_assets(&self) {
        let ref_receiver = &self.inner.ref_counter.channel.receiver;
        let mut ref_map = self.inner.ref_counter.ref_counts.write();
        let mut free_list:Vec<HandleId> = Vec::new();
        loop {
            let ref_event = match ref_receiver.try_recv() {
                Ok(v) => v,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Closed) => panic!("RefChange channel disconnected."),
            };
            match ref_event {
                RefEvent::Increment(id) => {
                    *ref_map.entry(id).or_insert(0) += 1;
                },
                RefEvent::Decrement(id) => {
                    let entry = ref_map.entry(id).or_insert(0);
                    *entry -= 1;
                    if *entry == 0 {
                       free_list.push(id);
                    }
                }
            }
        }

        if !free_list.is_empty() {
            let lifecycle_events = self.inner.lifecycle_events.read();
            for id in free_list {
                if ref_map.get(&id).cloned().unwrap_or(0) == 0 {
                     if  let Some(channel) = lifecycle_events.get(id.typ()) {
                         channel.sender.try_send(LifecycleEvent::Free(id)).unwrap();
                         self.remove_asset_meta(&id);
                     }
                }
            }
        }
    }
}


pub enum RefEvent {
    Increment(HandleId),
    Decrement(HandleId),
}


pub struct AssetRefCounter {
    pub channel:Arc<RefEventChannel>,
    ref_counts:RwLock<HashMap<HandleId,usize>>
}

pub struct RefEventChannel {
    pub sender:Sender<RefEvent>,
    receiver:Receiver<RefEvent>,
}

impl Default for AssetRefCounter {
    fn default() -> Self {
        let (sender, receiver) = smol::channel::unbounded();
        Self {
            channel: Arc::new(RefEventChannel {sender,receiver}),
            ref_counts: Default::default() 
        }  
    }
}

pub struct LifecycleEventChannel {
    pub sender:Sender<LifecycleEvent>,
    pub receiver:Receiver<LifecycleEvent>,
}

impl Default for LifecycleEventChannel {
    fn default() -> Self {
        let (sender, receiver) = smol::channel::unbounded();
        Self {sender,receiver}
    }
}

pub enum LifecycleEvent {
    Create(Box<dyn AssetDynamic>,HandleId,Option<LoadingTrack>),
    Free(HandleId),
} 

pub fn free_unused_assets_system(asset_server: Res<AssetServer>) {
    asset_server.free_unused_assets();
}