use bevy_ecs::bundle::{Bundle};
use seija_render::Material;
use bevy_render::draw::Draw;
use bevy_render::mesh::{Mesh};
use bevy_render::prelude::{Visible};
use bevy_transform::components::{Transform,GlobalTransform};
use bevy_asset::{Handle};
#[derive(Bundle)]
pub struct Image {
    pub mesh:Handle<Mesh>,
    pub mat:Material,
    draw:Draw,
    visible:Visible,
    trans:Transform,
    global_trans:GlobalTransform
}

impl Image {
    pub fn new(mat:Material,mesh:Handle<Mesh>) -> Image {
        Image {
            mesh:mesh,
            visible:Visible::default(),
            draw:Draw::default(),
            mat,
            trans:Transform::default(),
            global_trans:GlobalTransform::default()
        }
    }
}