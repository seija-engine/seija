use crate::resource::{Indices, Mesh,MeshAttributeType};

pub struct SkyBox;

impl From<SkyBox> for Mesh {
    fn from(_: SkyBox) -> Mesh {
        let positions:Vec<[f32;3]> = vec![
            [-1.0f32,1.0f32,-1.0f32],
            [-1.0f32,1.0f32, 1.0f32],
            [1.0f32,1.0f32, 1.0f32],
            [1.0f32,1.0f32,-1.0f32],

            [-1.0f32,-1.0f32,-1.0f32],
            [-1.0f32,-1.0f32, 1.0f32],
            [ 1.0f32,-1.0f32, 1.0f32],
            [ 1.0f32,-1.0f32,-1.0f32]
        ];

        let indices = Indices::U32(vec![
            //+x
		    3,6,7,3,2,6,
            //-x
		    0,5,1,0,4,5,
            //+y
		    0,1,2,0,2,3,
           //-y
		    4,6,5,4,7,6,
            //+z
		    1,5,6,1,6,2,
            //-z
		    0,7,4,0,3,7
        ]);
        let mut mesh = Mesh::new(wgpu::PrimitiveTopology::TriangleList);
        mesh.set(MeshAttributeType::POSITION,positions);
        mesh.set_indices(Some(indices));
        mesh.build();
        mesh
    }
}