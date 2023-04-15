use bevy_ecs::prelude::World;
use seija_asset::{IAssetLoader, HandleUntyped, add_to_asset_type};
use seija_asset::async_trait::async_trait;
use serde_json::Value;
use smol_str::SmolStr;
use seija_asset::{AssetLoaderParams, AssetServer, AssetDynamic, downcast_rs::DowncastSync};
use seija_core::TypeUuid;
use seija_core::anyhow::{Result,anyhow};
use crate::resource::Texture;
use seija_core::smol;
use super::{TextureDescInfo, ImageInfo, read_image_info};

#[derive(Default)]
pub(crate) struct  TextureLoader;
#[async_trait]
impl IAssetLoader for TextureLoader {
    fn typ(&self) -> uuid::Uuid { Texture::TYPE_UUID }
    fn add_to_asset(&self, world:&mut World, res:Box<dyn AssetDynamic>) -> Result<HandleUntyped> {
        add_to_asset_type::<Texture>(world, res)
    }
    fn sync_load(&self,_:&mut World,path:&str,server:&AssetServer,params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        if path.ends_with(".json") {
            let json_path = server.full_path(path)?;
            let json_bytes = std::fs::read(json_path.as_path())?;
            let cube_json:Value = serde_json::from_slice(&json_bytes)?;
            let path_list = read_cube_json_path(cube_json)?;
            let mut image_bytes:Vec<Vec<u8>> = vec![];
            for path in path_list.iter() {
               let image_path = json_path.parent().unwrap().join(path.as_str());
               let bytes = std::fs::read(image_path)?;
               image_bytes.push(bytes);
            }
            let texture = make_cube_map(image_bytes)?;
            return Ok(Box::new(texture));
        }
        let full_path = server.full_path(path)?;
        log::error!("full_path:{}",full_path.display());
        let bytes = std::fs::read(full_path)?;
        let texture = Texture::from_image_bytes(&bytes, read_desc(params))?;
        Ok(Box::new(texture))
    }

    async fn async_load(&self,server:AssetServer,path:SmolStr,
                        _:Option<Box<dyn DowncastSync>>,
                        params:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        if path.ends_with(".json") {
            let json_path = server.full_path(path.as_str())?;
            let json_bytes = smol::fs::read(json_path.as_path()).await?;
            let cube_json:Value = serde_json::from_slice(&json_bytes)?;
            let path_list = read_cube_json_path(cube_json)?;
            let mut image_bytes:Vec<Vec<u8>> = vec![];
            for path in path_list.iter() {
               let image_path = json_path.parent().unwrap().join(path.as_str());
               let bytes = smol::fs::read(image_path).await?;
               image_bytes.push(bytes);
            }
            let texture = make_cube_map(image_bytes)?;
            return Ok(Box::new(texture));
        }
        let full_path = server.full_path(path.as_str())?;
        let bytes = smol::fs::read(full_path).await?;
        let texture = Texture::from_image_bytes(&bytes, read_desc(params))?;
        Ok(Box::new(texture))
    }
}

impl AssetLoaderParams for TextureDescInfo {}

fn read_desc(params:Option<Box<dyn AssetLoaderParams>>) -> TextureDescInfo {
    params.and_then(|v| v.downcast::<TextureDescInfo>().ok())
    .map(|v| *v)
    .unwrap_or(Default::default())
}


fn read_cube_json_path(cube_json:Value) -> Result<Vec<SmolStr>> {
    let left = cube_json.get("left").and_then(Value::as_str).ok_or(anyhow!("left"))?;
    let right = cube_json.get("right").and_then(Value::as_str).ok_or(anyhow!("right"))?;
    let top = cube_json.get("top").and_then(Value::as_str).ok_or(anyhow!("top"))?;
    let bottom = cube_json.get("bottom").and_then(Value::as_str).ok_or(anyhow!("bottom"))?;
    let back = cube_json.get("back").and_then(Value::as_str).ok_or(anyhow!("back"))?;
    let front = cube_json.get("front").and_then(Value::as_str).ok_or(anyhow!("front"))?;
    Ok(vec![left.into(),right.into(),top.into(),bottom.into(),back.into(),front.into()])
}

fn make_cube_map(images:Vec<Vec<u8>>) -> Result<Texture> {
    let mut image_infos:Vec<ImageInfo> = vec![];
    for image_bytes in images.iter() {
        let dyn_image = image::load_from_memory(&image_bytes)?;
        let image_info = read_image_info(dyn_image);
        image_infos.push(image_info)
    }
    let fst = &image_infos[0];
    let byte_length = fst.data.len();
    let mut all_bytes:Vec<u8> = vec![0;fst.data.len() * 6];
    for index in 0..6usize {
        let start = index * byte_length;
        let end = start + byte_length;
        let data_ref = &image_infos[index].data;
        all_bytes[start..end].clone_from_slice(data_ref);
    }
    let info = ImageInfo {
        width:fst.width,
        height:fst.height,
        format:fst.format,
        data:all_bytes
    };
    let mut desc = TextureDescInfo::default();
    desc.desc.size.depth_or_array_layers = 6;
    //desc.desc.dimension = wgpu::TextureDimension::D3;
    desc.view_desc.dimension = Some(wgpu::TextureViewDimension::Cube);
    let texture = Texture::create_image(info, desc);
    Ok(texture)
}