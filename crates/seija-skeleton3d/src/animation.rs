use glam::{Vec3, Quat};

#[derive(Default,Debug)]
pub struct Float3Key {
   pub ratio:f32,
   pub track:usize,
   pub value:Vec3
}

#[derive(Default,Debug)]
pub struct QuaternionKey {
  pub ratio:f32,
  pub track:usize,
  pub value:Quat
}

#[derive(Default,Debug)]
pub struct Animation {
   pub(crate) name:String,
   pub(crate) duration:f32,
   pub(crate) num_tracks:usize,
   pub(crate) translations_:Vec<Float3Key>,
   pub(crate) rotations_:Vec<QuaternionKey>,
   pub(crate) scales_:Vec<Float3Key>
}

impl Animation {
   pub fn name(&self) -> &str {
      self.name.as_str()
   }
}