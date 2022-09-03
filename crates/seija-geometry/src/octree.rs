#[derive(PartialEq, Eq,Clone,Debug, Copy,Hash)]
pub struct OctantId(pub usize);

#[derive(Debug)]
pub struct Octree<T> {
    values:Vec<Octant<T>>,
    root:OctantId
}

#[derive(Debug)]
pub struct Octant<T> {
    parent:Option<OctantId>,
    children:Vec<OctantId>,
    value:T
}

