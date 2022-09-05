#[derive(PartialEq, Eq,Clone,Debug, Copy,Hash)]
pub struct OctantId(pub usize);

#[derive(Debug)]
pub struct Octree<T> {
    pub values:Vec<Octant<T>>,
    pub root:OctantId
}

impl<T> Octree<T> {
    pub fn new(root:T) -> Self {
        let values = vec![Octant::new(root)];
        let root = OctantId(0);
        Octree { values, root }
    }

    pub fn get(&self,id:OctantId) -> &Octant<T> {
        &self.values[id.0]
    }

    pub fn get_mut(&mut self,id:OctantId) -> &mut Octant<T> {
        &mut self.values[id.0]
    }
}

#[derive(Debug)]
pub struct Octant<T> {
   pub parent:Option<OctantId>,
   pub children:Vec<OctantId>,
   pub value:T
}

impl<T> Octant<T> {
    pub fn new(value:T) -> Self {
        Octant { parent: None, children: vec![], value }
    }

   
}