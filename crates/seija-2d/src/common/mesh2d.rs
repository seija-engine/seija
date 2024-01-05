use seija_core::math::{Vec3, Vec4, Vec2};
use seija_render::resource::{Mesh, MeshAttributeType, Indices};
use wgpu::PrimitiveTopology;

#[derive(Debug,Clone,Default)]
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


impl Into<Mesh> for Mesh2D {
    fn into(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut positons:Vec<[f32;3]> = vec![];
        let mut uvs:Vec<[f32;2]> = vec![];
        let mut colors:Vec<[f32;4]> = vec![];
        for vertex in self.points.iter() {
           positons.push([vertex.pos.x,vertex.pos.y,vertex.pos.z]);
           uvs.push([vertex.uv.x,vertex.uv.y]);
           colors.push(self.color.into());
        }
        
        mesh.set(MeshAttributeType::POSITION, positons);
        mesh.set(MeshAttributeType::UV0, uvs);
        mesh.set(MeshAttributeType::COLOR,colors);
       
        mesh.set_indices(Some(Indices::U32(self.indexs)));
        mesh.build();
        mesh
    }
}