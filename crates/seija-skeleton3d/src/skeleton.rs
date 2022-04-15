use seija_transform::{TransformMatrix};

#[derive(Debug,Default)]
pub struct Skeleton {
    pub joint_rest_poses:Vec<TransformMatrix>,
    pub joint_parents:Vec<Option<usize>>,
    pub joint_names:Vec<Option<String>>
}

impl Skeleton {
    pub fn new(count:usize) -> Self {
        Skeleton { 
            joint_rest_poses: Vec::with_capacity(count),
            joint_parents: Vec::with_capacity(count),
            joint_names: Vec::with_capacity(count) 
        }
    }

    pub fn num_joints(&self) -> usize { self.joint_parents.len() }
}