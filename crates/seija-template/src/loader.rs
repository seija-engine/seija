use std::collections::HashMap;
use std::sync::Arc;

use crate::component::TComponentManager;
use crate::errors::TemplateError;
use crate::reader::read_tmpl_entity;
use crate::types::{TEntityChildren, TemplateInner};
use crate::{Template, TEntity};

use seija_app::ecs::world::World;
use seija_asset::downcast_rs::DowncastSync;
use seija_asset::{async_trait::async_trait, IAssetLoader};
use seija_asset::{AssetDynamic, AssetLoaderParams, AssetServer, AsyncLoadMode, HandleUntyped, AssetRequest};
use seija_core::anyhow::{anyhow,Result};
use seija_core::smol;
use seija_core::TypeUuid;
use smol_str::SmolStr;

#[derive(Default)]
pub(crate) struct TemplateLoader;

#[async_trait]
impl IAssetLoader for TemplateLoader {
    fn typ(&self) -> seija_core::uuid::Uuid {
        Template::TYPE_UUID
    }

    fn mode(&self) -> AsyncLoadMode { AsyncLoadMode::Perpare }

    fn sync_load(&self,_: &mut World,_: &str,_: &AssetServer,_: Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        //let full_path = server.full_path(path)?;
        //let xml_string = std::fs::read_to_string(full_path)?;
        //let template = Template::from_str(&xml_string)?;

        
        //TODO 资源预加载
        //Ok(Box::new(template))
        todo!()
    }

    fn perpare(&self,world: &mut World,_: Option<Box<dyn DowncastSync>>) -> Option<Box<dyn DowncastSync>> {
        let mgr = world.get_resource::<TComponentManager>().unwrap().clone();
        Some(Box::new(mgr))
    }

    async fn async_load(&self,server: AssetServer,path: SmolStr,mut touch_data: Option<Box<dyn DowncastSync>>,_: Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
        if let Some(touch_data) = touch_data.take() {
            let mgr = touch_data
                .into_any()
                .downcast::<TComponentManager>()
                .map_err(|_| TemplateError::TypeCastError)?;

            let full_path = server.full_path(path.as_str())?;

            let xml_string = smol::fs::read_to_string(full_path).await?;
            let entity = read_tmpl_entity(&xml_string)?;
            

            let childrens = load_deps_template(&entity,&server).await?;
            let mut assets = vec![];
            for (asset_typ, asset_path) in mgr.search_assets(&entity)? {
                let req = server.load_async_untyped(&asset_typ, asset_path.as_str(), None)?;
                let handle = req.wait_handle().await.ok_or(TemplateError::LoadAssetError)?;
                assets.push(handle);
            }

            let inner = TemplateInner {assets,childrens,entity:Arc::new(entity) };
            let template = Template {inner:Arc::new(inner) };
            return Ok(Box::new(template));
        }

        Err(anyhow!("TComponentManager"))
    }
}

async fn load_deps_template(tentiy:&TEntity,server:&AssetServer) -> Result<HashMap<SmolStr,HandleUntyped>> {
    let mut req_list:Vec<(AssetRequest,SmolStr)> = vec![];
    for children in tentiy.children.iter() {
        if let TEntityChildren::Template(path) = children {
            let req = server.load_async::<Template>(path.as_str(), None)?;
            req_list.push((req,path.clone()));
        }
    }
    
    let mut ret_map:HashMap<SmolStr,HandleUntyped> = HashMap::default();
    for (req,path) in req_list.drain(..) {
        let handle = req.wait_handle().await.ok_or(TemplateError::LoadChildenError(path.clone()))?;
        ret_map.insert(path, handle);
    }
    Ok(ret_map)
}