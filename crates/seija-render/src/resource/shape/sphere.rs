use hexasphere::shapes::IcoSphere;

use crate::resource::{Indices, Mesh,MeshAttributeType};

#[derive(Debug,Clone, Copy)]
pub struct Sphere {
    pub radius: f32,
    subdivisions: usize,
}

impl Sphere {
    pub fn new(r:f32) -> Self {
        Sphere {
            radius:r,
            subdivisions:5
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self { 
            radius: 1.0, 
            subdivisions:2 
        }
    }
}

impl From<Sphere> for Mesh {
    fn from(sphere: Sphere) -> Self {
        let generated = IcoSphere::new(sphere.subdivisions, |point| {
            let inclination = point.z.acos();
            let azumith = point.y.atan2(point.x);

            let norm_inclination = 1.0 - (inclination / std::f32::consts::PI);
            let norm_azumith = (azumith / std::f32::consts::PI) * 0.5;

            [norm_inclination, norm_azumith]
        });

        let raw_points = generated.raw_points();

        let points = raw_points.iter().map(|&p| (p * sphere.radius).into()).collect::<Vec<[f32; 3]>>();

        let normals = raw_points.iter().copied().map(Into::into).collect::<Vec<[f32; 3]>>();

        let uvs = generated.raw_data().to_owned();
        //println!("uvs :{:?}",&uvs);

        let mut indices = Vec::with_capacity(generated.indices_per_main_triangle() * 20);

        for i in 0..20 {
            generated.get_indices(i, &mut indices);
        }
        let indices = Indices::U32(indices);
        let mut mesh = Mesh::new(wgpu::PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        
        mesh.set(MeshAttributeType::POSITION, points);
        mesh.set(MeshAttributeType::NORMAL, normals);
        mesh.set(MeshAttributeType::UV0, uvs);
        mesh.build();

        mesh
    }
}