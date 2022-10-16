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

#[derive(Clone)]
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

    pub fn search_assets(&self, entity: &TEntity) -> Result<Vec<(Uuid, SmolStr)>> {
        let mut all_assets = vec![];
        self._search_assets(entity, &mut all_assets)?;
        Ok(all_assets)
    }

    fn _search_assets(&self,entity: &TEntity,all_assets:&mut Vec<(Uuid,SmolStr)>)  -> Result<()> {
        for tcomponent in entity.components.iter() {
            let opt = self.get_opt(tcomponent)?;
            let mut assets = opt.search_assets(tcomponent)?;
            all_assets.extend(assets.drain(..));
        }
        for centity in entity.children.iter() {
             match centity {
                TEntityChildren::TEntity(e) => {
                    self._search_assets(&e,all_assets)?;
                }
                 _ => {},
             }
        }
        Ok(())
    }

}
pub trait ITComponentOpt: Send + Sync + 'static {
    fn search_assets(&self, _component: &TComponent) -> Result<Vec<(Uuid,SmolStr)>> { Ok(vec![]) }
    fn create_component(&self,server:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity)-> Result<()>;
}


pub(crate) struct TransformTemplateOpt;

impl ITComponentOpt for TransformTemplateOpt {
    fn search_assets(&self, _: &TComponent) -> Result<Vec<(Uuid,SmolStr)>> {
       Ok(vec![])
    }

    fn create_component(&self,_:&AssetServer, component: &TComponent,queue:&mut CommandQueue,entity:Entity) -> Result<()> {
        let p = component.read_v3("position").unwrap_or(Vec3::ZERO);
        let r = component.read_v3("rotation").unwrap_or(Vec3::ZERO);
        let s = component.read_v3("scale").unwrap_or(Vec3::ONE);
        let rr = Quat::from_euler(EulerRot::YXZ, r.y.to_radians(), r.x.to_radians(), r.z.to_radians());
        let t = Transform::new(p, rr, s);
        let insert = Insert {entity,component:t };
        queue.push(insert);
        Ok(())
    }
}