use seija_asset::Handle;
use seija_render::{resource::Texture, material::Material};
use crate::common::ImageGenericInfo;

pub struct Image {
    pub(crate) common:ImageGenericInfo,
    pub(crate) texture:Handle<Texture>,
    pub(crate) custom_material:Handle<Material>,
}