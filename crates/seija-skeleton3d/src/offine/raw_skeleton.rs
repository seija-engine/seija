use seija_transform::Transform;

#[derive(Default)]
pub struct RawJoint {
    pub name:Option<String>,
    pub children:Vec<RawJoint>,
    pub transform:Transform
}

pub struct RawSkeleton {
   pub roots:Vec<RawJoint>
}


impl RawSkeleton {
    
    pub fn iter_depth_first<F>(&self,mut f: F) where F:FnMut(&RawJoint,Option<&RawJoint>) {
        self._iter_depth_first(&self.roots, None, &mut  f);
    }

    fn _iter_depth_first<F>(&self,children:&Vec<RawJoint>,parent:Option<&RawJoint>,f:&mut F) where F:FnMut(&RawJoint,Option<&RawJoint>) {
        for joint in children.iter() {
            f(joint,parent);
            self._iter_depth_first(&joint.children, Some(joint), f);
        }
    }

    pub fn iter_breadth_first<F>(&self,mut f: F) where F:FnMut(&RawJoint,Option<&RawJoint>) {
        self._iter_breadth_first(&self.roots, None, &mut  f);
    }

    fn _iter_breadth_first<F>(&self,children:&Vec<RawJoint>,parent:Option<&RawJoint>,f:&mut F) where F:FnMut(&RawJoint,Option<&RawJoint>) {
        for joint in children.iter() {
            f(joint,parent);
        }
        for joint in children.iter() {
            self._iter_breadth_first(&joint.children, Some(joint), f);
        }
    }

    pub fn num_joints(&self) -> usize {
        let mut count = 0;
        self.iter_depth_first(|_,_| {count += 1} );
        count
    }
    
}