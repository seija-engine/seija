use bevy_ecs::prelude::World;
use seija_app::App;
use seija_core::CoreStage;
mod system;
mod camera_query;
mod view_list;
pub use self::system::QuerySystem;


pub fn init_system(app:&mut App) {
    app.add_resource(QuerySystem::default());
    app.add_system(CoreStage::PreUpdate ,camera_query::camera_query_check_add);
    app.add_system(CoreStage::Update ,camera_query::camera_query_update);
}