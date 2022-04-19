use crate::{Animation, Float3Key};
use glam::{Quat, Vec3};
use seija_transform::TransformMatrix;

pub struct SamplingJob {
    pub last_ratio: f32,
    cache_tracks: Vec<TrackKeys>,
    track_cur_indexs: Vec<TrackKeyIndex>,
}

#[derive(Default, Debug)]
struct TrackKeys {
    translations: Vec<usize>,
    rotations: Vec<usize>,
    scales: Vec<usize>,
}

#[derive(Default,Debug, Clone)]
struct TrackKeyIndex {
    trans_index: usize,
    rotation_index: usize,
    scale_index: usize,
}

impl SamplingJob {
    fn create(anim: &Animation) -> Self {
        let mut job = SamplingJob {
            last_ratio: 0f32,
            cache_tracks: vec![],
            track_cur_indexs: vec![],
        };
        job.init(anim);
        job
    }

    fn init(&mut self, anim: &Animation) {
        for _ in 0..anim.num_tracks {
            let track_keys = TrackKeys::default();
            self.cache_tracks.push(track_keys);
        }

        for (idx, tk) in anim.translations_.iter().enumerate() {
            self.cache_tracks[tk.track].translations.push(idx);
        }
        for (idx, tk) in anim.scales_.iter().enumerate() {
            self.cache_tracks[tk.track].scales.push(idx);
        }
        for (idx, tk) in anim.rotations_.iter().enumerate() {
            self.cache_tracks[tk.track].rotations.push(idx);
        }
        self.track_cur_indexs = vec![TrackKeyIndex::default(); anim.num_tracks];
    }

    pub fn run(&mut self, animation: &Animation, output: &mut Vec<TransformMatrix>, ratio: f32) {
        if animation.num_tracks == 0 {
            return;
        }
        let anim_ratio = ratio.clamp(0f32, 1f32);
        self.search_cur_keys(animation, anim_ratio);
       
        for (track_index,key_index) in self.track_cur_indexs.iter().enumerate() {
            let cur_cache_track = &self.cache_tracks[track_index];
            let cur_t_index = key_index.trans_index;
            if cur_cache_track.translations.len() >= cur_t_index + 1 {
                let cur_key_index = cur_cache_track.translations[cur_t_index];
                let next_key_index = cur_cache_track.translations[cur_t_index + 1];
                let cur_key = &animation.translations_[cur_key_index];
                let next_key = &animation.translations_[next_key_index];
                
                let mut interp_t_ratio = (ratio - cur_key.ratio) * (1f32 / (next_key.ratio - cur_key.ratio));
                let lerp_postion = cur_key.value.lerp(next_key.value, interp_t_ratio);
                println!("{} {} {} = {}",&cur_key.value,&next_key.value,interp_t_ratio,&lerp_postion);

            }
        }

       
      
        self.last_ratio = ratio;
    }

    fn search_cur_keys(&mut self, anim: &Animation, ratio: f32) {
        for (track_index, track) in self.cache_tracks.iter().enumerate() {
            let mut last_index = 0;
            for (index,at_index) in track.translations.iter().enumerate() {
                let curkey = &anim.translations_[*at_index];
                if curkey.ratio >= ratio {
                    self.track_cur_indexs[track_index].trans_index = last_index;
                    break;
                }
                last_index = index;
            }
            last_index = 0;
            for  (index,at_index) in track.rotations.iter().enumerate() {
                let curkey = &anim.rotations_[*at_index];
                if curkey.ratio >= ratio {
                    self.track_cur_indexs[track_index].rotation_index = last_index;
                    break;
                }
                last_index = index;
            }
            last_index = 0;
            for (index,at_index) in track.rotations.iter().enumerate() {
                let curkey = &anim.scales_[*at_index];
                if curkey.ratio >= ratio {
                    self.track_cur_indexs[track_index].scale_index = last_index;
                    break;
                }
                last_index = index;
            }
        }
    }
}

#[test]
fn aaa() {
    use crate::offine::animation_builder::AnimationBuilder;
    use crate::offine::raw_animation::{
        RawAnimation, RawJointTrack, RawRotationKey, RawTranslationKey,
    };
    let mut raw_anim = RawAnimation::default();
    raw_anim.duration = 1f32;

    let mut new_track = RawJointTrack::default();
    new_track.rotations.push(RawRotationKey {
        time: 0.3f32,
        value: Quat::from_axis_angle(Vec3::X, 30f32.to_radians()),
    });
    new_track.translations.push(RawTranslationKey {
        time: 0.5f32,
        value: Vec3::new(10f32, 0f32, 0f32),
    });
    new_track.translations.push(RawTranslationKey {
        time: 0.8f32,
        value: Vec3::new(20f32, 0f32, 0f32),
    });
    raw_anim.tracks.push(new_track);

    new_track = RawJointTrack::default();
    new_track.rotations.push(RawRotationKey {
        time: 0.25f32,
        value: Quat::from_axis_angle(Vec3::X, 30f32.to_radians()),
    });
    new_track.translations.push(RawTranslationKey {
        time: 0.6f32,
        value: Vec3::new(10f32, 0f32, 0f32),
    });
    new_track.translations.push(RawTranslationKey {
        time: 0.9f32,
        value: Vec3::new(20f32, 0f32, 0f32),
    });
    raw_anim.tracks.push(new_track);

    let anim = AnimationBuilder::build(&raw_anim);
    let mut outs: Vec<TransformMatrix> = vec![];

    let mut job = SamplingJob::create(&anim);
    job.run(&anim, &mut outs, 0.1f32);
    job.run(&anim, &mut outs, 0.2f32);
}
