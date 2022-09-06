mod sphere;
mod plane;
mod frustum;
use glam::{Mat4, Vec3};
pub use sphere::{Sphere};
pub use plane::{Plane};
pub use frustum::{Frustum};
pub mod volume;
mod traits;
pub mod bound;

pub use traits::Contains;


pub fn proj_view_corners(proj_view:&Mat4) -> [Vec3;8] {
    let inv = proj_view.inverse();
    let mut pos = [Vec3::new(-1f32, -1f32, -1f32),
                          Vec3::new(1f32, -1f32, -1f32),
                          Vec3::new(-1f32, 1f32, -1f32),
                          Vec3::new(1f32, 1f32, -1f32),
                          Vec3::new(-1f32, -1f32, 1f32),
                          Vec3::new(1f32, -1f32, 1f32),
                          Vec3::new(-1f32, 1f32, 1f32),
                          Vec3::new(1f32, 1f32, 1f32),];
    for p in pos.iter_mut() {
        *p = inv.project_point3(*p);
    }
    pos
}


pub fn calc_bound_sphere<'a>(points:[Vec3;8]) -> Sphere {
   
    let mut s = Vec3::ZERO;
    let mut count = 0;
    for p in points.iter() {
        s += *p;
        count = count + 1;
    }
    s = s * (1f32 / count as f32);
    let mut length = 0f32;
    for p in points.iter() {
        length = length.max((*p - s).length_squared());
    }

    length = length.sqrt();
    Sphere { center:s ,radius:length }
}
