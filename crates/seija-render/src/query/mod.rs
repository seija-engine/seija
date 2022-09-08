use seija_app::App;
use seija_core::{CoreStage, StartupStage};
mod system;
mod camera_query;
mod shadow_query;
mod view_list;
mod scene_octree;
mod scene_octree_mgr;

pub use system::{QuerySystem,ViewQuery,IdOrName};

use crate::RenderStage;


pub fn init_system(app:&mut App) {
   
    
    app.add_system(RenderStage::PostRender ,camera_query::camera_query_check_add);
    app.add_system(CoreStage::Update ,camera_query::camera_query_update);
    app.add_system(CoreStage::Update, shadow_query::shadow_query_update);
    app.add_system(CoreStage::Update, camera_query::update_camera_octree_query);
    app.add_system2(CoreStage::Startup, StartupStage::PostStartup,scene_octree_mgr::on_post_startup);
    app.add_system(CoreStage::Last ,scene_octree_mgr::on_last_update);
}