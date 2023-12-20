use std::collections::HashMap;

use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Component;
use bevy_ecs::query::{Without, ChangeTrackers};
use bevy_ecs::system::{Res, Resource, ResMut, Commands,ParamSet};
use bevy_ecs::{system::{SystemParam, Query}, prelude::Entity, query::{Or, Changed}};
use glyph_brush::ab_glyph::{FontArc, Font, ScaleFont};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::math::{Vec3, Vec2, Vec4};
use seija_core::time::Time;
use seija_core::window::AppWindow;
use seija_input::event::{ImeEvent, KeyboardInput, InputState};
use seija_input::keycode::KeyCode;
use seija_render::material::Material;
use seija_transform::TransformMatrix;
use seija_transform::{Transform,events::EntityCommandsEx}; 
use crate::event::{UIEvent, UIEventType};
use crate::mesh2d::{Vertex2D, Mesh2D};
use crate::render::UIRender2D;
use crate::system::UIRenderRoot;
use crate::text::{Text, Font as TextFont};

use super::rect2d::Rect2D;
#[derive(Component,Debug,Clone,Default)]
#[repr(C)] 
pub struct Input {
    pub text_entity:Option<Entity>,
    pub text:String,
    pub font_size:u32
}


#[derive(Resource,Default)]
pub struct InputTextSystemData {
    cache_dict:HashMap<Entity,InputTextCache>,
    active_input:Option<Entity>
}

#[derive(Debug)]
pub struct CharInfo {
    chr:char,
    advance:f32
}

pub struct InputTextCache {
    pub cursor:i32,
    pub entity:Entity,
    pub text_entity:Option<Entity>,
    pub caret_entity:Entity,
    pub caret_mat:Handle<Material>,
    pub font_arc:Option<FontArc>,
    pub time:f32,
    pub is_show:bool,
    pub cache_rect:Rect2D,
    pub cache_char_infos:Vec<CharInfo>,
    pub caret_color:Vec4
}

impl InputTextCache {
    pub fn new(entity:Entity,caret_entity:Entity,caret_mat:Handle<Material>) -> InputTextCache {
        InputTextCache {
            cursor:0, 
            entity,
            text_entity:None,
            font_arc:None,
            caret_entity,
            caret_mat,
            time:0f32,
            is_show:false,
            cache_rect:Rect2D::default(),
            cache_char_infos:vec![],
            caret_color:Vec4::new(0f32, 1f32, 0.2f32, 1f32)
        }
    }
}


#[derive(SystemParam)]
pub struct InputParams<'w,'s> {
    pub(crate) texts:Query<'w,'s,&'static mut Text,Without<Input>>,
    pub(crate) sys_data:ResMut<'w,InputTextSystemData>,
    pub(crate) commands:Commands<'w,'s>,
    pub(crate) ui_roots:Res<'w,UIRenderRoot>,
}

pub fn input_system(mut params:InputParams,
                   mut sets:ParamSet<(Query<(&mut Input,&Rect2D)>,Query<Entity,Or<(Changed<Input>,Changed<Rect2D>)>>)>,
                   font_assets:Res<Assets<TextFont>>,
                   server:Res<AssetServer>,
                   mut mat_assets:ResMut<Assets<Material>>,
                   mut ui_events:EventReader<UIEvent>,
                   time:Res<Time>,
                   window:Res<AppWindow>,
                   mut trans:Query<&mut Transform>,
                   mut ime_events:EventReader<ImeEvent>,
                   mut key_events:EventReader<KeyboardInput>) {
   for entity in sets.p1().iter().collect::<Vec<_>>() {
        if let Ok((input,rect)) = sets.p0().get(entity) {
            //init input
            if !params.sys_data.cache_dict.contains_key(&entity) {
                init_input(entity, input,&rect, &mut params.sys_data, &mut params.texts,
                           &mut params.commands,&params.ui_roots,&server,&mut mat_assets,&font_assets);
            }
        }
   }

   for ev in ui_events.iter() {
     if !ev.event_type.contains(UIEventType::TOUCH_START) {
        continue;
     }
     if let Some(input_cache) = params.sys_data.cache_dict.get_mut(&ev.entity) {
        let t = trans.get(ev.entity).unwrap();
        //这里，这个转窗口坐标方式不安全
        let end_postion = (1f32 / t.global.scale) * t.global.position;
        click_input(input_cache,&mut mat_assets,&window,end_postion,&ev,&t.global);
        
        let out_pos = calc_caret_position(&input_cache);
        if let Ok(mut t) = trans.get_mut(input_cache.caret_entity) {
            t.local.position.x = out_pos.x;
        }
     }
     params.sys_data.active_input = Some(ev.entity);
   }
   flash_input(&mut params.sys_data, &mut mat_assets, &time);
   
   //接收到输入事件
   if let Some(active_entity) = params.sys_data.active_input {
    if let Some(cache) = params.sys_data.cache_dict.get_mut(&active_entity) {
        if let Some(Ok(mut text)) = cache.text_entity.map(|v|params.texts.get_mut(v))  {
            if let Ok((mut input,_)) = sets.p0().get_mut(cache.entity) {
                for event in ime_events.iter() {
                    match event {
                       ImeEvent::ReceivedCharacter(chr) => {
                        if !chr.is_control() { 
                            let font_scaled = cache.font_arc.as_ref().unwrap().as_scaled(input.font_size as f32);
                            let h_advance = font_scaled.h_advance(font_scaled.glyph_id(*chr));
                            let char_info = CharInfo {chr:*chr,advance : h_advance };
                            cache.cache_char_infos.insert((cache.cursor + 1) as usize, char_info);
                            input.text = String::from_iter(cache.cache_char_infos.iter().map(|c| c.chr));
                            text.text = input.text.clone();
                            cache.cursor = cache.cursor + 1;
                            let out_pos = calc_caret_position(&cache);
                            if let Ok(mut t) = trans.get_mut(cache.caret_entity) {
                                t.local.position.x = out_pos.x;
                            }
                        }
                       },
                       ImeEvent::Commit(s) => {
                        let font_scaled = cache.font_arc.as_ref().unwrap().as_scaled(input.font_size as f32);
                        let mut char_len = 0;
                        let mut new_chars = vec![];
                        for chr in s.chars() {
                            let h_advance = font_scaled.h_advance(font_scaled.glyph_id(chr));
                            let char_info = CharInfo {chr,advance : h_advance };
                            char_len +=1;
                            new_chars.push(char_info);
                        }
                        if cache.cache_char_infos.len() == 0 {
                            cache.cache_char_infos = new_chars;
                        } else {
                            let idx = cache.cursor as usize + 1;
                            cache.cache_char_infos.splice(idx..idx, new_chars);
                        }
                        
                        input.text = String::from_iter(cache.cache_char_infos.iter().map(|c| c.chr));
                        text.text = input.text.clone();
                        cache.cursor += char_len;
                        let out_pos = calc_caret_position(&cache);
                        if let Ok(mut t) = trans.get_mut(cache.caret_entity) {
                            t.local.position.x = out_pos.x;
                        }
                       }
                    }
                }
            }
        }        
    }
   }

   if let Some(active_entity) = params.sys_data.active_input {
    if let Some(cache) = params.sys_data.cache_dict.get_mut(&active_entity) {
        if let Some(Ok(mut text)) = cache.text_entity.map(|v|params.texts.get_mut(v))  {
            if let Ok((mut input,_)) = sets.p0().get_mut(cache.entity) {
                for key in key_events.iter() {
                    if key.state == InputState::Pressed {
                        match key.key_code {
                            KeyCode::Backspace => {
                                if cache.cursor >= 0 && cache.cursor as usize <= cache.cache_char_infos.len() { 
                                    cache.cache_char_infos.remove(cache.cursor as usize);
                                    cache.cursor -= 1;
                                    input.text = String::from_iter(cache.cache_char_infos.iter().map(|c| c.chr));
                                    text.text = input.text.clone();
                                    let out_pos = calc_caret_position(&cache);
                                    if let Ok(mut t) = trans.get_mut(cache.caret_entity) {
                                        t.local.position.x = out_pos.x;
                                    }
                                }
                            },
                            KeyCode::Left => {
                                if cache.cursor >= 0 { 
                                    cache.cursor = cache.cursor - 1;
                                    let out_pos = calc_caret_position(&cache);
                                    if let Ok(mut t) = trans.get_mut(cache.caret_entity) {
                                        t.local.position.x = out_pos.x;
                                    }
                                }
                            }
                            KeyCode::Right => {
                                if cache.cursor < cache.cache_char_infos.len() as i32 - 1 { 
                                    cache.cursor += 1;
                                    let out_pos = calc_caret_position(&cache);
                                    if let Ok(mut t) = trans.get_mut(cache.caret_entity) {
                                        t.local.position.x = out_pos.x;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
   }

}


fn init_input(entity:Entity,input:&Input,rect:&Rect2D,sys_data:&mut InputTextSystemData,
              texts:&mut Query<&mut Text,Without<Input>>,commands:&mut Commands,root:&UIRenderRoot
              ,server:&AssetServer,mats:&mut Assets<Material>,fonts:&Assets<TextFont>) {
    //create caret
    let mut caret_mat = Material::from_def(root.caret_mat_def.clone(), server).unwrap();
    caret_mat.props.set_float4("color", Vec4::new(1f32, 0f32, 0f32, 0f32), 0);
    let caret_mesh = build_input_caret_mesh(rect, 0f32);
    let h_mat = mats.add(caret_mat);
    let r2d = UIRender2D {
        mesh2d:caret_mesh,
        texture:None,
        mat_def:root.caret_mat_def.clone(),
        custom_mat:Some(h_mat.clone())
    };
    let caret_entity = commands.spawn((Transform::default(),rect.clone(),r2d)).set_parent(Some(entity)).id();

    let mut text_cache = InputTextCache::new(entity,caret_entity,h_mat);
    text_cache.text_entity = input.text_entity;
    text_cache.cache_rect = rect.clone();
    
    
    if let Some(text_entity) = input.text_entity {
        if let Ok(mut text) = texts.get_mut(text_entity) {
            text.text = input.text.clone();
            text.font_size = input.font_size;
            if let Some(font_id) = text.font.as_ref() {
               text_cache.font_arc = fonts.get(&font_id.id).map(|v| v.asset.clone());
            }
        }
    }
    update_input_char_infos(&mut text_cache,&input.text,input.font_size);
    sys_data.cache_dict.insert(entity, text_cache);
}

fn update_input_char_infos(cache:&mut InputTextCache,text:&str,font_size:u32) {
    if let Some(font_arc) = cache.font_arc.as_ref() {
       let font_scaled = font_arc.as_scaled(font_size as f32);
       cache.cache_char_infos.clear();
       for chr in text.chars() {
         let glyph_id = font_scaled.glyph_id(chr);
         let h_advance = font_scaled.h_advance(glyph_id);
         cache.cache_char_infos.push(CharInfo { chr, advance: h_advance });
       }
    }
}

fn click_input(input_cache:&mut InputTextCache,mats:&mut Assets<Material>,window:&AppWindow,input_pos:Vec3,ev:&UIEvent,t:&TransformMatrix) {
    input_cache.is_show = true;
    input_cache.time = 0f32;
    //激活ime
    window.set_ime_allowed(true);
    let mut window_pos = world_to_window(Vec2::new(input_pos.x, input_pos.y),window);
    window_pos.x -= input_cache.cache_rect.width * 0.5f32;
    window_pos.y += input_cache.cache_rect.height * 0.5f32;
    window.set_ime_position(window_pos);
    //展示光标
    if let Some(caret_material) = mats.get_mut(&input_cache.caret_mat.id) {
        let color = input_cache.caret_color;
        caret_material.props.set_float4("color", Vec4::new(color.x, color.y, color.z, 1f32), 0);
    }

    update_caret_cursor(input_cache,ev.pos,t);
}

fn update_caret_cursor(cache:&mut InputTextCache,click_pos:Vec2,t:&TransformMatrix) {
    let conv_pos = t.matrix().inverse() * Vec4::new(click_pos.x, click_pos.y, 0f32,1f32);
    let start_x = cache.cache_rect.width * 0.5f32;
    let x = conv_pos.x + start_x;
    cache.cursor = -1;
    let mut idx:i32 = 0;
    let mut offset_x = 0f32;
    loop {
        if idx as usize >= cache.cache_char_infos.len() {
            cache.cursor = cache.cache_char_infos.len() as i32;
            break;
        }
        let char_info = &cache.cache_char_infos[idx as usize];
        let half_size = char_info.advance * 0.5f32;
        let cur_x = offset_x + half_size;
        if x <= cur_x {
            cache.cursor = idx - 1; break;
        } else if x <= cur_x + half_size {
           cache.cursor = idx; break;
        } else {
            offset_x += char_info.advance;
            idx += 1
        }
    }
    if cache.cursor > 0 && cache.cursor as usize >= cache.cache_char_infos.len() {
        cache.cursor = cache.cache_char_infos.len() as i32 - 1;
    }
}

fn calc_caret_position(cache:&InputTextCache) -> Vec2 {
    let mut offset_size = 0f32;
    for (index,char_info) in cache.cache_char_infos.iter().enumerate() {
        if index as i32 <= cache.cursor  {
            offset_size += char_info.advance;
        }
    }
    Vec2::new(offset_size - cache.cache_rect.width * 0.5f32, 0f32)
}

fn world_to_window(mut mouse_pos:Vec2,window:&AppWindow) -> Vec2 {
    mouse_pos.x = mouse_pos.x + window.width() as f32 * 0.5f32;
    mouse_pos.y = window.height() as f32 * 0.5f32 - mouse_pos.y;
    mouse_pos
}

fn flash_input(sys_data:&mut InputTextSystemData,mats:&mut Assets<Material>,time:&Time) {
    if let Some(active_entity) =sys_data.active_input {
        if let Some(cache) = sys_data.cache_dict.get_mut(&active_entity) {
            cache.time = cache.time + time.delta_seconds();
            if cache.time > 0.5f32 {
                if let Some(caret_material) = mats.get_mut(&cache.caret_mat.id) {
                    let color = cache.caret_color;
                    if cache.is_show {
                        caret_material.props.set_float4("color", Vec4::new(color.x, color.y, color.z, 0f32), 0); 
                    } else {
                        caret_material.props.set_float4("color", Vec4::new(color.x, color.y, color.z, 1f32), 0);
                    }
                    cache.is_show = !cache.is_show;
                }
                cache.time = 0f32;
            }
        }
    }
}

fn build_input_caret_mesh(rect2d:&Rect2D,z_order:f32) -> Mesh2D {
    let indexs = vec![2,1,0,2,3,1];
    let caret_width = 2f32;
    let offset = caret_width / 2f32;
    let hh = rect2d.height  * 0.8f32 * 0.5f32;
    let points = vec![
          Vertex2D { //left top
             pos: Vec3::new(-offset, hh, z_order),
             uv:Vec2::new(0f32, 0f32)
          },
          Vertex2D { //right top
            pos: Vec3::new(offset, hh, z_order),
            uv:Vec2::new(1f32, 0f32)
         },
         Vertex2D {//left bottom
            pos: Vec3::new(-offset, -hh, z_order),
             uv:Vec2::new(0f32, 1f32)
         },
         Vertex2D {//right bottom
            pos: Vec3::new(offset, -hh, z_order),
            uv:Vec2::new(1f32, 1f32)
         },
    ];

    Mesh2D { 
        color:Vec4::ONE, 
        points, 
        indexs 
    }
}