use glam::Mat4;

use crate::{
    bound::{PlaneBound, Relation},
    Plane, proj_view_corners, volume::AABB3,
};

#[derive(Debug)]
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
    pub fn new(left:Plane,right:Plane,bottom:Plane,top:Plane,near:Plane,far:Plane) -> Self {
        Frustum { left, right, bottom, top, near, far }
    }

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

    pub fn from_matrix4(mat4:&Mat4) -> Option<Self> {
        let left = Plane::from_vec4_alt(mat4.row(3)  + mat4.row(0)).normalize()?;
        let right = Plane::from_vec4_alt(mat4.row(3) - mat4.row(0)).normalize()?;
        let bottom = Plane::from_vec4_alt(mat4.row(3) + mat4.row(1)).normalize()?;
        let top = Plane::from_vec4_alt(mat4.row(3) - mat4.row(1)).normalize()?;
        let near = Plane::from_vec4_alt(mat4.row(3) + mat4.row(2)).normalize()?;
        let far = Plane::from_vec4_alt(mat4.row(3) - mat4.row(2)).normalize()?;

       
        Some(Frustum::new(left, right, bottom, top, near, far))
    }

    pub fn create_aabb(mat4:&Mat4) -> AABB3 {
        let corners = proj_view_corners(mat4);
        let mut min = corners[0];
        let mut max = corners[0];
        for v in corners.iter().skip(1) {
            min = min.min(*v);
            max = max.max(*v);
        }
        AABB3::new(min, max)
    }
}
