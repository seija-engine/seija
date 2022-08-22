mod material_def;
mod material;
mod types;
mod system;
mod texture_prop_def;
pub mod loader;
pub mod errors;
pub use material::{Material};
pub use material_def::{MaterialDef,read_material_def,PassDef,ShaderInfoDef,MaterialDefineAsset};
use seija_app::App;
use seija_asset::{AddAsset};
pub use types::{RenderOrder,Cull,ZTest,RenderPath,STextureDescriptor};
pub use texture_prop_def::{TexturePropDef,TexturePropInfo};
pub use system::{MaterialSystem};
use seija_core::TypeUuid;

use self::loader::{MaterialDefineAssetLoader, MaterialLoader};




pub(crate) fn init_material(app:&mut App) {

    app.add_asset::<MaterialDefineAsset>();
    app.add_asset::<Material>();
    app.add_asset_loader(MaterialDefineAsset::TYPE_UUID, MaterialDefineAssetLoader);
    app.add_asset_loader(Material::TYPE_UUID, MaterialLoader);
}