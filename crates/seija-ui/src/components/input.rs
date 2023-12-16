use std::collections::HashMap;

use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Component;
use bevy_ecs::system::{Res, Resource, ResMut, Commands};
use bevy_ecs::{system::{SystemParam, Query}, prelude::Entity, query::{Or, Changed}};
use glyph_brush::ab_glyph::{FontArc, Font, PxScale, Glyph, PxScaleFont, ScaleFont};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::math::{Vec3, Vec2, Vec4};
use seija_core::time::Time;
use seija_core::window::AppWindow;
use seija_render::material::Material;
use seija_transform::{Transform,events::EntityCommandsEx}; 
use crate::event::UIEvent;
use crate::mesh2d::{Vertex2D, Mesh2D};
use crate::render::UIRender2D;
use crate::system::UIRenderRoot;
use crate::text::{Text, self, Font as TextFont};

use super::rect2d::Rect2D;
use seija_input::Input as SysInput;
#[derive(Component,Debug,Clone,Default)]
#[repr(C)] 
pub struct Input {
    pub text_entity:Option<Entity>,
    pub text:String
}


#[derive(Resource,Default)]
pub struct InputTextSystemData {
    cache_dict:HashMap<Entity,InputTextCache>,
    active_input:Option<Entity>
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
    pub cache_rect:Rect2D
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
            cache_rect:Rect2D::default()
        }
    }
}


#[derive(SystemParam)]
pub struct InputParams<'w,'s> {
    pub(crate) update_inputs:Query<'w,'s,(Entity,&'static Input,&'static Rect2D),Or<(Changed<Input>,Changed<Rect2D>)>>,
    pub(crate) texts:Query<'w,'s,&'static mut Text>,
    pub(crate) sys_data:ResMut<'w,InputTextSystemData>,
    pub(crate) commands:Commands<'w,'s>,
    pub(crate) ui_roots:Res<'w,UIRenderRoot>,
}

pub fn input_system(mut params:InputParams,
                   font_assets:Res<Assets<TextFont>>,
                   server:Res<AssetServer>,
                   mut mat_assets:ResMut<Assets<Material>>,
                   mut ui_events:EventReader<UIEvent>,
                   time:Res<Time>,
                   window:Res<AppWindow>,
                   trans:Query<&Transform>) {
   for (entity,input,rect) in params.update_inputs.iter() {
     //init input
     if !params.sys_data.cache_dict.contains_key(&entity) {
        init_input(entity, input,&rect, &mut params.sys_data, &mut params.texts,
                   &mut params.commands,&params.ui_roots,&server,&mut mat_assets);
     }
   }

   for ev in ui_events.iter() {
     if let Some(input_cache) = params.sys_data.cache_dict.get_mut(&ev.entity) {
        let t = trans.get(ev.entity).unwrap();
        //这里，如果从UICanvas到当前节点有其他缩放会有问题
        let end_postion = (1f32 / t.global.scale) * t.global.position;
        click_input(input_cache,&mut mat_assets,&window,end_postion);  
     }
     params.sys_data.active_input = Some(ev.entity);
   }

   flash_input(&mut params.sys_data, &mut mat_assets, &time);
}


fn init_input(entity:Entity,input:&Input,rect:&Rect2D,sys_data:&mut InputTextSystemData,
              texts:&mut Query<&mut Text>,commands:&mut Commands,root:&UIRenderRoot
              ,server:&AssetServer,mats:&mut Assets<Material>) {
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
    sys_data.cache_dict.insert(entity, text_cache);
    if let Some(text_entity) = input.text_entity {
        if let Ok(mut text) = texts.get_mut(text_entity) {
            text.text = input.text.clone();
        }
    }
}

fn click_input(input_cache:&mut InputTextCache,mats:&mut Assets<Material>,window:&AppWindow,input_pos:Vec3) {
    let mut window_pos = world_to_window(Vec2::new(input_pos.x, input_pos.y),window);
    window_pos.x -= input_cache.cache_rect.width * 0.5f32;
    window_pos.y += input_cache.cache_rect.height * 0.5f32;
    
    input_cache.is_show = true;
    input_cache.time = 0f32;
    window.set_ime_allowed(true);
    window.set_ime_position(window_pos);
    if let Some(caret_material) = mats.get_mut(&input_cache.caret_mat.id) {
        caret_material.props.set_float4("color", Vec4::new(0f32, 1f32, 0f32, 1f32), 0);
    }
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
                    if cache.is_show {
                        caret_material.props.set_float4("color", Vec4::new(0f32, 1f32, 0f32, 0f32), 0); 
                    } else {
                        caret_material.props.set_float4("color", Vec4::new(0f32, 1f32, 0f32, 1f32), 0);
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
    let caret_width = 1f32;
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