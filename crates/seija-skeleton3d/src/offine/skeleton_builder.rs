use crate::skeleton::{Skeleton};
use super::raw_skeleton::{RawSkeleton, RawJoint, RawSkeletonDFIter};


pub struct SkeletonBuilder;

impl SkeletonBuilder {
    pub fn build(raw_skeleton:&RawSkeleton) -> Skeleton {
        let num_joints = raw_skeleton.num_joints();
        
        let mut linear_joints:Vec<(&RawJoint,Option<usize>)> = Vec::with_capacity(num_joints);
        
        let iter = RawSkeletonDFIter::new(raw_skeleton);
        iter.run(|current,parent| {
            let mut parent_index:Option<usize> = None;
            if let Some(parent) = parent {
                for idx in (0..linear_joints.len()).rev()  {
                    if linear_joints[idx].0 == parent {
                        parent_index = Some(idx);
                        break;
                    }
                }
            }
            linear_joints.push((current,parent_index));
        });

        let mut skeleton = Skeleton::new(num_joints);
        for (joint,parent) in linear_joints.iter() {
            skeleton.joint_names.push(joint.name.clone());
            skeleton.joint_parents.push(parent.clone());
            skeleton.joint_rest_poses.push(joint.transform.clone());
        }
        skeleton
    }
}

