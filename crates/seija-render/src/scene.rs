use glam::Vec3;
use seija_geometry::volume::AABB3;

pub struct SceneEnv {
    pub aabb:AABB3
}

impl Default for SceneEnv {
    fn default() -> Self {
        let p = Vec3::new(1000f32,1000f32 , 1000f32);
        SceneEnv { aabb: AABB3::new(-p,p) }
    }
}
