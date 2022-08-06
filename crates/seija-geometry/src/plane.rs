use glam::{Vec3};

pub struct Plane {
   pub normal:Vec3,
   pub distance:f32
}

impl Plane {
    pub fn new(normal:Vec3,distance:f32) -> Self {
        Plane { normal, distance }
    }

    pub fn from_ps(a:Vec3,b:Vec3,c:Vec3) -> Option<Self> {
        let v0 = b - a;
        let v1 = c - a;
        let n = v0.cross(v1);
        if n.abs_diff_eq(Vec3::ZERO, f32::EPSILON) {
            None
        } else {
            let n = n.normalize();
            let d = -a.dot(n);
            Some(Plane::new(n, d))
        }
    }

    pub fn normalize(&self) -> Option<Plane> {
        
        if self.normal.abs_diff_eq(Vec3::ZERO, f32::EPSILON) {
            None
        } else {
            let denom = 1f32 / self.normal.length();
            Some(Plane::new(self.normal * denom, self.distance * denom))
        }
    }
}