use std::{collections::HashMap, sync::Arc};
use bevy_ecs::prelude::{World, Entity};
use lite_clojure_eval::{Variable, GcRefCell, EvalRT};
use seija_asset::Handle;
use crate::{UpdateNodeBox, ScriptContext, resource::{RenderResourceId, Texture}, RenderContext, query::{QuerySystem, IdOrName}};

use super::atom::Atom;

pub struct RenderPathDef {
   pub name:String,
   pub start_fn:Variable,
}


pub struct RenderPath {
   def:Arc<RenderPathDef>,
   pub env:GcRefCell<HashMap<Variable,Variable>>,
   nodes:Vec<UpdateNodeBox>
}

impl RenderPath {
   pub fn from_def(def:Arc<RenderPathDef>) -> Self {
      RenderPath { nodes:vec![],env:GcRefCell::new(HashMap::default()),def }
   }

   pub fn start(&mut self,vm:&mut EvalRT,world:&mut World,ctx:&mut RenderContext,e:Entity,target:Option<Handle<Texture>>) {
      {
         let query_system = world.get_resource::<QuerySystem>().unwrap();
         let query_id = IdOrName::Id(e.to_bits());
         let query_index = query_system.get(query_id).unwrap();
         self.env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":camera-query".to_string())), 
                                      Variable::Int(query_index as i64));
         self.env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":camera-id".to_string())), 
                                      Variable::Int(e.to_bits() as i64));
      };
      let nodes_mut = &mut self.nodes;
      let node_ptr = nodes_mut as *mut Vec<UpdateNodeBox> as *mut u8;
      self.env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":nodes".to_string())), 
                                   Variable::UserData(node_ptr));
      let resid = if let Some(texture) = target.as_ref() {
         Box::new(Atom::new(RenderResourceId::Texture(texture.clone_weak())))
      } else {
         Box::new(Atom::new(RenderResourceId::MainSwap))
      };
      let res_ptr = Box::into_raw(resid) as *mut Atom<RenderResourceId> as *mut u8;
      self.env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":targetView".to_string())), 
                                   Variable::UserData(res_ptr));
      if let Err(err) = vm.invoke_func2(&self.def.start_fn, vec![Variable::Map(self.env.clone()) ]) {
         log::error!("eval path :start error:{} {:?}",self.def.name.as_str(),err);
      }

      for node in self.nodes.iter_mut() {
         node.set_params(vm, true);
         node.init(world, ctx);
      }
   }

   pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
      for node in self.nodes.iter_mut() {
         node.prepare(world, ctx);
      }
      for node in self.nodes.iter_mut() {
         node.update(world, ctx);
      }
   }
}

#[derive(Default)]
pub struct RenderPathList {
   pub path_dic:HashMap<String,Arc<RenderPathDef>>,
   list:Vec<RenderPath>
}

impl RenderPathList {
  
   pub fn add_render_path(&mut self,
                          path:&str,
                          sc:&mut ScriptContext,
                          target:Option<Handle<Texture>>,
                          world:&mut World,
                          ctx:&mut RenderContext,
                          e:Entity) {
   
      if let Some(def) = self.path_dic.get(path) {
         let mut render_path = RenderPath::from_def(def.clone());
         render_path.start(&mut sc.rt,world,ctx,e,target);
         self.list.push(render_path);
      } else {
         log::error!("not found reder path:{}",path);
      }
   }

   pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
      for path in self.list.iter_mut() {
         path.update(world, ctx);
      }
   }

}