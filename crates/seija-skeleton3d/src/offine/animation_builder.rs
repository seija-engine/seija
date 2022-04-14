use std::cmp::Ordering;

use glam::Vec3;

use crate::animation::Animation;

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
        let mut sorting_rotations:Vec<SortingRotationKey> = Vec::with_capacity(translations);
        let mut sorting_scales:Vec<SortingScaleKey> = Vec::with_capacity(translations);
        
        for (index,track) in raw_animation.tracks.iter().enumerate() {
            Self::copy_to_sort(&track.translations, index, raw_animation.duration, &mut sorting_translations);
            Self::copy_to_sort(&track.rotations, index, raw_animation.duration, &mut sorting_rotations);
            Self::copy_to_sort(&track.scales, index, raw_animation.duration, &mut sorting_scales);
        }

        let inv_duration:f32 = 1f32 / raw_animation.duration;
        
        animation
    }

    fn copy_to_sort<ST>(src_list:&Vec<ST::AssocType>,track:usize,duration:f32,dst_list:&mut Vec<ST>) where ST:ISortKey {
        if src_list.len() == 0 {
            Self::push_back_identity_key(track, 0f32, dst_list);
            Self::push_back_identity_key(track, duration, dst_list);
        } else if src_list.len() == 1 {
            let raw_key = src_list.first().unwrap();
            let first = ST::create(track as u16, -1f32, raw_key.clone());
            dst_list.push(first);
            let mut last = ST::create(track as u16, 0f32, raw_key.clone());
            last.set_key_time(duration);
            dst_list.push(last);
           
        } else {
            let mut prev_time = -1f32;
            if let Some(fst) = src_list.first() {
                if ST::key_time(fst) != 0f32 {
                    let first = ST::create(track as u16, prev_time, fst.clone());
                    dst_list.push(first);
                    prev_time = 0f32;
                }
            }
            for raw_key in src_list.iter() {
                let key = ST::create(track as u16, prev_time, raw_key.clone());
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

    fn copy_to_animation_v3<ST>(src:&mut Vec<ST>,dst:&mut Vec<Vec3>,inv_duration:f32) where ST:ISortKey {
        src.sort_by(|a,b| {
            let time_diff = a.get_prev_key_time() - b.get_prev_key_time();
            if time_diff == 0f32 {
                return a.track().cmp(&b.track());
            } else {
                a.get_prev_key_time().partial_cmp(&b.get_prev_key_time()).unwrap_or( Ordering::Equal)
            }
        });
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
    fn key(&self) -> &Self::AssocType { &self.key }
    fn get_prev_key_time(&self) -> f32 {self.prev_key_time }
    fn create(track:u16,prev_key_time:f32,key:Self::AssocType) -> Self {
        Self {track,prev_key_time,key }
    }
}