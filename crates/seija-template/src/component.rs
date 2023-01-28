use bevy_ecs::system::Resource;
use relative_path::RelativePath;
use seija_app::ecs::prelude::Entity;
use seija_app::ecs::system::{CommandQueue, Insert};
use seija_asset::AssetServer;
use seija_core::anyhow::{Result};
use seija_core::math::{Vec3, Quat, EulerRot};
use seija_core::uuid::Uuid;
use seija_transform::Transform;
use smol_str::SmolStr;
use std::{collections::HashMap, sync::Arc};

use crate::errors::TemplateError;
use crate::types::TEntityChildren;
use crate::{TComponent, TEntity};

#[derive(Clone,Resource)]
pub struct TComponentManager {
    opts: Arc<HashMap<SmolStr, Box<dyn ITComponentOpt>>>,
}

impl TComponentManager {
    pub fn new(opts: HashMap<SmolStr, Box<dyn ITComponentOpt>>) -> Self {
        TComponentManager {
            opts: Arc::new(opts),
        }
    }

    pub fn create<'w,'s,'a>(&self,t_component:&TComponent,server:&AssetServer,queue:&mut CommandQueue,entity:Entity) -> Result<()> {
        let opt = self.get_opt(t_component)?;
        opt.create_component(server, t_component, queue, entity)
    }

    pub fn get_opt(&self,tcomponent:&TComponent) -> Result<&Box<dyn ITComponentOpt>> {
        self
        .opts
        .get(tcomponent.typ.as_str())
        .ok_or(TemplateError::NotFoundOpt(tcomponent.typ.clone()).into())
    }

    pub fn search_assets(&self, entity: &mut TEntity,template_file_path:&RelativePath) -> Result<Vec<(Uuid, SmolStr)>> {
        let mut all_assets = vec![];
        self._search_assets(entity, &mut all_assets,template_file_path)?;
        Ok(all_assets)
    }

    fn _search_assets(&self,entity: &mut TEntity,all_assets:&mut Vec<(Uuid,SmolStr)>,template_dir:&RelativePath)  -> Result<()> {
        for tcomponent in entity.components.iter_mut() {
            let opt = self.get_opt(tcomponent)?;
            let mut assets = opt.search_assets(tcomponent,template_dir)?;
            all_assets.extend(assets.drain(..));
        }
        for centity in entity.children.iter_mut() {
             match centity {
                TEntityChildren::TEntity(e) => {
                    self._search_assets(e,all_assets,template_dir)?;
                }
                 _ => {},
             }
        }
        Ok(())
    }

}
pub trait ITComponentOpt: Send + Sync + 'static {
    fn search_assets(&self, _component: &mut TComponent,_template_dir:&RelativePath) -> Result<Vec<(Uuid,SmolStr)>> { Ok(vec![]) }
    fn create_component(&self,server:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()>;
}


pub(crate) struct TransformTemplateOpt;

impl ITComponentOpt for TransformTemplateOpt {
    fn search_assets(&self, _: &mut TComponent,_:&RelativePath) -> Result<Vec<(Uuid,SmolStr)>> {
       Ok(vec![])
    }

    fn create_component(&self,_:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity) -> Result<()> {
        let p = component.read_v3("position").unwrap_or(Vec3::ZERO);
        let r = component.read_v3("rotation").unwrap_or(Vec3::ZERO);
        let s = component.read_v3("scale").unwrap_or(Vec3::ONE);
        let rr = Quat::from_euler(EulerRot::YXZ, r.y.to_radians(), r.x.to_radians(), r.z.to_radians());
        let t = Transform::new(p, rr, s);
        let insert:Insert<Transform> = Insert {entity,bundle:t };
        queue.push(insert);
        Ok(())
    }
}