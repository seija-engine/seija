use glam::Vec4;

#[derive(Debug)]
pub struct AmbientLight {
    pub color: Vec4,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self { color: Vec4::new(0.3f32, 0.3f32, 0.3f32, 1f32) }
    }
}