use bevy_ecs::component::Component;
use seija_asset::Handle;
use seija_core::{math::{Vec4, Mat4}, Rect};
use seija_render::{resource::Texture, material::Material};
use crate::common::{ImageGenericInfo, ImageType, Mesh2D, Rect2D};

#[derive(Component)]
pub struct Image {
    pub(crate) common:ImageGenericInfo,
    pub(crate) texture:Handle<Texture>,
    pub(crate) custom_material:Option<Handle<Material>>,
}

impl Image {
    pub fn new(texture:Handle<Texture>,color:Vec4) -> Self {
        let mut info = ImageGenericInfo::default();
        info.color = color;
        info.typ = ImageType::Simple;
        Image { common: info, texture, custom_material: None }
    }

    pub fn build_mesh(&self,rect2d:&Rect2D) -> Mesh2D {
        let one_rect = Rect {x:0f32,y:0f32,width:1f32,height:1f32 }; 
        self.common.build_simple_mesh(&Mat4::IDENTITY, rect2d, &one_rect,0f32)
    }
}