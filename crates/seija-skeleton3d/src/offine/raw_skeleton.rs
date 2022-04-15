use seija_transform::{ TransformMatrix};

#[derive(Default,PartialEq)]
pub struct RawJoint {
    pub name:Option<String>,
    pub children:Vec<RawJoint>,
    pub transform:TransformMatrix
}

pub struct RawSkeleton {
   pub roots:Vec<RawJoint>
}


impl RawSkeleton {
    pub fn num_joints(&self) -> usize {
        let mut count = 0;
        RawSkeletonDFIter::new(self).run(|_,_| {count += 1; });
        count
    }   
}

pub struct RawSkeletonDFIter<'a> {
    raw_skeleton:&'a RawSkeleton
}

impl<'a> RawSkeletonDFIter<'a> {
    pub fn new(raw_skeleton:&'a RawSkeleton) -> Self {
        RawSkeletonDFIter { raw_skeleton }
    }
    pub fn run<F>(&self,mut f:F) where F:FnMut(&'a RawJoint,Option<&'a RawJoint>) {
        self._iter_depth_first(&self.raw_skeleton.roots, None, &mut f);
    }

    fn _iter_depth_first<F>(&self,children:&'a Vec<RawJoint>,parent:Option<&'a RawJoint>,f: &mut F) where F:FnMut(&'a RawJoint,Option<&'a RawJoint>) {
        for joint in children.iter() {
            f(joint,parent);
            self._iter_depth_first(&joint.children, Some(joint), f);
        }
    }
}


pub struct RawSkeletonBFIter<'a> {
    raw_skeleton:&'a RawSkeleton
}

impl<'a> RawSkeletonBFIter<'a> {
    pub fn new(raw_skeleton:&'a RawSkeleton) -> Self {
        RawSkeletonBFIter { raw_skeleton }
    }
    pub fn run<F>(&self,mut f:F) where F:FnMut(&'a RawJoint,Option<&'a RawJoint>) {
        self._iter_breadth_first(&self.raw_skeleton.roots, None, &mut f);
    }

    fn _iter_breadth_first<F>(&self,children:&'a Vec<RawJoint>,parent:Option<&'a RawJoint>,f:&mut F) where F:FnMut(&'a RawJoint,Option<&'a RawJoint>) {
        for joint in children.iter() {
            f(joint,parent);
        }
        for joint in children.iter() {
            self._iter_breadth_first(&joint.children, Some(joint), f);
        }
    }
}
