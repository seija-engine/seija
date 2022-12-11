use seija_core::math::{Vec3, Vec4, Vec2};

#[derive(Debug)]
pub struct Mesh2D {
    pub color:Vec4,
    pub points:Vec<Vertex2D>,
    pub indexs:Vec<u32>
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex2D {
    pub pos:Vec3,
    pub uv:Vec2
}
