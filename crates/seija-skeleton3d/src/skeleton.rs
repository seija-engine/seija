use seija_transform::Transform;

pub struct Skeleton {
    pub joint_rest_poses:Vec<Transform>,
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
}