use std::{collections::HashMap, fmt::Debug};

use bevy_ecs::prelude::{Res, ResMut};
use crossbeam_channel::{Sender, TryRecvError};
use seija_core::event::{EventWriter, Events};
use crate::{asset::Asset, handle::{Handle, HandleId}, server::{AssetServer, LifecycleEvent, RefEvent}};

pub enum AssetEvent<T: Asset> {
    Created { handle: Handle<T> },
    Modified { handle: Handle<T> },
    Removed { handle: Handle<T> },
}
impl<T: Asset> Debug for AssetEvent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetEvent::Created { handle } => 
              f.debug_struct(&format!("AssetEvent<{}>::Created",std::any::type_name::<T>())).field("handle", &handle.id).finish(),
            AssetEvent::Modified { handle } => 
              f.debug_struct(&format!("AssetEvent<{}>::Modified",std::any::type_name::<T>())).field("handle", &handle.id).finish(),
            AssetEvent::Removed { handle } => 
              f.debug_struct(&format!("AssetEvent<{}>::Removed",std::any::type_name::<T>())).field("handle", &handle.id).finish(),
        }
    }
}

#[derive(Debug)]
pub struct Assets<T: Asset> {
    assets: HashMap<HandleId, T>,
    pub events:Events<AssetEvent<T>>,
    ref_sender:Sender<RefEvent>
}


impl<T: Asset> Assets<T> {
    pub fn new(ref_sender:Sender<RefEvent>) -> Assets<T> {
        Assets {
            assets:Default::default(),
            events:Default::default(),
            ref_sender
        }
    }

    pub fn add(&mut self,asset:T) -> Handle<T> {
        let id = HandleId::random::<T>();
        self.assets.insert(id, asset);
        self.events.send(AssetEvent::Created {
            handle: Handle::weak(id),
        });
        self.create_handle(id)
    }

    pub fn get(&self, handle_id: &HandleId) -> Option<&T> {
        self.assets.get(handle_id)
    }


    fn create_handle(&self, id: HandleId) -> Handle<T> {
        Handle::strong(id, self.ref_sender.clone())
    }


    pub fn contains(&self, handle: HandleId) -> bool {
        self.assets.contains_key(&handle)
    }

    pub fn set_untracked(&mut self, handle_id: HandleId, asset: T) {
        if self.assets.insert(handle_id, asset).is_some() {
            self.events.send(AssetEvent::Modified {
                handle: Handle::weak(handle_id),
            });
        } else {
            self.events.send(AssetEvent::Created {
                handle: Handle::weak(handle_id),
            });
        }
    }

    pub fn remove(&mut self, handle_id: HandleId) -> Option<T> {
        let asset = self.assets.remove(&handle_id);
        if asset.is_some() {
            self.events.send(AssetEvent::Removed {
                handle: Handle::weak(handle_id),
            });
        }
        asset
    }

    pub fn clear(&mut self) {
        self.assets.clear()
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }

    pub fn asset_event_system( mut events: EventWriter<AssetEvent<T>>,mut assets: ResMut<Assets<T>>) {
        events.send_batch(assets.events.drain())
    }

    pub fn update_assets_system(server:Res<AssetServer>,mut assets:ResMut<Assets<T>>) {
        let life_events = server.lifecycle_events.read();
        let life_event = life_events.get(&T::TYPE_UUID).unwrap();
        loop {
            match life_event.receiver.try_recv() {
                Ok(LifecycleEvent::Create(_id)) => { },
                Ok(LifecycleEvent::Free(id)) => {
                    assets.remove(id);
                },
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => panic!("AssetChannel disconnected."),
            }
        }
    }
}