use glam::Vec3;

use crate::Plane;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum Relation {
    In,
    Cross,
    Out,
}

pub trait PlaneBound {
    fn relate_plane(&self, plane: &Plane) -> Relation;
}

impl PlaneBound for Vec3 {
    fn relate_plane(&self, plane: &Plane) -> Relation {
        let dist = self.dot(plane.normal);
        if dist > plane.distance {
            Relation::In
        } else if dist < plane.distance {
            Relation::Out
        } else {
            Relation::Cross
        }
    }
}