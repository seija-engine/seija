use glam::Mat4;
use seija_core::{TypeUuid,uuid::{Uuid}};
use seija_transform::TransformMatrix;

#[derive(Debug,TypeUuid)]
#[uuid = "dbc54cbc-ec55-453d-bda8-80af390d35ea"]
pub struct RuntimeSkeleton {
   pub values:Vec<TransformMatrix>,
   pub mat4s:Vec<Mat4>
}

impl RuntimeSkeleton {
   pub fn new(count:usize) -> Self {
      let values = vec![TransformMatrix::default() ;count];
      let mat4s = vec![Mat4::IDENTITY;count];
      RuntimeSkeleton { values, mat4s }
   }
}