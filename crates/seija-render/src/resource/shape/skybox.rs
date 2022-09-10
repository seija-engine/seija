use glam::Vec3;

use crate::resource::{Indices, Mesh,MeshAttributeType};

pub struct SkyBox;

impl From<SkyBox> for Mesh {
    fn from(_: SkyBox) -> Mesh {
        let min = Vec3::new(-1f32, -1f32, -1f32);
        let max = Vec3::new(1f32, 1f32, 1f32);
        let vertices = &[
            //front
            ([min.x, max.y, min.z], [0., 0.]), 
            ([max.x, max.y, min.z], [1., 0.]),
            ([min.x, min.y, min.z], [0., 1.]), 
            ([max.x, min.y, min.z], [1., 1.]), 
            //right
            ([max.x, max.y, min.z], [0., 0.]), 
            ([max.x, max.y, max.z], [1., 0.]),
            ([max.x, min.y, min.z], [0., 1.]), 
            ([max.x, min.y, max.z], [1., 1.]),
            //left
            ([min.x, max.y, max.z], [0., 0.]), 
            ([min.x, max.y, min.z], [1., 0.]),
            ([min.x, min.y, max.z], [0., 1.]), 
            ([min.x, min.y, min.z], [1., 1.]), 
            //back
            ([max.x, max.y, max.z], [0., 0.]), 
            ([min.x, max.y, max.z], [1., 0.]),
            ([max.x, min.y, max.z], [0., 1.]), 
            ([min.x, min.y, max.z], [1., 1.]), 
            //bottom
            ([min.x, min.y, min.z], [0., 0.]), 
            ([max.x, min.y, min.z], [1., 0.]),
            ([min.x, min.y, max.z], [0., 1.]), 
            ([max.x, min.y, max.z], [1., 1.]),
            //top
            ([min.x, max.y, min.z], [0., 0.]), 
            ([max.x, max.y, min.z], [1., 0.]),
            ([min.x, max.y, max.z], [0., 1.]), 
            ([max.x, max.y, max.z], [1., 1.]), 
        ];

        let mut positions = Vec::with_capacity(24);
        let mut uvs = Vec::with_capacity(24);
        for (position,uv) in vertices.iter() {
            positions.push(*position);
            uvs.push(*uv);
        }

        let idx_u32s = vec![
            0,3,2, 0,1,3,
            4,7,6, 4,5,7,
            8,11,10, 8,9,11,
            12,15,14, 12,13,15,
            16,19,18, 16,17,19,
            20,23,22, 20,21,23
        ];
        let indices = Indices::U32(idx_u32s);
        let mut mesh = Mesh::new(wgpu::PrimitiveTopology::TriangleList);
        mesh.set(MeshAttributeType::POSITION,positions);
        mesh.set(MeshAttributeType::UV0, uvs);
        mesh.set_indices(Some(indices));
        mesh.build();
        mesh
    }
}