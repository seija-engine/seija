mod material_def;
mod material;
mod types;
pub mod errors;
pub use material::{Material};
pub use material_def::{MaterialDef,read_material_def};
use seija_app::App;
use seija_asset::AddAsset;
pub use types::RenderOrder;


pub(crate) fn init_material(app:&mut App) {
    app.add_asset::<MaterialDef>();
}