use std::collections::HashMap;

use seija_asset::{IAssetLoader, AssetDynamic,HandleUntyped, add_to_asset_type, AssetServer, AssetLoaderParams, this_asset_path, Handle};
use seija_core::{TypeUuid, OptionExt};
use seija_core::bevy_ecs::prelude::*;
use seija_core::anyhow;
use seija_asset::async_trait::async_trait;
use seija_core::smol_str::SmolStr;
use seija_core::smol;
use seija_render::resource::Texture;
use crate::{SpriteSheet, MetaData, SpriteInfo, Rect};
use serde::{Deserialize};
use relative_path::RelativePath;

#[derive(Default)]
pub struct SpriteSheetLoader;

#[derive(Deserialize)]
struct SerdeSprite {
    height:u32,
    width:u32,
    x:u32,
    y:u32,
    name:SmolStr
}

#[derive(Deserialize)]
struct SerdeData {
    pub meta:MetaData,
    pub sprites:Vec<SerdeSprite>
}

impl SpriteSheetLoader {
    fn create(texture:Handle<Texture>,serde_data:SerdeData) -> SpriteSheet {
        let mut name_dict:HashMap<SmolStr,usize> = HashMap::new();
        let mut sprites:Vec<SpriteInfo> = vec![];
        for (index,s) in serde_data.sprites.iter().enumerate() {
            name_dict.insert(s.name.clone(), index);
            let info = SpriteInfo {
                rect:Rect { x:s.x, y:s.y, width:s.width, height:s.height },
                uv: Rect { 
                  x:s.x as f32 / serde_data.meta.width as f32, 
                  y:s.y as f32 / serde_data.meta.height as f32, 
                  width:s.width as f32 / serde_data.meta.width as f32, 
                  height:s.height as f32 / serde_data.meta.height as f32 
              },
            };
            sprites.push(info);
        }

        SpriteSheet {
            texture,
            meta:serde_data.meta,
            sprites,
            name_dict
        }
    }
}

#[async_trait]
impl IAssetLoader for SpriteSheetLoader {
    fn typ(&self) -> seija_core::uuid::Uuid { SpriteSheet::TYPE_UUID }

    fn add_to_asset(&self,world: &mut World,res:Box<dyn AssetDynamic>) -> anyhow::Result<HandleUntyped>  { add_to_asset_type::<SpriteSheet>(world, res) }


    fn sync_load(&self,world:&mut World,path:&str,server:&AssetServer,_:Option<Box<dyn AssetLoaderParams>>) -> anyhow::Result<Box<dyn AssetDynamic>> {
        let file_path = RelativePath::new(path).parent().get()?;
        let full_path = server.full_path(path)?;
        let bytes = std::fs::read(&full_path)?;
        let serde_data:SerdeData = serde_json::from_slice::<SerdeData>(bytes.as_slice())?;
        let server = world.get_resource::<AssetServer>().unwrap().clone();
        let texture_path = this_asset_path(file_path, serde_data.meta.texture.as_str());
        let h_texture = server.load_sync::<Texture>(world,texture_path.as_str(),None)?;
        Ok(Box::new(SpriteSheetLoader::create(h_texture, serde_data)))
    }

    async fn async_load(&self,server:AssetServer,path:SmolStr,
        _:Option<Box<dyn seija_asset::downcast_rs::DowncastSync>>,
        _:Option<Box<dyn AssetLoaderParams>>) -> anyhow::Result<Box<dyn AssetDynamic>> {
            let file_path = RelativePath::new(path.as_str()).parent().get()?;
            let full_path = server.full_path(path.as_str())?;
            let bytes = smol::fs::read(&full_path).await?;
            let serde_data:SerdeData = serde_json::from_slice::<SerdeData>(bytes.as_slice())?;
            let texture_path = this_asset_path(file_path, serde_data.meta.texture.as_str());
            let req = server.load_async::<Texture>(texture_path.as_str(),None)?;
            let h_texture = req.wait_handle().await.get()?.typed::<Texture>();
            Ok(Box::new(SpriteSheetLoader::create(h_texture, serde_data)))
    }

    
}