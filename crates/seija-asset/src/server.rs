use std::{collections::HashMap, sync::{Arc}};
use bevy_ecs::prelude::Res;
use crossbeam_channel::{self, Receiver, Sender, TryRecvError};
use parking_lot::RwLock;
use uuid::Uuid;
use crate::{Asset, Assets, HandleId};

pub struct AssetServer {
    pub ref_counter:AssetRefCounter,
    pub lifecycle_events:RwLock<HashMap<Uuid,LifecycleEventChannel>>
}

impl AssetServer {
    pub fn new() -> AssetServer {
        AssetServer { 
            ref_counter:AssetRefCounter::default(),
            lifecycle_events:Default::default()
        }
    }

    pub fn register_type<T:Asset>(&self) -> Assets<T> {
        self.lifecycle_events.write().insert(T::TYPE_UUID, LifecycleEventChannel::default());
        Assets::new(self.ref_counter.channel.sender.clone())

    }

    pub fn free_unused_assets(&self) {
        let ref_receiver = &self.ref_counter.channel.receiver;
        let mut ref_map = self.ref_counter.ref_counts.write();
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
            let lifecycle_events = self.lifecycle_events.read();
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
    Create(HandleId),
    Free(HandleId),
} 

pub fn free_unused_assets_system(asset_server: Res<AssetServer>) {
    asset_server.free_unused_assets();
}