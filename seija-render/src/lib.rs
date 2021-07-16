mod material;
use bevy_app::{AppBuilder,Plugin};
use bevy_asset::AddAsset;
pub use material::material::Material;
pub use material::material::{MaterialDesc,MaterialProp};
pub use material::asset::MaterialDescLoader;
pub use material::table::MaterialDescTable;

#[derive(Default)]
pub struct SeijaRenderPlugin;

impl Plugin for SeijaRenderPlugin {

    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<MaterialDesc>();
        app.add_asset::<Material>();
        app.add_asset_loader(MaterialDescLoader);
        app.insert_resource(MaterialDescTable::default());
    }

}