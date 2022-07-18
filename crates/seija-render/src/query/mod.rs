use seija_app::App;
use seija_core::CoreStage;
mod system;
mod camera_query;
mod shadow_query;
mod view_list;
pub use system::{QuerySystem,ViewQuery,IdOrName};


pub fn init_system(app:&mut App) {
    app.add_resource(QuerySystem::default());
    app.add_system(CoreStage::PreUpdate ,camera_query::camera_query_check_add);
    app.add_system(CoreStage::Update ,camera_query::camera_query_update);
    app.add_system(CoreStage::Update, shadow_query::shadow_query_update);
}