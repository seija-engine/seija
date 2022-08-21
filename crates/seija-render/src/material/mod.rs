mod material_def;
mod material;
mod types;
mod storage;
mod system2;
mod system;
mod texture_prop_def;
pub mod loader;
pub mod errors;
pub use material::{Material};
pub use material_def::{MaterialDef,read_material_def,PassDef,ShaderInfoDef,MaterialDefineAsset};
use seija_app::App;
use seija_asset::{AddAsset, AssetServer, AssetStage};
pub use types::{RenderOrder,Cull,ZTest,RenderPath,STextureDescriptor};
pub use texture_prop_def::{TexturePropDef,TexturePropInfo};
pub use system::{MaterialSystem};


pub use storage::MaterialStorage;

use self::storage::material_storage_event;


pub(crate) fn init_material(app:&mut App) {
    let server = app.world.get_resource::<AssetServer>().unwrap();
    server.register_type::<Material>();
    let mut storage = MaterialStorage::new(server.get_ref_sender());
    storage.init(&mut app.world);
    app.add_resource(storage);
    
    app.add_system(AssetStage::AssetEvents, material_storage_event);

    app.add_asset::<MaterialDefineAsset>();
    app.add_asset::<Material>();
}