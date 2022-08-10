use std::{collections::HashMap, sync::{Arc}};
use bevy_ecs::prelude::Res;
use crossbeam_channel::{self, Receiver, Sender, TryRecvError};
use parking_lot::RwLock;
use uuid::Uuid;
use seija_core::smol;
use crate::{Asset, Assets, HandleId, asset::{AssetLoader, AssetLoaderParams}, loader::{LoadingTrack, TrackState}, HandleUntyped, AssetDynamic};



#[derive(Clone)]
pub struct AssetServer {
   pub inner:Arc<AssetServerInner>
}

pub struct AssetServerInner {
    pub ref_counter:AssetRefCounter,
    pub lifecycle_events:RwLock<HashMap<Uuid,LifecycleEventChannel>>,
    loaders:RwLock<HashMap<Uuid,Arc<dyn AssetLoader>>>
}

impl AssetServer {
    pub fn new() -> AssetServer {
        AssetServer {
            inner:Arc::new(
                AssetServerInner {
                    ref_counter:AssetRefCounter::default(),
                    lifecycle_events:Default::default(),
                    loaders:RwLock::new(HashMap::default()),
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
    
    pub async fn _load_async<T:Asset>(server:AssetServer,path:&str,loading_track:LoadingTrack,params:Option<Box<dyn AssetLoaderParams>>) -> bool {
        if let Some(loader) = server.inner.loaders.read().get(&T::TYPE_UUID) {
            loading_track.set_state(TrackState::Loading);
            match loader.clone().load(server.clone(),loading_track.clone(),path, params).await {
                Ok(asset) => {
                    let events = server.inner.lifecycle_events.write();
                    if let Some(event) = events.get(&T::TYPE_UUID) {
                        event.sender.send(LifecycleEvent::Create(asset, loading_track.handle().id)).unwrap();
                    }
                },
                Err(err) => {
                    loading_track.set_state(TrackState::Fail);
                    log::error!("{}",err); 
                },
            }
        }
        false
    }

    pub fn load_async<T:Asset>(&self,path:&str,params:Option<Box<dyn AssetLoaderParams>>) -> Option<LoadingTrack> {
        if !self.inner.loaders.read().contains_key(&T::TYPE_UUID) { return None; }
        let hid = HandleId::random::<T>();
        let h_untyped = self.make_handle_untyped(hid);
        let loading_track = LoadingTrack::new(h_untyped);    
        let path_string = path.to_string();
        let clone_server = self.clone();
        let clone_track = loading_track.clone();
        //smol::spawn(async move {
        //    Self::_load_async(clone_server, &path_string, loading_track, params)
        //}).detach();
        return Some(clone_track);
    }

    fn make_handle_untyped(&self,id:HandleId) -> HandleUntyped {
        let sender = self.inner.ref_counter.channel.sender.clone();
        HandleUntyped::strong(id, sender)
    }


    pub fn free_unused_assets(&self) {
        let ref_receiver = &self.inner.ref_counter.channel.receiver;
        let mut ref_map = self.inner.ref_counter.ref_counts.write();
        let mut free_list:Vec<HandleId> = Vec::new();
        loop {
            let ref_event = match ref_receiver.try_recv() {
                Ok(v) => v,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("RefChange channel disconnected."),
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
                         channel.sender.send(LifecycleEvent::Free(id)).unwrap();
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
        let (sender, receiver) = crossbeam_channel::unbounded();
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
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self {sender,receiver}
    }
}

pub enum LifecycleEvent {
    Create(Box<dyn AssetDynamic>,HandleId),
    Free(HandleId),
} 

pub fn free_unused_assets_system(asset_server: Res<AssetServer>) {
    asset_server.free_unused_assets();
}