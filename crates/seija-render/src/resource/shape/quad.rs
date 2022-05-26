use crate::resource::{Mesh, MeshAttributeType, Indices};

use super::calc_tangent;

pub struct Quad {
    size:f32
}

impl Quad {
    pub fn new(size:f32) -> Self {
        Quad { size }
    }
}

impl From<Quad> for Mesh {
    fn from(quad: Quad) -> Self {
        let hs = quad.size / 2f32;
        let positions:Vec<[f32;3]> = vec![
            [-hs,-hs,0f32], //left bottom
            [-hs, hs,0f32], //left top
            [hs,hs,0f32], //right top
            [hs,-hs,0f32], //right bottom
        ];
        let uvs:Vec<[f32;2]> = vec![
            [0f32,1f32],[0f32,0f32],[1f32,0f32],[1f32,1f32],
        ];
        let normals:Vec<[f32;3]> = vec![
            [0f32,0f32,1f32],
            [0f32,0f32,1f32],
            [0f32,0f32,1f32],
            [0f32,0f32,1f32],
        ];
        let indices = vec![2,1,0,0,3,2];
        let tangent = calc_tangent(&positions, &uvs, &indices);
        let mut mesh = Mesh::new(wgpu::PrimitiveTopology::TriangleList);
        mesh.set(MeshAttributeType::POSITION,positions);
        mesh.set(MeshAttributeType::UV0, uvs);
        mesh.set(MeshAttributeType::NORMAL, normals);
        mesh.set(MeshAttributeType::TANGENT, tangent);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.build();
        mesh
    }
}