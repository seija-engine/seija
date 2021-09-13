use std::{collections::HashMap, fmt::Debug};

use bevy_ecs::prelude::ResMut;
use seija_core::event::{EventWriter, Events};
use crate::{asset::Asset, handle::{Handle, HandleId}};

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
    events:Events<AssetEvent<T>>
}


impl<T: Asset> Assets<T> {
    pub fn new() -> Assets<T> {
        Assets {
            assets:Default::default(),
            events:Default::default(),
        }
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
}