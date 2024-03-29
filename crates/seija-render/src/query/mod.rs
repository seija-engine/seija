use seija_app::App;
use seija_core::{CoreStage};
mod system;
mod camera_query;
mod shadow_query;
mod view_list;
mod scene_octree;
mod scene_octree_mgr;
pub use scene_octree_mgr::{SceneOctreeModule};
pub use system::{QuerySystem,ViewQuery,IdOrName};

use crate::RenderStage;

pub fn init_system(app:&mut App) {    
    app.add_system(CoreStage::PostUpdate ,camera_query::camera_query_check_add);
    app.add_system(RenderStage::PreRender ,camera_query::camera_query_update);
    app.add_system(RenderStage::PreRender, shadow_query::shadow_query_update);
}