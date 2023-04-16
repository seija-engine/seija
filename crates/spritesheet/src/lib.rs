use std::collections::HashMap;
use loader::SpriteSheetLoader;
use seija_app::IModule;
use seija_core::Rect;
use seija_core::type_uuid::{TypeUuid};
use seija_asset::{Handle, AddAsset};
use seija_render::resource::Texture;
mod loader;
use seija_core::uuid::Uuid;
use seija_core::smol_str::{SmolStr};
use serde::Deserialize;
pub mod ffi;

pub struct SpriteSheetModule;

impl IModule for SpriteSheetModule {
    fn init(&mut self,app:&mut seija_app::App) {
       app.add_asset::<SpriteSheet>();
       app.add_asset_loader::<SpriteSheet,SpriteSheetLoader>();
    }
}

#[derive(TypeUuid,Debug)]
#[uuid = "26a121e6-a1bc-d805-3452-831772db38db"]
pub struct SpriteSheet {
    pub meta:MetaData,
    pub sprites:Vec<SpriteInfo>,
    pub texture:Handle<Texture>,
    pub name_dict:HashMap<SmolStr,usize>
}

impl SpriteSheet {
    pub fn get_index(&self,name:&str) -> Option<usize> {
        self.name_dict.get(name).map(|v| *v)
    }

    pub fn get_info(&self,index:usize) -> Option<&SpriteInfo> {
        self.sprites.get(index)
    }
}

#[derive(Deserialize,Debug)]
pub struct MetaData {
    pub width:u32,
    pub height:u32,
    pub texture:SmolStr
}

#[derive(Debug)]
pub struct SpriteInfo {
    pub rect:Rect<u32>,
    pub uv:Rect<f32>
}
