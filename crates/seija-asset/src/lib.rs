use std::path::PathBuf;
use bevy_ecs::prelude::World;
use bevy_ecs::schedule::{StageLabel};
use bevy_ecs::schedule::SystemStage;
use loading_queue::AssetLoadingQueue;
use relative_path::RelativePath;
use seija_app::{App, IModule};
use seija_core::{AddCore, CoreStage};
mod server;
mod handle;
mod asset;
mod assets;
pub mod errors;
mod lifecycle;
mod loading_queue;
pub use asset::*;
pub use handle::{HandleId,HandleUntyped,Handle};
pub use assets::{Assets,AssetEvent};
pub use server::{AssetServer,AssetRequest,AssetInfo};
pub use lifecycle::{RefEvent,LifecycleEvent};
use seija_core::bevy_ecs::change_detection::Mut;
pub use downcast_rs;
pub use async_trait;
pub mod ffi;

#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel)]
pub enum AssetStage {
    LoadAssets,
    AssetEvents,
    AsyncLoader
}

pub struct AssetModule(pub PathBuf);

impl IModule for AssetModule {
    fn init(&mut self,app:&mut App) {
        app.add_resource(AssetLoadingQueue::default());
        app.add_resource(AssetServer::new(self.0.clone()));
        app.schedule.add_stage_before(CoreStage::PreUpdate, AssetStage::LoadAssets, SystemStage::parallel());
        app.schedule.add_stage_after(CoreStage::PostUpdate, AssetStage::AssetEvents, SystemStage::parallel());
        app.add_system(CoreStage::PreUpdate, server::free_unused_assets_system);
        app.add_system(CoreStage::PreUpdate, update_asset_system);
    }
}


pub trait AddAsset {
    fn add_asset<T>(&mut self)  where T: Asset;
    fn add_asset_loader<T:Asset,F:IAssetLoader + Default>(&mut self,);
}

impl AddAsset for App {
    fn add_asset<T>(&mut self) where T: Asset {
        let asset_server = self.world.get_resource::<AssetServer>().unwrap();
        let assets = asset_server.register_type::<T>();
        self.add_resource(assets);
        self.add_system(AssetStage::AssetEvents, Assets::<T>::asset_event_system);
        self.add_system(AssetStage::LoadAssets, Assets::<T>::update_assets_system);
        self.add_event::<AssetEvent<T>>();
    }

    fn add_asset_loader<T:Asset,F:IAssetLoader + Default>(&mut self) {
        let asset_server = self.world.get_resource::<AssetServer>().unwrap();
        asset_server.register_loader::<T,F>(Default::default());
    }
}


fn update_asset_system(world:&mut World) {
    world.resource_scope(|w:&mut World,mut loading_queue:Mut<AssetLoadingQueue>| {
       if let Some(server) = w.get_resource::<AssetServer>() {
          server.inner.life_cycle.free_unused_assets();
          let mut req_list = server.inner.request_list.write();
          if req_list.len() > 0 {
            let mut new_req_list:Vec<_> = req_list.drain(..).collect();
            drop(req_list);
            while let Some((uri,hid,params,loader)) = new_req_list.pop() {
                loading_queue.push_uri(uri,hid,loader,w,params);
             }
          }
          
       }
       loading_queue.update(w);
    })
}

pub fn this_asset_path(this_dir:&RelativePath,cur_path:&str) -> String {
    if cur_path.starts_with('/') {
        cur_path.trim_start_matches('/').into()
    } else {
        this_dir.join_normalized(cur_path).as_str().into()
    }
}