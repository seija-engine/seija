use std::process::id;

use crate::{Animation, Float3Key, offine::raw_animation::RawScaleKey};
use glam::{Quat, Vec3};
use seija_transform::TransformMatrix;

#[derive(Default)]
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
            last_ratio: -1f32,
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

    pub fn run(&mut self, animation: &Animation, output: &mut Vec<TransformMatrix>, ratio: f32) -> bool {
        if animation.num_tracks == 0 || animation.num_tracks != output.len() {
            return false;
        }
       
        let anim_ratio = ratio.clamp(0f32, 1f32);
        if self.last_ratio < 0f32 || anim_ratio < self.last_ratio {
            self.search_cur_keys(animation, anim_ratio);
        } else {
            for (track_index,key_index) in self.track_cur_indexs.iter_mut().enumerate() {
                let cache_track = &self.cache_tracks[track_index];
                for index in key_index.trans_index..cache_track.translations.len() {
                   let cur_key = &animation.translations_[cache_track.translations[index]];
                   if cur_key.ratio >= ratio {
                       key_index.trans_index = index - 1;
                       break;
                   }
                }

                for index in key_index.scale_index..cache_track.scales.len() {
                    let cur_key = &animation.scales_[cache_track.scales[index]];
                    if cur_key.ratio >= ratio {
                        key_index.scale_index = index - 1;
                        break;
                    }
                 }

                 for index in key_index.rotation_index..cache_track.rotations.len() {
                    let cur_key = &animation.rotations_[cache_track.rotations[index]];
                    if cur_key.ratio >= ratio {
                        key_index.rotation_index = index - 1;
                        break;
                    }
                 }
            }
        }
       
       
        for (track_index,key_index) in self.track_cur_indexs.iter().enumerate() {
            let cur_cache_track = &self.cache_tracks[track_index];
            let cur_t_index = key_index.trans_index;
            if cur_cache_track.translations.len() >= cur_t_index + 1 {
                let cur_key_index = cur_cache_track.translations[cur_t_index];
                let next_key_index = cur_cache_track.translations[cur_t_index + 1];
                let cur_key = &animation.translations_[cur_key_index];
                let next_key = &animation.translations_[next_key_index];
                let interp_t_ratio = (ratio - cur_key.ratio) * (1f32 / (next_key.ratio - cur_key.ratio));
              
                let lerp_postion = cur_key.value.lerp(next_key.value, interp_t_ratio);
                output[cur_key.track].position = lerp_postion;
            }

            let cur_s_index = key_index.scale_index;
            if cur_cache_track.scales.len() >= cur_s_index + 1 {
                let cur_key = &animation.scales_[cur_cache_track.scales[cur_s_index]];
                let next_key = &animation.scales_[cur_cache_track.scales[cur_s_index + 1]];
                let interp_s_ratio = (ratio - cur_key.ratio) * (1f32 / (next_key.ratio - cur_key.ratio));
                let lerp_scale = cur_key.value.lerp(next_key.value, interp_s_ratio);
                output[cur_key.track].scale = lerp_scale;
            }

            let cur_r_index = key_index.rotation_index;
            if cur_cache_track.rotations.len() >= cur_r_index + 1 {
                let cur_key = &animation.rotations_[cur_cache_track.rotations[cur_r_index]];
                let next_key = &animation.rotations_[cur_cache_track.rotations[cur_r_index + 1]];
                let interp_r_ratio = (ratio - cur_key.ratio) * (1f32 / (next_key.ratio - cur_key.ratio));
                let lerp_rotation = cur_key.value.lerp(next_key.value, interp_r_ratio);
                output[cur_key.track].rotation = lerp_rotation;
            }
        }       
        self.last_ratio = ratio;
        true
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
    new_track.translations.push(RawTranslationKey {
        time: 0.0f32,
        value: Vec3::new(10f32, 0f32, 0f32),
    });
    new_track.translations.push(RawTranslationKey {
        time: 0.5f32,
        value: Vec3::new(20f32, 0f32, 0f32),
    });
    new_track.translations.push(RawTranslationKey {
        time: 1f32,
        value: Vec3::new(100f32, 0f32, 0f32),
    });

    new_track.scales.push(RawScaleKey {
        time: 0f32,
        value: Vec3::new(0f32, 0f32, 0f32),
    });
    new_track.scales.push(RawScaleKey {
        time: 1f32,
        value: Vec3::new(10f32, 0f32, 0f32),
    });

    new_track.rotations.push(RawRotationKey {
        time: 0f32,
        value: Quat::from_rotation_x(0f32),
    });
    new_track.rotations.push(RawRotationKey {
        time: 1f32,
        value: Quat::from_rotation_x(90f32.to_radians()),
    });
    raw_anim.tracks.push(new_track);

    

    let anim = AnimationBuilder::build(&raw_anim);
    let mut outs: Vec<TransformMatrix> = vec![];
    outs.push(TransformMatrix::default());
    println!("0 {:?}",outs);
    println!("===================");
    let mut job = SamplingJob::create(&anim);
    job.run(&anim, &mut outs, 0f32);
    println!("1 {:?}",outs);
    job.run(&anim, &mut outs, 0.25f32);
    println!("2 {:?}",outs);
    job.run(&anim, &mut outs, 0.5f32);
    println!("3 {:?}",outs);
    job.run(&anim, &mut outs, 0.75f32);
    println!("4 {:?}",outs);
    job.run(&anim, &mut outs, 1f32);
    println!("5 {:?}",outs);
    println!("===================");
    //job.run(&anim, &mut outs, 0.75f32);
    //println!("6 {:?}",outs);
}


