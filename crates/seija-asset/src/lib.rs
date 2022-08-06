use bevy_ecs::schedule::{StageLabel};
use bevy_ecs::schedule::SystemStage;
use seija_app::{App, IModule};
use seija_core::{AddCore, CoreStage};
mod server;
mod handle;
mod asset;
mod assets;
pub use asset::{Asset};
pub use handle::{HandleId,HandleUntyped,Handle};
pub use assets::{Assets,AssetEvent};
pub use server::{AssetServer, RefEvent,LifecycleEvent};

#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel)]
pub enum AssetStage {
    LoadAssets,
    AssetEvents,
}

pub struct AssetModule;

impl IModule for AssetModule {
    fn init(&mut self,app:&mut seija_app::App) {
        app.add_resource(AssetServer::new());
        app.schedule.add_stage_before(CoreStage::PreUpdate, AssetStage::LoadAssets, SystemStage::parallel());
        app.schedule.add_stage_after(CoreStage::PostUpdate, AssetStage::AssetEvents, SystemStage::parallel());
        app.add_system(CoreStage::PreUpdate, server::free_unused_assets_system);
    }
}


pub trait AddAsset {
    fn add_asset<T>(&mut self)  where T: Asset;
}

impl AddAsset for App {
    fn add_asset<T>(&mut self) where T: Asset {
        let asset_server = self.world.get_resource::<AssetServer>().unwrap();
        let assets = asset_server.register_type::<T>();
      
        self.add_resource(assets);
        self.add_system(AssetStage::AssetEvents, Assets::<T>::asset_event_system);
        self.add_system(AssetStage::LoadAssets, Assets::<T>::update_assets_system);
        //self.add_event::<AssetEvent<T>>();
        self.add_event::<AssetEvent<T>>();
    }
}