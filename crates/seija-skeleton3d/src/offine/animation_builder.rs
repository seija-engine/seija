use std::{cmp::Ordering};

use glam::{Vec3, Quat, Vec4};

use crate::animation::{Animation, Float3Key, QuaternionKey};

use super::raw_animation::{RawAnimation, RawTranslationKey, RawRotationKey, RawScaleKey};

pub struct AnimationBuilder;

impl AnimationBuilder {
    pub fn build(raw_animation:&RawAnimation) -> Animation {
        let mut animation = Animation::default();
        animation.name = raw_animation.name.clone();
        animation.duration = raw_animation.duration;
        animation.num_tracks = raw_animation.num_tracks();

        let mut translations:usize = 0;
        let mut rotations:usize = 0; 
        let mut scales:usize = 0;
        for track in raw_animation.tracks.iter() {
            translations += track.translations.len() + 2;
            rotations += track.rotations.len() + 2;
            scales += track.scales.len() + 2;
        }
        let mut sorting_translations:Vec<SortingTranslationKey> = Vec::with_capacity(translations);
        let mut sorting_rotations:Vec<SortingRotationKey> = Vec::with_capacity(rotations);
        let mut sorting_scales:Vec<SortingScaleKey> = Vec::with_capacity(scales);
        
        for (index,track) in raw_animation.tracks.iter().enumerate() {
            Self::copy_to_sort(&track.translations, index, raw_animation.duration, &mut sorting_translations);
            Self::copy_to_sort(&track.rotations, index, raw_animation.duration, &mut sorting_rotations);
            Self::copy_to_sort(&track.scales, index, raw_animation.duration, &mut sorting_scales);
        }

        let inv_duration:f32 = 1f32 / raw_animation.duration;
        Self::copy_to_animation_v3(&mut sorting_translations,&mut animation.translations_,inv_duration);
        Self::copy_to_animation_v3(&mut sorting_scales,&mut animation.scales_,inv_duration);
        Self::copy_to_animation_quat(&mut sorting_rotations,&mut animation.rotations_,inv_duration);

        animation
    }

    fn copy_to_sort<ST>(src_list:&Vec<ST::AssocType>,track:usize,duration:f32,dst_list:&mut Vec<ST>) where ST:ISortKey {
        if src_list.len() == 0 {
            Self::push_back_identity_key(track, 0f32, dst_list);
            Self::push_back_identity_key(track, duration, dst_list);
        } else if src_list.len() == 1 {
            let raw_key = src_list.first().unwrap();
            let mut first = ST::create(track as u16, -1f32, raw_key.clone());
            first.set_key_time(0f32);
            dst_list.push(first);
            let mut last = ST::create(track as u16, 0f32, raw_key.clone());
            last.set_key_time(duration);
            dst_list.push(last);
           
        } else {
            let mut prev_time = -1f32;
            if let Some(fst) = src_list.first() {
                if ST::key_time(fst) != 0f32 {
                    let mut first = ST::create(track as u16, prev_time, fst.clone());
                    first.set_key_time(0f32);
                    dst_list.push(first);
                    prev_time = 0f32;
                }
            }
            for raw_key in src_list.iter() {
                let mut key = ST::create(track as u16, prev_time, raw_key.clone());
                key.set_key_time(ST::key_time(raw_key));
                dst_list.push(key);
                prev_time = ST::key_time(raw_key);
            }
            if let Some(back) = src_list.last() {
                if ST::key_time(back) - duration != 0f32 {
                    let mut last = ST::create(track as u16, prev_time, back.clone());
                    last.set_key_time(duration);
                    dst_list.push(last);
                }
            }
        }
    }

    fn copy_to_animation_v3<ST>(src:&mut Vec<ST>,dst:&mut Vec<Float3Key>,inv_duration:f32) where ST:ISortKey {
        src.sort_by(Self::sort_fn);
        for item in src.iter() {
            let mut key = Float3Key::default();
            key.ratio = ST::key_time(item.key()) * inv_duration;
            key.track = item.track() as usize;
            if let Ok(v3) = item.value() {
                key.value = v3.clone();
            }
            dst.push(key);
        }
    }

    fn sort_fn<ST>(a:&ST,b:&ST) -> Ordering where ST:ISortKey {
        let time_diff = a.get_prev_key_time() - b.get_prev_key_time();
        if time_diff == 0f32 {
            return a.track().cmp(&b.track());
        } else {
            a.get_prev_key_time().partial_cmp(&b.get_prev_key_time()).unwrap_or( Ordering::Equal)
        }
    }

    fn copy_to_animation_quat(src:&mut Vec<SortingRotationKey>,dst:&mut Vec<QuaternionKey>,inv_duration:f32) {
        let ident:Quat = Quat::IDENTITY;
        let mut track:u16 = u16::MAX;
        for idx in 0..src.len() {
            let mut normalized = Self::safe_normal_quat(&src[idx].key.value,&ident);
            if track != src[idx].track {
                if normalized.w < 0f32 {
                    normalized = -normalized;
                }
            } else {
                let prev_k = &src[idx - 1];
                let prev = Vec4::new(prev_k.key.value.x, prev_k.key.value.y, prev_k.key.value.z, prev_k.key.value.w);
                let now_k = &src[idx];
                let now = Vec4::new(now_k.key.value.x, now_k.key.value.y, now_k.key.value.z, now_k.key.value.w);
                if prev.dot(now) < 0f32 {
                    normalized = -normalized;
                }
            }
            src[idx].key.value = normalized;
            track = src[idx].track;
        }
        src.sort_by(Self::sort_fn);
        for item in src.iter() {
            let mut dst_key = QuaternionKey::default();
            dst_key.ratio = item.key.time * inv_duration;
            dst_key.track = item.track as usize;
            dst_key.value = item.key.value;
            dst.push(dst_key);
        }
    }

    fn safe_normal_quat(q:&Quat,safer:&Quat) -> Quat {
        let sq_len:f32 = q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w;
        if sq_len == 0f32 {return safer.clone() }
        let inv_len:f32 = 1f32 / sq_len.sqrt();
        Quat::from_array([q.x * inv_len,q.y * inv_len,q.z * inv_len,q.w * inv_len])
    }

    fn push_back_identity_key<ST>(track:usize,duration:f32,dst_list:&mut Vec<ST>) where ST:ISortKey {
        let mut prev_time:f32 = -1f32;
        if let Some(last) = dst_list.last() {
            if last.track() as usize == track {
                prev_time = ST::key_time(last.key());
            }
        }
        let mut key = ST::create(track as u16, prev_time,ST::AssocType::default());
        key.set_key_time(duration);
        dst_list.push(key);
    }
}


trait ISortKey {
    type AssocType:Default + Clone;
    fn create(track:u16,prev_key_time:f32,key:Self::AssocType) -> Self;
    fn track(&self) -> u16;
    fn key(&self) -> &Self::AssocType;
    fn key_time(assoc:&Self::AssocType) -> f32;
    fn set_key_time(&mut self,t:f32);
    fn value(&self) -> Result<&Vec3,&Quat>;
    fn get_prev_key_time(&self) -> f32;

}

struct SortingTranslationKey {
    pub track:u16,
    pub prev_key_time:f32,
    pub key:RawTranslationKey
}

impl ISortKey for SortingTranslationKey {
    type AssocType = RawTranslationKey;
    fn track(&self) -> u16 { self.track }
    fn key_time(assoc:&Self::AssocType) -> f32 { assoc.time }
    fn key(&self) -> &Self::AssocType { &self.key }
    fn set_key_time(&mut self,t:f32) {self.key.time = t; }
    fn get_prev_key_time(&self) -> f32 {self.prev_key_time }
    fn value(&self) -> Result<&Vec3,&Quat> { Ok(&self.key.value)  }
    fn create(track:u16,prev_key_time:f32,key:Self::AssocType) -> Self {
        Self {track,prev_key_time,key }
    }

    
}

struct SortingRotationKey {
    pub track:u16,
    pub prev_key_time:f32,
    pub key:RawRotationKey
}

impl ISortKey for SortingRotationKey {
    type AssocType = RawRotationKey;
    fn track(&self) -> u16 { self.track }
    fn key_time(assoc:&Self::AssocType) -> f32 { assoc.time }
    fn set_key_time(&mut self,t:f32) {self.key.time = t; }
    fn value(&self) -> Result<&Vec3,&Quat> { Err(&self.key.value)  }
    fn key(&self) -> &Self::AssocType { &self.key }
    fn get_prev_key_time(&self) -> f32 {self.prev_key_time }
    fn create(track:u16,prev_key_time:f32,key:Self::AssocType) -> Self {
        Self {track,prev_key_time,key }
    }
}

struct SortingScaleKey {
    pub track:u16,
    pub prev_key_time:f32,
    pub key:RawScaleKey
}

impl ISortKey for SortingScaleKey { 
    type AssocType = RawScaleKey;
    fn track(&self) -> u16 { self.track }
    fn key_time(assoc:&Self::AssocType) -> f32 { assoc.time }
    fn set_key_time(&mut self,t:f32) {self.key.time = t; }
    fn value(&self) -> Result<&Vec3,&Quat> { Ok(&self.key.value)  }
    fn key(&self) -> &Self::AssocType { &self.key }
    fn get_prev_key_time(&self) -> f32 {self.prev_key_time }
    fn create(track:u16,prev_key_time:f32,key:Self::AssocType) -> Self {
        Self {track,prev_key_time,key }
    }
}