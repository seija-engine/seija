use seija_asset::Handle;
use seija_render::resource::Texture;
use crate::common::ImageGenericInfo;

pub struct Image {
    common:ImageGenericInfo,
    texture:Handle<Texture>
}