use crate::component::TComponentManager;
use crate::errors::TemplateError;
use crate::reader::read_tmpl_entity;
use crate::types::{TEntityChildren, TemplateInner};
use crate::{TEntity, Template};
use relative_path::RelativePath;
use seija_app::ecs::world::World;
use seija_asset::add_to_asset_type;
use seija_asset::downcast_rs::DowncastSync;
use seija_asset::{async_trait::async_trait, IAssetLoader};
use seija_asset::{
    AssetDynamic, AssetLoaderParams, AssetRequest, AssetServer, AsyncLoadMode, HandleUntyped
};
use seija_core::anyhow::{anyhow, Result};
use seija_core::{smol, OptionExt};
use seija_core::TypeUuid;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub(crate) struct TemplateLoader;

#[async_trait]
impl IAssetLoader for TemplateLoader {
    fn typ(&self) -> seija_core::uuid::Uuid {
        Template::TYPE_UUID
    }

    fn add_to_asset(&self, world:&mut World, res:Box<dyn AssetDynamic>) -> Result<HandleUntyped> {
       add_to_asset_type::<Template>(world, res)
    }

    fn mode(&self) -> AsyncLoadMode {
        AsyncLoadMode::Perpare
    }

    fn sync_load(
        &self,
        world: &mut World,
        path: &str,
        server: &AssetServer,
        _: Option<Box<dyn AssetLoaderParams>>,
    ) -> Result<Box<dyn AssetDynamic>> {
        let file_path = RelativePath::new(path);
        let template_dir = file_path.parent().get()?;
    
        let mgr = world.get_resource::<TComponentManager>().unwrap().clone();
        let full_path = server.full_path(path)?;
        let xml_string = std::fs::read_to_string(full_path)?;
        let mut entity = read_tmpl_entity(&xml_string)?;
        let mut assets: Vec<HandleUntyped> = vec![];
      
        let childrens = load_dep_template_sync(world,&mut entity, &server, &mgr, &mut assets,template_dir)?;
        for (asset_typ, asset_path) in mgr.search_assets(&mut entity,template_dir)? {
            //log::error!("load_sync_untyped:{}",asset_path.as_str());
            let handle = server.load_sync_untyped(world,&asset_typ, asset_path.as_str(), None)?;
            assets.push(handle);
        }
        let inner = TemplateInner {
            assets,
            childrens,
            entity: Arc::new(entity),
        };

        let template = Template {
            inner: Arc::new(inner),
        };
        Ok(Box::new(template))
    }

    fn perpare(
        &self,
        world: &mut World,
        _: Option<Box<dyn DowncastSync>>,
    ) -> Option<Box<dyn DowncastSync>> {
        let mgr = world.get_resource::<TComponentManager>().unwrap().clone();
        Some(Box::new(mgr))
    }

    async fn async_load(
        &self,
        server: AssetServer,
        path: SmolStr,
        mut touch_data: Option<Box<dyn DowncastSync>>,
        _: Option<Box<dyn AssetLoaderParams>>,
    ) -> Result<Box<dyn AssetDynamic>> {
        if let Some(touch_data) = touch_data.take() {
            let mgr = touch_data
                .into_any()
                .downcast::<TComponentManager>()
                .map_err(|_| TemplateError::TypeCastError)?;

            let full_path = server.full_path(path.as_str())?;
            let path_buf = RelativePath::new(path.as_str());
            let template_dir = path_buf.parent().get()?;

            let xml_string = smol::fs::read_to_string(full_path).await?;
            let mut entity = read_tmpl_entity(&xml_string)?;

            let mut assets = vec![];
            let childrens = load_dep_template(&mut entity, &server, &mgr, &mut assets,template_dir).await?;
            for (asset_typ, asset_path) in mgr.search_assets(&mut entity,template_dir)? {
                let req = server.load_async_untyped(&asset_typ, asset_path.as_str(), None)?;
                let handle = req
                    .wait_handle()
                    .await
                    .ok_or(TemplateError::LoadAssetError)?;
                assets.push(handle);
            }

            let inner = TemplateInner {
                assets,
                childrens,
                entity: Arc::new(entity),
            };
            let template = Template {
                inner: Arc::new(inner),
            };
            return Ok(Box::new(template));
        }

        Err(anyhow!("TComponentManager"))
    }
}

async fn load_dep_template(
    tentiy: &mut TEntity,
    server: &AssetServer,
    mgr: &Box<TComponentManager>,
    all_assets: &mut Vec<HandleUntyped>,
    template_dir:&RelativePath
) -> Result<HashMap<SmolStr, HandleUntyped>> {
    let mut req_list: Vec<(AssetRequest, SmolStr)> = vec![];
    for children in tentiy.children.iter_mut() {
        if let TEntityChildren::Template(template) = children {
            let req = server.load_async::<Template>(template.res.as_str(), None)?;
            req_list.push((req, template.res.clone()));
            for comp in template.components.iter_mut() {
                let opt = mgr.get_opt(comp)?;
                for (asset_typ, asset_path) in opt.search_assets(comp,template_dir)? {
                    let req = server.load_async_untyped(&asset_typ, asset_path.as_str(), None)?;
                    let handle = req
                        .wait_handle()
                        .await
                        .ok_or(TemplateError::LoadAssetError)?;
                    all_assets.push(handle);
                }
            }
        }
    }

    let mut ret_map: HashMap<SmolStr, HandleUntyped> = HashMap::default();
    for (req, path) in req_list.drain(..) {
        let handle = req
            .wait_handle()
            .await
            .ok_or(TemplateError::LoadChildenError(path.clone()))?;
        ret_map.insert(path, handle);
    }
    Ok(ret_map)
}

fn load_dep_template_sync(
    world:&mut World,
    tentiy: &mut TEntity,
    server: &AssetServer,
    mgr: &TComponentManager,
    all_assets: &mut Vec<HandleUntyped>,
    template_dir:&RelativePath
) -> Result<HashMap<SmolStr, HandleUntyped>> {
    let mut ret_map:HashMap<SmolStr,HandleUntyped> = HashMap::default();
    for children in tentiy.children.iter_mut() {
        if let TEntityChildren::Template(template) = children {
            let res = server.load_sync::<Template>(world,template.res.as_str(), None)?;
            ret_map.insert(template.res.clone(), res.untyped());
            for comp in template.components.iter_mut() {
                let opt = mgr.get_opt(comp)?;
                for (asset_typ, asset_path) in opt.search_assets(comp,template_dir)? {
                    let cres = server.load_sync_untyped(world,&asset_typ, asset_path.as_str(), None)?;
                    all_assets.push(cres);
                }
            }
        }
    }
    Ok(ret_map)
}
