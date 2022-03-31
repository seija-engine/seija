use crate::resource::{Mesh, MeshAttributeType, Indices};

use super::calc_tangent;

pub struct Plane {
    pub size:f32,
    pub quad_count:usize,
}

impl Default for Plane {
    fn default() -> Self { Self { size: 1.0,quad_count:1 } }
}


impl Plane {
    pub fn new(size:f32,quad_count:usize) -> Self {
        Plane { size, quad_count }
    }
}

impl From<Plane> for Mesh {
    fn from(plane: Plane) -> Self {
        let all_pos_count = (plane.quad_count + 1) * (plane.quad_count + 1);
        let grid_size = plane.size as f32 / plane.quad_count as f32;
        let grid_uv = 1f32 / plane.size;
        let mut positions:Vec<[f32;3]> = Vec::with_capacity(all_pos_count);
        let mut uvs:Vec<[f32;2]> = Vec::with_capacity(all_pos_count);
        for y in 0..(plane.quad_count + 1) {
            for x in 0..(plane.quad_count + 1) {
                let pos:[f32;3] = [x as f32 * grid_size,0f32,y as f32 * grid_size];
                positions.push(pos);
                uvs.push([grid_uv * x as f32,grid_uv * y as f32]);
            }
        }
        let mut normals:Vec<[f32;3]> = Vec::with_capacity(all_pos_count);
        for _ in 0..all_pos_count {
            normals.push([0f32,1f32,0f32]);
        }
        let mut indices:Vec<u32> = Vec::with_capacity(plane.quad_count * plane.quad_count * 2);
        let pos_row_len = plane.quad_count + 1;     
        for y in 0..plane.quad_count {
            for x in 0..plane.quad_count {
                let y_offset:u32 = (y * pos_row_len) as u32;
                let y_offset2:u32 = ((y+1) * pos_row_len) as u32;
                let lt:u32 = y_offset + x as u32;
                let rt:u32 = y_offset + x as u32 + 1;
                let lb:u32 = y_offset2 + x as u32;
                let rb:u32 = y_offset2 + x as u32 + 1;
                indices.extend_from_slice(&[lb,rb,rt,lb,rt,lt]);
            }
        }
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