mod material_def;
mod material;
mod types;
mod center;
pub mod errors;
pub use material::{Material};
pub use center::MaterialDefCenter;
pub use material_def::{MaterialDef,read_material_def};
use seija_app::App;
use seija_core::{AddCore};
use bevy_ecs::prelude::{IntoSystem};
use seija_asset::{AssetEvent, AssetServer, AssetStage};
pub use types::RenderOrder;

use self::center::material_center_event;



pub(crate) fn init_material(app:&mut App) {
    let server = app.world.get_resource::<AssetServer>().unwrap();
    let material_def_center = MaterialDefCenter::new(server.ref_counter.channel.sender.clone());
    app.add_resource(material_def_center);
    app.add_system(AssetStage::AssetEvents, material_center_event.system());
    app.add_event::<AssetEvent<MaterialDef>>();
    
}