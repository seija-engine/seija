use std::{collections::HashMap, sync::Arc};
use bevy_ecs::prelude::{World};
use lite_clojure_eval::{Variable, GcRefCell, EvalRT};
use crate::{camera::camera::Camera, UpdateNodeBox, ScriptContext, resource::RenderResourceId, RenderContext};

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

   pub fn start(&mut self,vm:&mut EvalRT,camera:&Camera,world:&World,ctx:&mut RenderContext) {
      let nodes_mut = &mut self.nodes;
      let node_ptr = nodes_mut as *mut Vec<UpdateNodeBox> as *mut u8;
      self.env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":nodes".to_string())), 
                                   Variable::UserData(node_ptr));
      let resid = if let Some(texture) = camera.target.as_ref() {
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
  

   pub fn add_render_path(&mut self,path:&String,sc:&mut ScriptContext,camera:&Camera,world:&World,ctx:&mut RenderContext) {
      if let Some(def) = self.path_dic.get(path) {
         let mut render_path = RenderPath::from_def(def.clone());
         render_path.start(&mut sc.rt,camera,world,ctx);
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