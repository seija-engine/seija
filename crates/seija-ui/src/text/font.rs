use bevy_ecs::world::World;
use glyph_brush::ab_glyph::FontArc;
use seija_asset::{HandleUntyped, add_to_asset_type, AssetServer, AssetLoaderParams};
use seija_asset::{IAssetLoader, AssetDynamic};
use seija_core::smol_str::SmolStr;
use seija_core::uuid::Uuid;
use seija_core::TypeUuid;
use seija_core::anyhow;
use seija_asset::async_trait::async_trait;

#[derive(Debug,TypeUuid)]
#[uuid = "088a59fc-efcc-4071-916c-a4dab4a87a3b"]
pub struct Font {
   pub asset:FontArc
}

#[derive(Default)]
pub(crate) struct FontLoader;

#[async_trait]
impl IAssetLoader for FontLoader {
    fn typ(&self) -> Uuid { Font::TYPE_UUID }

    fn add_to_asset(&self,world: &mut World,res:Box<dyn AssetDynamic>) -> anyhow::Result<HandleUntyped>  {
        add_to_asset_type::<Font>(world, res)
    }

    fn sync_load(&self,_: &mut World,path: &str,server: &AssetServer,_:Option<Box<dyn AssetLoaderParams>>) -> anyhow::Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path)?;
        let bytes = std::fs::read(&full_path)?;
        let font = FontArc::try_from_vec(bytes)?;
        Ok(Box::new(Font { asset: font }))
    }

    async fn async_load(&self,server:AssetServer,path:SmolStr,
        _:Option<Box<dyn seija_asset::downcast_rs::DowncastSync>>,
        _:Option<Box<dyn AssetLoaderParams>>) -> anyhow::Result<Box<dyn AssetDynamic>> {
        let full_path = server.full_path(path.as_str())?;
        let bytes = smol::fs::read(&full_path).await?;
        let font = FontArc::try_from_vec(bytes)?;
        Ok(Box::new(Font { asset: font }))
    }
}
