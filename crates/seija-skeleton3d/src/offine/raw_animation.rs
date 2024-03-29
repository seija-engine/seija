use glam::{Vec3, Quat};

#[derive(Default,Clone,Debug)]
#[repr(C)]
pub struct RawTranslationKey {
    pub time:f32,
    pub value:Vec3
}

impl RawTranslationKey {
    pub fn new(time:f32,value:Vec3) -> RawTranslationKey {
        RawTranslationKey { time, value }
    }
}


#[derive(Clone,Debug)]
#[repr(C)]
pub struct RawRotationKey {
    pub time:f32,
    pub value:Quat
}

impl RawRotationKey {
    pub fn new(time:f32,value:[f32;4]) -> RawRotationKey {
        RawRotationKey { time, value:Quat::from_array(value) }
    }
}

impl Default for RawRotationKey {
    fn default() -> Self {
        Self { time: 0f32, value: Quat::IDENTITY }
    }
}

#[derive(Clone,Debug)]
#[repr(C)]
pub struct RawScaleKey {
    pub time:f32,
    pub value:Vec3
}

impl RawScaleKey {
    pub fn new(time:f32,value:Vec3) -> RawScaleKey {
        RawScaleKey { time, value }
    }
}

impl Default for RawScaleKey {
    fn default() -> Self {
        Self { time: 0f32, value: Vec3::ONE }
    }
}

#[derive(Default)]
pub struct RawJointTrack {
    pub translations:Vec<RawTranslationKey>,
    pub rotations:Vec<RawRotationKey>,
    pub scales:Vec<RawScaleKey>
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