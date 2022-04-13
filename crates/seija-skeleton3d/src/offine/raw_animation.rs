use glam::{Vec3, Quat};

#[derive(Default)]
pub struct RawTranslationKey {
    pub time:f32,
    pub value:Vec3
}

#[derive(Default)]
pub struct RawRotationKey {
    pub time:f32,
    pub value:Quat
}

#[derive(Default)]
pub struct RawScaleKey {
    pub time:f32,
    pub value:Vec3
}

#[derive(Default)]
pub struct RawJointTrack {
    translations:RawTranslationKey,
    rotations:RawRotationKey,
    scales:RawScaleKey
}


#[derive(Default)]
pub struct RawAnimation {
    pub name:String,
    pub duration:f32,
    pub tracks:Vec<RawJointTrack>,
}

impl RawAnimation {
    pub fn num_tracks(&self) -> usize {
        self.tracks.len()
    }
}