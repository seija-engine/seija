use std::ops::Mul;

use glam::{Vec3, Mat4, Vec4, Vec4Swizzles};

use crate::{traits::Contains, bound::{PlaneBound, Relation}};

use super::aabb::IAABB;
#[derive(Debug,Clone)]
pub struct AABB3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB3 {
    pub fn new(p1: Vec3, p2: Vec3) -> Self {
        AABB3 {
            min: Vec3::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z)),
            max: Vec3::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z)),
        }
    }

    #[inline]
    pub fn to_corners(&self) -> [Vec3; 8] {
        [
            self.min,
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            self.max,
        ]
    }

    #[inline]
    pub fn to_corners_v4(&self) -> [Vec4; 8] {
        [
            Vec4::new(self.min.x, self.min.y, self.min.z, 1f32),
            Vec4::new(self.max.x, self.min.y, self.min.z,1f32),
            Vec4::new(self.min.x, self.max.y, self.min.z,1f32),
            Vec4::new(self.max.x, self.max.y, self.min.z,1f32),
            Vec4::new(self.min.x, self.min.y, self.max.z,1f32),
            Vec4::new(self.max.x, self.min.y, self.max.z,1f32),
            Vec4::new(self.min.x, self.max.y, self.max.z,1f32),
            Vec4::new(self.max.x, self.max.y, self.max.z, 1f32),
        ]
    }

    pub fn add_margin(&self, margin: Vec3) -> Self {
        AABB3::new(
            Vec3::new(
                self.min.x - margin.x,
                self.min.y - margin.y,
                self.min.z - margin.z,
            ),
            Vec3::new(
                self.max.x + margin.x,
                self.max.y + margin.y,
                self.max.z + margin.z,
            ),
        )
    }

    pub fn transform(&self,mat4:&Mat4) -> Self {
        let corners = self.to_corners_v4();
        let mut min = mat4.mul(corners[0]);
        let mut max = min;
        for p in corners.into_iter().skip(1) {
           let np = mat4.mul(p);
           min = min.min(np);
           max = max.max(np);
        }
        AABB3::new(min.xyz(), max.xyz())
    }

    pub fn grow(&self,p:Vec3) -> Self {
        let min = self.min.min(p);
        let max = self.max.max(p);
        AABB3::new(min, max)
    }

}

impl IAABB for AABB3 {
    fn min(&self) -> Vec3 {
        self.min
    }

    fn max(&self) -> Vec3 {
        self.max
    }
}

impl Contains<Vec3> for AABB3 {
    fn contains(&self, p: &Vec3) -> bool {
        self.min.x <= p.x
            && p.x < self.max.x
            && self.min.y <= p.y
            && p.y < self.max.y
            && self.min.z <= p.z
            && p.z < self.max.z
    }
}

impl Contains<AABB3> for AABB3 {
    fn contains(&self, other: &AABB3) -> bool {
        let other_min = other.min;
        let other_max = other.max;

        other_min.x >= self.min.x
            && other_min.y >= self.min.y
            && other_min.z >= self.min.z
            && other_max.x <= self.max.x
            && other_max.y <= self.max.y
            && other_max.z <= self.max.z
    }
}

impl PlaneBound for AABB3 {
    fn relate_plane(&self, plane: &crate::Plane) -> Relation {
        let corners = self.to_corners();
        let first = corners[0].relate_plane(plane);
        for p in corners[1..].iter() {
            if p.relate_plane(plane) != first {
                return Relation::Cross;
            }
        }
        first
    }
}
