use glam::Mat4;
use seija_transform::TransformMatrix;

use crate::Skeleton;

pub struct  LocalToModelJob<'a> {
    skeleton:&'a Skeleton,
    pub root:Option<&'a Mat4>
}

impl<'a> LocalToModelJob<'a> {
    pub fn new(skeleton:&'a Skeleton) -> Self {
        LocalToModelJob { skeleton, root: None }
    }

    pub fn run(&self,input:&Vec<TransformMatrix>,output:&mut Vec<Mat4>) {
        let parents = &self.skeleton.joint_parents;
        let ident = Mat4::IDENTITY;
        let root_mat4 = if let Some(root) = self.root { root } else { &ident };
        for idx in 0..parents.len() {
            let local = input[idx].matrix();
            if let Some(parent_index) =  parents[idx] {
                output[idx] = output[parent_index] * local;
            } else {
                output[idx] = local.mul_mat4(root_mat4);
            }
        }
    }
}