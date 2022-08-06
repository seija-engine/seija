use glam::{Vec3, Quat};
use seija_skeleton3d::{self, offine::{raw_skeleton::{RawSkeleton, RawJoint}, skeleton_builder::SkeletonBuilder, raw_animation::{RawAnimation, RawJointTrack, RawTranslationKey, RawRotationKey}, animation_builder::AnimationBuilder}, Skeleton};
use lazy_static::{lazy_static};
use std::f32::consts as fconst;

static TRANS_UP:Vec3 = Vec3::ZERO;
static TRANS_DOWN:Vec3 = Vec3::Z;
static TRANS_FOOT:Vec3 = Vec3::X;
lazy_static! {
    static ref ROT_LEFT_UP:Quat = Quat::from_axis_angle(Vec3::Y, -fconst::FRAC_PI_2); 
    static ref ROT_LEFT_DOWN:Quat = Quat::from_axis_angle(Vec3::X, fconst::FRAC_PI_2) * Quat::from_axis_angle(Vec3::Y, -fconst::FRAC_PI_2);
    static ref ROT_RIGHT_UP:Quat = Quat::from_axis_angle(Vec3::Y, fconst::FRAC_PI_2); 
    static ref ROT_RIGHT_DOWN:Quat = Quat::from_axis_angle(Vec3::X, fconst::FRAC_PI_2) * Quat::from_axis_angle(Vec3::Y, -fconst::FRAC_PI_2);
}

#[allow(dead_code)]
struct MillipedeAnimation {
    slice_count:i32,
    kspin_length:f32,
    walk_cycle_length:f32,
    walk_cycle_count:i32,
    spin_loop:f32,
    duration:f32
}

impl Default for MillipedeAnimation {
    fn default() -> Self {
        let walk_cycle_count = 4;
        let walk_cycle_length = 2f32;
        let kspin_length = 0.5f32;
        Self { 
            slice_count: 3, 
            kspin_length,
            walk_cycle_length,
            walk_cycle_count,
            spin_loop: 2f32 * walk_cycle_count as f32 * walk_cycle_length / kspin_length,
            duration: 6f32
        }
    }
}

impl MillipedeAnimation {
    pub fn build(&self) {
        let raw_skeleton = self.create_skeleton();
        let skeleton = SkeletonBuilder::build(&raw_skeleton);
        let raw_animation = self.create_animation(&skeleton);
        let _animation = AnimationBuilder::build(&raw_animation);
    }

    fn pre_compute_keys(&self) -> Vec<RawTranslationKey> {
        vec![
            RawTranslationKey {time:0f32,value:Vec3::new(0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.125f32,value:Vec3::new(-0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.145f32,value:Vec3::new(-0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
            RawTranslationKey {time:0.23f32,value:Vec3::new(0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
            RawTranslationKey {time:0.25f32,value:Vec3::new(0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.375f32,value:Vec3::new(-0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.395f32,value:Vec3::new(-0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
            RawTranslationKey {time:0.48f32,value:Vec3::new(0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
            RawTranslationKey {time:0.5f32,value:Vec3::new(0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.625f32,value:Vec3::new(-0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.645f32,value:Vec3::new(-0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
            RawTranslationKey {time:0.73f32,value:Vec3::new(0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
            RawTranslationKey {time:0.75f32,value:Vec3::new(0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.875f32,value:Vec3::new(-0.25f32 * self.walk_cycle_length, 0f32, 0f32) },
            RawTranslationKey {time:0.895f32,value:Vec3::new(-0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
            RawTranslationKey {time:0.98f32,value:Vec3::new(0.17f32 * self.walk_cycle_length, 0.3f32, 0f32) },
        ]
    }

    fn create_skeleton(&self) -> RawSkeleton {
        //root
        let mut root = RawJoint::default();
        root.name = Some("root".into());
        root.transform.scale = Vec3::ONE;
        root.transform.rotation = Quat::IDENTITY;
        root.transform.position = Vec3::new(0f32, 1f32, -self.slice_count as f32 * self.kspin_length);
        
        let mut root_ref = &mut root;

        for idx in 0..self.slice_count {
            //left leg
            let mut lu = RawJoint::default();
            lu.name = Some(format!("lu_{}",idx));
            lu.transform.position = TRANS_UP;
            lu.transform.rotation = *ROT_LEFT_UP;
            lu.transform.scale = Vec3::ONE;
            
            let mut ld = RawJoint::default();
            ld.name = Some(format!("ld_{}",idx));
            ld.transform.position = TRANS_DOWN;
            ld.transform.rotation = *ROT_LEFT_DOWN;
            ld.transform.scale = Vec3::ONE;
            
            let mut lf = RawJoint::default();
            lf.name = Some(format!("lf_{}",idx));
            lf.transform.position = Vec3::X;
            lf.transform.rotation = Quat::IDENTITY;
            lf.transform.scale = Vec3::ONE;

            ld.children.push(lf);
            lu.children.push(ld);
            root_ref.children.push(lu);

            //right leg
            let mut ru = RawJoint::default();
            ru.name = Some(format!("ru_{}",idx));
            ru.transform.position = TRANS_UP;
            ru.transform.rotation = *ROT_RIGHT_UP;
            ru.transform.scale = Vec3::ONE;
            
            let mut rd = RawJoint::default();
            rd.name = Some(format!("rd_{}",idx));
            rd.transform.position = TRANS_DOWN;
            rd.transform.rotation = *ROT_RIGHT_DOWN;
            rd.transform.scale = Vec3::ONE;
            
            let mut rf = RawJoint::default();
            rf.name = Some(format!("rf_{}",idx));
            rf.transform.position = Vec3::X;
            rf.transform.rotation = Quat::IDENTITY;
            rf.transform.scale = Vec3::ONE;

            rd.children.push(rf);
            ru.children.push(rd);
            root_ref.children.push(ru);
            

            let mut sp = RawJoint::default();
            sp.name = Some(format!("sp_{}",idx));
            sp.transform.position = Vec3::new(0f32, 0f32, self.kspin_length);
            sp.transform.rotation = Quat::IDENTITY;
            sp.transform.scale = Vec3::ONE;
            root_ref.children.push(sp);
        
            root_ref = &mut root_ref.children[2];
        }

        let skeleton = RawSkeleton {roots:vec![root] };
        skeleton
    } 

    fn create_animation(&self,skeleton:&Skeleton) -> RawAnimation {
        let mut raw_animation = RawAnimation::default();
        raw_animation.duration = self.duration;
        let compute_keys = self.pre_compute_keys();
        for idx in 0..skeleton.num_joints() {
            let mut track = RawJointTrack::default();
            
            let joint_name = skeleton.joint_names[idx].as_ref().map(|v| v.as_str()).unwrap_or("");
            if joint_name.starts_with("ld") || joint_name.starts_with("rd") {
                let left = joint_name.chars().nth(0) == Some('l');
                let spine_number_str = joint_name.split('_').last().unwrap_or("");
                let spine_number = i32::from_str_radix(spine_number_str, 10).unwrap();           
                let offset = self.duration * (self.slice_count  - spine_number) as f32 / self.spin_loop;
                let phase = offset % self.duration;
                let mut i_offset = 0;
                while i_offset < compute_keys.len() && compute_keys[i_offset].time < phase  {
                    i_offset += 1;
                }
                let mut j = i_offset;
                while j < i_offset + compute_keys.len() {
                    let rkey = &compute_keys[j % compute_keys.len()];
                    let mut new_time = rkey.time - phase;
                    if new_time < 0f32 {
                        new_time = self.duration - phase + rkey.time;
                    }
                    if left {
                        let tkey = RawTranslationKey {time:new_time,value:TRANS_DOWN + rkey.value };
                        track.translations.push(tkey);
                    } else {
                        let tkey = RawTranslationKey {time:new_time,value:Vec3::new(
                            TRANS_DOWN.x - rkey.value.x,
                            TRANS_DOWN.y + rkey.value.y,
                            TRANS_DOWN.z + rkey.value.z
                        )};
                        track.translations.push(tkey);
                    }
                    j += 1;
                }

                if left {
                    let rkey = RawRotationKey {time:0f32,value:*ROT_LEFT_DOWN};
                    track.rotations.push(rkey);
                } else {
                    let rkey = RawRotationKey {time:0f32,value:*ROT_RIGHT_DOWN};
                    track.rotations.push(rkey);
                }
            } else if joint_name.starts_with("lu") {
                let tkey = RawTranslationKey {time:0f32,value:TRANS_UP};
                track.translations.push(tkey);
                let rkey = RawRotationKey {time:0f32,value:*ROT_LEFT_UP};
                track.rotations.push(rkey);
            } else if joint_name.starts_with("ru") {
                let tkey = RawTranslationKey {time:0f32,value:TRANS_UP};
                track.translations.push(tkey);
                let rkey = RawRotationKey {time:0f32,value:*ROT_RIGHT_UP};
                track.rotations.push(rkey);
            } else if joint_name.starts_with("lf") {
                let tkey = RawTranslationKey {time:0f32,value:TRANS_FOOT};
                track.translations.push(tkey);
            } else if joint_name.starts_with("rf") {
                let tkey = RawTranslationKey {time:0f32,value:TRANS_FOOT};
                track.translations.push(tkey);
            } else if joint_name.starts_with("sp") {
                let skey = RawTranslationKey {time:0f32,value:Vec3::new(0f32, 0f32, self.kspin_length) };
                track.translations.push(skey);
                let rkey = RawRotationKey {time:0f32,value:Quat::IDENTITY };
                track.rotations.push(rkey);
            } else if joint_name.starts_with("root") {
                let tkey = RawTranslationKey {
                    time:0f32,
                    value:Vec3::new(0f32, 1f32, -self.slice_count as f32 * self.kspin_length) 
                };
                track.translations.push(tkey);
            }

            
            if track.translations[0].time != 0f32 {
                let first = &track.translations[0];
                let last = &track.translations[track.translations.len() - 1];
                let lerp_time = first.time / (first.time + self.duration - last.time);
                
                let tkey = RawTranslationKey {time:0f32,value:Self::lerp(&first.value, &last.value, lerp_time) };
                track.translations.insert(0, tkey);
                
            }

            if track.translations[track.translations.len() - 1].time != self.duration {
                let first = &track.translations[0];
                let last = &track.translations[track.translations.len() - 1];
                let lerp_time = (self.duration - last.time) / (first.time + self.duration - last.time);
                let tkey = RawTranslationKey {time:self.duration,value:Self::lerp(&first.value, &last.value, lerp_time) };
                track.translations.push(tkey);
            }
            raw_animation.tracks.push(track);
        }

        
        raw_animation
    }

    fn lerp(a:&Vec3,b:&Vec3,f:f32) -> Vec3 {
        Vec3::new((b.x - a.x) * f + a.x, (b.y - a.y) * f + a.y, (b.z - a.z) * f + a.z)
    }
}

#[test]
fn test_builder() {
    let ma = MillipedeAnimation::default();
    ma.build();
}