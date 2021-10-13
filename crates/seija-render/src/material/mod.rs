mod material_def;
mod material;
mod types;
mod storage;
mod system;
pub mod errors;
pub use material::{Material};
pub use storage::MaterialStorage;
pub use material_def::{MaterialDef,read_material_def,PassDef};
use seija_app::App;
use bevy_ecs::prelude::{IntoSystem};
use seija_asset::{AssetServer, AssetStage};
pub use types::{RenderOrder,Cull,ZTest};

use self::storage::material_storage_event;
pub use system::{MaterialSystem};


pub(crate) fn init_material(app:&mut App) {
    let server = app.world.get_resource::<AssetServer>().unwrap();
    server.register_type::<Material>();
    
    let storage = MaterialStorage::new(server.ref_counter.channel.sender.clone());
    app.add_resource(storage);
   
    app.add_system(AssetStage::AssetEvents, material_storage_event.system());
}