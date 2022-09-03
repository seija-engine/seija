use crate::{Plane, bound::{PlaneBound, Relation}};

pub struct Frustum {
    pub left: Plane,
    /// Right plane
    pub right: Plane,
    /// Bottom plane
    pub bottom: Plane,
    /// Top plane
    pub top: Plane,
    /// Near plane
    pub near: Plane,
    /// Far plane
    pub far: Plane,
}

impl Frustum {
    pub fn contains<B: PlaneBound>(&self, bound: &B) -> Relation {
        [
            &self.left,
            &self.right,
            &self.top,
            &self.bottom,
            &self.near,
            &self.far,
        ]
        .iter()
        .fold(Relation::In, |cur, p| {
            use std::cmp::max;
            let r = bound.relate_plane(p);
            max(cur, r)
        })
}
}