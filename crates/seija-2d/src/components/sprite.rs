use std::sync::atomic::{AtomicBool, Ordering};

use bevy_ecs::component::Component;
use seija_asset::{Handle, Assets};
use seija_core::math::{Vec4, Vec3};
use seija_render::{material::Material, resource::{Mesh, MeshAttributeType, Indices}};
use spritesheet::{SpriteSheet, SpriteInfo};
use wgpu::PrimitiveTopology;
use crate::common::Rect2D;


#[derive(Component)]
pub struct Sprite2D {
    pub(crate) color:Vec4,
    pub(crate) sheet:Option<Handle<SpriteSheet>>,
    pub(crate) custom_material:Option<Handle<Material>>,
    pub(crate) sprite_index:usize,

    pub(crate) is_material_dirty:AtomicBool,
    pub(crate) is_sheet_dirty:AtomicBool,
}

impl Sprite2D {
    pub fn simple(sheet:Option<Handle<SpriteSheet>>,sprite_index:usize,color:Vec4) -> Self { 
        Sprite2D { 
            color,
            sheet, 
            custom_material: None, 
            sprite_index,
            is_material_dirty:AtomicBool::new(false),
            is_sheet_dirty:AtomicBool::new(false),
        }
    }

    pub fn set_sprite_index(&mut self,index:usize) {
        self.sprite_index = index;
        self.is_material_dirty.store(true, Ordering::SeqCst);
    }

    pub fn set_color(&mut self,color:Vec4) {
        self.color = color;
        self.is_material_dirty.store(true, Ordering::SeqCst);
    }

    pub fn set_sheet(&mut self,sheet:Option<Handle<SpriteSheet>>) {
        self.sheet = sheet;
        self.is_sheet_dirty.store(true, Ordering::SeqCst)
    }

    pub fn is_material_dirty(&self) -> bool {
        self.is_material_dirty.load(Ordering::SeqCst)
    }

    pub fn is_sheet_dirty(&self) -> bool {
        self.is_sheet_dirty.load(Ordering::SeqCst)
    }

    pub fn clear_dirtys(&self) {
        self.is_material_dirty.store(false, Ordering::SeqCst);
        self.is_sheet_dirty.store(false, Ordering::SeqCst);
    }

    pub fn build_mesh(&self,rect2d:&Rect2D) -> Mesh {
        Self::build_simple_mesh(rect2d)
    }

    fn build_simple_mesh(rect2d:&Rect2D) -> Mesh {
        let offset_x = -rect2d.width  * rect2d.anchor[0];
        let offset_y = -rect2d.height * rect2d.anchor[1];
        let indices = vec![2,1,0,2,3,1];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let lt = Vec3::new(0f32 + offset_x, rect2d.height + offset_y, 0f32);
        let rt = Vec3::new(rect2d.width + offset_x,rect2d.height + offset_y, 0f32);
        let lb = Vec3::new(0f32 + offset_x,0f32 + offset_y, 0f32);
        let rb = Vec3::new(rect2d.width + offset_x,0f32 + offset_y, 0f32);
        let positions:Vec<[f32;3]> = vec![lt.into(),rt.into(),lb.into(),rb.into()];
        mesh.set(MeshAttributeType::POSITION, positions);
        mesh.set(MeshAttributeType::INDEX0, vec![0i32,1i32,2i32,3i32]);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.build();
        mesh
    }
}