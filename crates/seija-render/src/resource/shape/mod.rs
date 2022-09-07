mod skybox;
mod sphere;
mod cube;
mod plane;
mod quad;

use glam::{Vec2, Vec3};
use seija_geometry::volume::AABB3;
pub use skybox::SkyBox;
pub use sphere::Sphere;
pub use cube::{Box,Cube};
pub use plane::{Plane};
pub use quad::{Quad};

use super::{Mesh, Indices, MeshAttributeType};


pub fn calc_tangent(verts:&Vec<[f32;3]>,uvs:&Vec<[f32;2]>,indices:&Vec<u32>) -> Vec<[f32;4]> {
    let tris = indices.len() / 3;
    let mut tangents:Vec<Vec3> = vec![Vec3::ZERO;verts.len()];
    for i in 0..tris {
        let index_0 = indices[i * 3] as usize;
        let index_1 = indices[i * 3 + 1] as usize;
        let index_2 = indices[i * 3 + 2] as usize;
        let pos1 = Vec3::from_slice(&verts[index_0]);
        let pos2 = Vec3::from_slice(&verts[index_1]);
        let pos3 = Vec3::from_slice(&verts[index_2]);
        let uv1 = Vec2::from_slice(&uvs[index_0]);
        let uv2 = Vec2::from_slice(&uvs[index_1]);
        let uv3 = Vec2::from_slice(&uvs[index_2]);

        let edge1:Vec3 = pos2 - pos1;
        let edge2:Vec3 = pos3 - pos1;
        let delta_uv1 = uv2 - uv1;
        let delta_uv2 = uv3 - uv1;

        let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);
        let x = f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x);
        let y = f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y);
        let z = f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z);
        
        let tangent = Vec3::new(x, y, z);

        tangents[index_0] += tangent;
        tangents[index_1] += tangent;
        tangents[index_2] += tangent;
    }

    let mut ret_list:Vec<[f32;4]> = Vec::with_capacity(tangents.len());
    for tangent in tangents.iter_mut() {
        let n = tangent.normalize();
        ret_list.push([n.x,n.y,n.z,1f32]);
    }
    ret_list
}

pub fn create_aabb_mesh(aabb:&AABB3) -> Mesh {
    let min = aabb.min;
    let max = aabb.max;

    let mut mesh = Mesh::new(wgpu::PrimitiveTopology::LineList);
    let positions:Vec<[f32;3]> = vec![
         [min.x,max.y,min.z],
         [max.x,max.y,min.z],
         [min.x,max.y,max.z],
         [max.x,max.y,max.z],

         [min.x,min.y,min.z],
         [max.x,min.y,min.z],
         [min.x,min.y,max.z],
         [max.x,min.y,max.z],
    ];

    let idx_u32s:Vec<u32> = vec![
        0,2, 0,1, 1,3, 2,3,
        4,6, 4,5, 5,7, 6,7,
        0,4, 1,5, 2,6, 3,7
    ];
    let indices = Indices::U32(idx_u32s);
    mesh.set(MeshAttributeType::POSITION,positions);
    mesh.set_indices(Some(indices));
    mesh.aabb = Some(aabb.clone());
    mesh.build();
    mesh
}