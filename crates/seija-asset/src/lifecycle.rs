use std::{sync::Arc, collections::HashMap};
use parking_lot::RwLock;
use seija_core::smol;
use seija_core::smol::channel::{Sender, Receiver, TryRecvError};
use uuid::Uuid;

use crate::server::AssetInfo;
use crate::{HandleId, AssetDynamic};

pub enum RefEvent {
    Increment(HandleId),
    Decrement(HandleId),
}

pub struct AssetRefCounter {
    pub channel: Arc<RefEventChannel>,
    ref_counts: RwLock<HashMap<HandleId, usize>>,
}

pub struct RefEventChannel {
    pub sender: Sender<RefEvent>,
    receiver: Receiver<RefEvent>,
}

impl Default for AssetRefCounter {
    fn default() -> Self {
        let (sender, receiver) = smol::channel::unbounded();
        Self {
            channel: Arc::new(RefEventChannel { sender, receiver }),
            ref_counts: Default::default(),
        }
    }
}

pub struct LifecycleEventChannel {
    pub sender: Sender<LifecycleEvent>,
    pub receiver: Receiver<LifecycleEvent>,
}

impl Default for LifecycleEventChannel {
    fn default() -> Self {
        let (sender, receiver) = smol::channel::unbounded();
        Self { sender, receiver }
    }
}

pub enum LifecycleEvent {
    Create(Box<dyn AssetDynamic>, HandleId,Arc<AssetInfo>),
    Free(HandleId),
}

#[derive(Default)]
pub(crate) struct AssetLifeCycle {
    pub ref_counter: AssetRefCounter,
    pub lifecycle_events: RwLock<HashMap<Uuid, LifecycleEventChannel>>,
}

impl AssetLifeCycle {
    pub fn register(&self,typ_id:&Uuid) {
        self.lifecycle_events
            .write()
            .insert(typ_id.clone(), LifecycleEventChannel::default());
    }

    pub fn sender(&self) -> Sender<RefEvent> {
        self.ref_counter.channel.sender.clone()
    }


    pub fn free_unused_assets(&self) {
        let ref_receiver = &self.ref_counter.channel.receiver;
        let mut ref_map = self.ref_counter.ref_counts.write();
        let mut free_list: Vec<HandleId> = Vec::new();
        loop {
            let ref_event = match ref_receiver.try_recv() {
                Ok(v) => v,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Closed) => panic!("RefChange channel disconnected."),
            };
            match ref_event {
                RefEvent::Increment(id) => {
                    *ref_map.entry(id).or_insert(0) += 1;
                }
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
                    if let Some(channel) = lifecycle_events.get(id.typ()) {
                        channel.sender.try_send(LifecycleEvent::Free(id)).unwrap();
                    }
                }
            }
        }
    }
}