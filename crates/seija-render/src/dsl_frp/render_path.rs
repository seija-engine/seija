use anyhow::{anyhow, Result};
use bevy_ecs::{
    prelude::Entity,
    query::{Added, With},
    world::World,
};
use lite_clojure_eval::{Variable, EvalRT, GcRefCell};
use lite_clojure_frp::FRPSystem;
use seija_core::OptionExt;
use seija_transform::Transform;
use smol_str::SmolStr;
use std::{collections::HashMap, sync::Arc};

use crate::{camera::camera::Camera, RenderContext, query::{QuerySystem, IdOrName}, resource::RenderResourceId, frp_context::{FRPContext, FRPContextInner}};

use super::{builder::FRPCompBuilder, frp_comp::{FRPComponent, IElement}, system::ElementCreator, RenderScriptPlugin, plugin::ApplyCameraType};

pub struct RenderPathDefine {
    pub name: SmolStr,
    pub start_func: Variable,
}

pub struct RenderPath {
    define:Arc<RenderPathDefine>,
    wait_init:bool,
    env:GcRefCell<HashMap<Variable,Variable>>,
    main_comp: Option<FRPComponent>,
    camera_entity:Entity
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
        let dynamic_id = frp_sys.new_dynamic(Variable::Int(query_index as i64) , frp_sys.never(), None).get()?;
        env_map.insert(camera_query_key, Variable::Int(dynamic_id as i64));

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
            main_comp:None,
            camera_entity:entity
        })
    }

    pub fn init(&mut self,world:&mut World,ctx:&mut RenderContext,vm:&mut EvalRT,
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
        match builder.build(creator,vm) {
            Ok(mut comp) => {
                if let Err(err) = comp.init(world, ctx, frp_sys, vm,creator) {
                    log::error!("comp init error:{:?}",err);
                }
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
    pub camera_events:Arc<Vec<(ApplyCameraType,SmolStr)>>,
    pub camera_dynamics:Arc<Vec<(ApplyCameraType,SmolStr,Variable)>>
}

impl RenderPathContext {
    pub fn add_define(&mut self, define: RenderPathDefine) {
        self.defines.insert(define.name.clone(), define.into());
    }

    pub fn apply_plugin(&mut self,plugin:&RenderScriptPlugin,_:Option<&FRPContext>) {
        self.camera_events = plugin.camera_events.clone();
        self.camera_dynamics = plugin.camera_dynamics.clone();
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext, creator: &ElementCreator,vm:&mut EvalRT,frp_ctx:&mut FRPContextInner) {
        //create new path
        let mut added_cameras = world.query_filtered::<(Entity, &Camera), (Added<Camera>, With<Transform>)>();
        for (entity, camera) in added_cameras.iter(world) {
            if let Some(define) = self.defines.get(camera.path.as_str()) {
                
                match RenderPath::create(entity,&world, camera,define.clone(),&mut frp_ctx.system) {
                    Ok(mut path) => { 
                        self.set_path_frp_vars(&mut path,frp_ctx);
                        self.path_list.push(path); 
                    },
                    Err(err) => { log::error!("{:?}",err); }
                  }
            } else {
                log::error!("not found render path:{}",camera.path.as_str());
            }
        }

        //update path
        for path in self.path_list.iter_mut() {
            if path.wait_init {
                if let Err(err) = path.init(world, ctx, vm, creator,&mut frp_ctx.system) {
                    log::error!("render path init error:{:?}",err);
                } else {
                    path.active(world, ctx,&mut frp_ctx.system);
                }
                path.wait_init = false;
            }
            if let Some(main_comp) = path.main_comp.as_mut() {
                main_comp.update(world, ctx,&mut frp_ctx.system);
            }
        }
    }

    fn set_path_frp_vars(&self,path:&mut RenderPath,ctx_inner:&mut FRPContextInner) {
        //camera_dynamics
        for (apply_type,dyn_name,default_var) in self.camera_dynamics.iter() {
            if let ApplyCameraType::Path(path_name) = apply_type {
                if path_name.as_str() != path.define.name.as_str() {
                    continue;
                }
            }
            let mut env_map_mut = path.env.borrow_mut();
            let dyn_id = ctx_inner.new_camera_dynamic(path.camera_entity, dyn_name.clone(), default_var.clone());
            let k = Variable::Keyword(GcRefCell::new(String::from(dyn_name.as_str())));
            env_map_mut.insert(k, Variable::Int(dyn_id as i64));
        }
        //camera_dynamics
        for (apply_type,event_name) in self.camera_events.iter() {
            if let ApplyCameraType::Path(path_name) = apply_type {
                if path_name.as_str() != path.define.name.as_str() {
                    continue;
                }
            }
            let mut env_map_mut = path.env.borrow_mut();
            let dyn_id = ctx_inner.new_camera_event(path.camera_entity, event_name.clone());
            let k = Variable::Keyword(GcRefCell::new(String::from(event_name.as_str())));
            env_map_mut.insert(k, Variable::Int(dyn_id as i64));
        }
    }
    
}
