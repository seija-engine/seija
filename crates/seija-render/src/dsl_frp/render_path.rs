use anyhow::{anyhow, Result};
use bevy_ecs::{
    prelude::Entity,
    query::{Added, With},
    world::World,
};
use lite_clojure_eval::{Variable, EvalRT, GcRefCell};
use lite_clojure_frp::FRPSystem;
use seija_transform::Transform;
use smol_str::SmolStr;
use std::{collections::HashMap, sync::Arc};

use crate::{camera::camera::Camera, RenderContext, query::{QuerySystem, IdOrName}, resource::RenderResourceId};

use super::{builder::FRPCompBuilder, frp_comp::{FRPComponent, IElement}, system::ElementCreator};

pub struct RenderPathDefine {
    pub name: SmolStr,
    pub start_func: Variable,
}

pub struct RenderPath {
    define:Arc<RenderPathDefine>,
    wait_init:bool,
    env:GcRefCell<HashMap<Variable,Variable>>,
    main_comp: Option<FRPComponent>,
}

impl RenderPath {
    pub fn create(entity:Entity,world:&World,camera:&Camera,define:Arc<RenderPathDefine>,frp_sys:&mut FRPSystem) -> Result<RenderPath> {
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
        let camera_target_event = frp_sys.new_event(None);
        let path_target_key = Variable::Keyword(GcRefCell::new(":path-target".to_string()));
        let res_id_ptr = Box::into_raw(res_id) as *mut u8;
        let dyn_target_texture = frp_sys.new_dynamic(Variable::UserData(res_id_ptr), camera_target_event, None).unwrap();
        env_map.insert(path_target_key, Variable::Int(dyn_target_texture as i64));
        
        
        Ok(RenderPath {
            define,
            wait_init:true,
            env:GcRefCell::new(env_map),
            main_comp:None
        })
    }

    pub fn init(&mut self,world:&mut World,_ctx:&mut RenderContext,vm:&mut EvalRT,
                creator:&ElementCreator,frp_sys:&mut FRPSystem) -> Result<()> {
        let mut builder = FRPCompBuilder::new();
        let builder_ptr = &mut builder as *mut FRPCompBuilder as *mut u8;
        vm.global_context().set_var("*BUILDER*", Variable::UserData(builder_ptr));
        let env_map_cell = self.env.clone();
        let world_ptr = world as *mut World as *mut u8;
        vm.global_context().set_var("*WORLD*", Variable::UserData(world_ptr));
        let frp_ptr = frp_sys as *mut FRPSystem as *mut u8;
        vm.global_context().set_var("*FRPSystem*", Variable::UserData(frp_ptr));
        vm.invoke_func2(&self.define.start_func, vec![Variable::Map(env_map_cell)]).map_err(|err| anyhow!("{:?}",err))?;
        match builder.build(creator) {
            Ok(comp) => {
                self.main_comp = Some(comp);
            },
            Err(err) => { return Err(anyhow!("build camera path {} comp error:{:?}",self.define.name.as_str(),err)); }
        }
        Ok(())
    }

    pub fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) {
      if let Some(comp) = self.main_comp.as_mut() {
        if let Err(err) = comp.active(world, ctx,frp_sys) {
            log::error!("active camera comp error:{:?}",err);
        }
      }
    }

}

#[derive(Default)]
pub struct RenderPathContext {
    defines: HashMap<SmolStr, Arc<RenderPathDefine>>,
    path_list: Vec<RenderPath>,
}

impl RenderPathContext {
    pub fn add_define(&mut self, define: RenderPathDefine) {
        self.defines.insert(define.name.clone(), define.into());
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext, creator: &ElementCreator,vm:&mut EvalRT,frp_sys:&mut FRPSystem) {
        //create new path
        let mut added_cameras = world.query_filtered::<(Entity, &Camera), (Added<Camera>, With<Transform>)>();
        for (entity, camera) in added_cameras.iter(world) {
            if let Some(define) = self.defines.get(camera.path.as_str()) {
                
                match RenderPath::create(entity,&world, camera,define.clone(),frp_sys) {
                    Ok(path) => { self.path_list.push(path); },
                    Err(err) => { log::error!("{:?}",err); }
                  }
            } else {
                log::error!("not found render path:{}",camera.path.as_str());
            }
        }

        //update path
        for path in self.path_list.iter_mut() {
            if path.wait_init {
                if let Err(err) = path.init(world, ctx, vm, creator,frp_sys) {
                    log::error!("render path init error:{:?}",err);
                } else {
                    path.active(world, ctx,frp_sys);
                }
                path.wait_init = false;
            }
            if let Some(main_comp) = path.main_comp.as_mut() {
                main_comp.update(world, ctx,frp_sys);
            }
        }
    }
}
