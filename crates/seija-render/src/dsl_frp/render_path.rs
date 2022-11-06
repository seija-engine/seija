use anyhow::{anyhow, Result};
use bevy_ecs::{
    prelude::Entity,
    query::{Added, With},
    world::World,
};
use lite_clojure_eval::{Variable, EvalRT, GcRefCell};
use seija_transform::Transform;
use smol_str::SmolStr;
use std::collections::HashMap;

use crate::{camera::camera::Camera, RenderContext, query::{QuerySystem, IdOrName}, resource::RenderResourceId};

use super::{builder::FRPCompBuilder, frp_comp::{FRPComponent, IElement}, system::ElementCreator};

pub struct RenderPathDefine {
    pub name: SmolStr,
    pub start_func: Variable,
}

pub struct RenderPath {
    wait_active:bool,
    camera_entity:Entity,
    env:GcRefCell<HashMap<Variable,Variable>>,
    main_comp: Option<FRPComponent>,
}

impl RenderPath {
    pub fn create(entity:Entity,define: &RenderPathDefine,
                  creator:&ElementCreator,world:&World,
                  camera:&Camera,vm:&mut EvalRT) -> Result<RenderPath> {
        
        let mut builder = FRPCompBuilder::new();
        let builder_ptr = &mut builder as *mut FRPCompBuilder as *mut u8;
        vm.global_context().set_var("*BUILDER*", Variable::UserData(builder_ptr));
        let mut env_map = HashMap::default();
        
        //:camera-id
        let camera_id_key = Variable::Keyword(GcRefCell::new(":camera-id".to_string()));
        env_map.insert(camera_id_key, Variable::Int(entity.to_bits() as i64));

        //:camera-query
        let query_system = world.get_resource::<QuerySystem>().unwrap();
        let query_id = IdOrName::Id(entity.to_bits());
        let query_index = query_system.get(query_id).unwrap();
        let camera_query_key = Variable::Keyword(GcRefCell::new(":camera-query".to_string()));
        env_map.insert(camera_query_key, Variable::Int(query_index as i64));

        //:path-target
        let res_id = if let Some(texture) = camera.target.as_ref() {
            Box::new(RenderResourceId::Texture(texture.clone_weak()))
        } else {
            Box::new(RenderResourceId::MainSwap)
        };
        let path_target_key = Variable::Keyword(GcRefCell::new(":path-target".to_string()));
        let res_id_ptr = Box::into_raw(res_id) as *mut u8;
        env_map.insert(path_target_key, Variable::UserData(res_id_ptr));
        
        let env_map_cell = GcRefCell::new(env_map);
        vm.invoke_func2(&define.start_func, vec![Variable::Map(env_map_cell.clone())]).map_err(|err| anyhow!("{:?}",err))?;
        match builder.build(creator) {
            Ok(main_comp) => {
               
                let path = RenderPath {
                    wait_active:true,
                    camera_entity:entity,
                    main_comp: Some(main_comp),
                    env:env_map_cell,
                };
                return Ok(path);
            }
            Err(err) => {
                return Err(anyhow!("build camera path {} comp error:{:?}",camera.path.as_str(),err));
            }
        }
    }

    pub fn active(&mut self,world:&mut World,ctx:&mut RenderContext) {
      if let Some(comp) = self.main_comp.as_mut() {
        if let Err(err) = comp.active(world, ctx) {
            log::error!("active camera comp error:{:?}",err);
        }
      }
    }

}

#[derive(Default)]
pub struct RenderPathContext {
    defines: HashMap<SmolStr, RenderPathDefine>,
    path_list: Vec<RenderPath>,
}

impl RenderPathContext {
    pub fn add_define(&mut self, define: RenderPathDefine) {
        self.defines.insert(define.name.clone(), define);
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext, creator: &ElementCreator,vm:&mut EvalRT) {
        let mut added_cameras =
            world.query_filtered::<(Entity, &Camera), (Added<Camera>, With<Transform>)>();
        for (entity, camera) in added_cameras.iter(world) {
            if let Some(define) = self.defines.get(camera.path.as_str()) {
               match RenderPath::create(entity,define, creator,&world, camera,vm) {
                 Ok(path) => {
                    self.path_list.push(path);
                 },
                 Err(err) => { log::error!("{:?}",err); }
               }
            } else {
                log::error!("not found render path:{}", camera.path.as_str());
            }
        }

        for path in self.path_list.iter_mut() {
            if path.wait_active {
                path.active(world, ctx);
                path.wait_active = false;
            }
            if let Some(main_comp) = path.main_comp.as_mut() {
                main_comp.update(world, ctx);
            }
        }
    }
}
