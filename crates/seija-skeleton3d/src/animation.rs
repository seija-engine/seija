use glam::{Vec3, Quat};

#[derive(Default)]
pub struct Animation {
   pub(crate) name:String,
   pub(crate) duration:f32,
   pub(crate) num_tracks:usize,
   translations_:Vec<Vec3>,
   rotations_:Vec<Quat>,
   scales_:Vec<Vec3>
}