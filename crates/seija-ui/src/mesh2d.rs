use seija_core::math::{Vec3, Vec4, Vec2};
use seija_render::{resource::{Mesh, MeshAttributeType, Indices}, wgpu::PrimitiveTopology};

#[derive(Debug,Clone,Default)]
pub struct Mesh2D {
    pub color:Vec4,
    pub points:Vec<Vertex2D>,
    pub indexs:Vec<u32>
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex2D {
    pub pos:Vec3,
    pub uv:Vec2,
    pub uv1:Vec2
}


impl Into<Mesh> for Mesh2D {
    fn into(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut positons:Vec<[f32;3]> = vec![];
        let mut uvs:Vec<[f32;2]> = vec![];
        let mut uv1s:Vec<[f32;2]> = vec![];
        for vertex in self.points.iter() {
           positons.push([vertex.pos.x,vertex.pos.y,vertex.pos.z]);
           uvs.push([vertex.uv.x,vertex.uv.y]);
           uv1s.push([vertex.uv1.x,vertex.uv1.y]);
        }
        
        mesh.set(MeshAttributeType::POSITION, positons);
        mesh.set(MeshAttributeType::UV0, uvs);
        mesh.set(MeshAttributeType::UV1, uv1s);
       
        mesh.set_indices(Some(Indices::U32(self.indexs)));
        mesh.build();
        mesh
    }
}