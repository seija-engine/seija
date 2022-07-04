use std::{collections::HashMap, sync::Arc};

use bevy_ecs::prelude::{World, Entity, Added, With};
use lite_clojure_eval::{Variable, GcRefCell, EvalRT};
use seija_transform::Transform;


use crate::{camera::camera::Camera, UpdateNodeBox, ScriptContext};

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
      let mut path = RenderPath { nodes:vec![],env:GcRefCell::new(HashMap::default()),def };
      
      path
   }

   pub fn start(&mut self,vm:&mut EvalRT) {
      log::error!("start???");
      let nodes_mut = &mut self.nodes;
      let node_ptr = nodes_mut as *mut Vec<UpdateNodeBox> as *mut u8;
      self.env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":nodes".to_string())), 
                                   Variable::UserData(node_ptr));  
      if let Err(err) = vm.invoke_func2(&self.def.start_fn, vec![Variable::Map(self.env.clone()) ]) {
         log::error!("eval path :start error:{} {:?}",self.def.name.as_str(),err);
      }
   }
}

#[derive(Default)]
pub struct RenderPathList {
   pub path_dic:HashMap<String,Arc<RenderPathDef>>,
   list:Vec<RenderPath>
}

impl RenderPathList {
   pub fn update_camera(&mut self,world:&mut World,sc:&mut ScriptContext) {
      let mut added_cameras = world.query_filtered::<(Entity,&Camera),(Added<Camera>,With<Transform>)>();
      for (_,add_camera) in added_cameras.iter(world) {
         self.add_render_path(&add_camera.path,sc);
      }
   }

   fn add_render_path(&mut self,path:&String,sc:&mut ScriptContext) {
      if let Some(def) = self.path_dic.get(path) {
         let mut render_path = RenderPath::from_def(def.clone());
         self.list.push(render_path);
         //render_path.l.start(&mut sc.rt);
         self.list.last_mut().map(|v| v.start( &mut sc.rt));
      } else {
         log::error!("not found reder path:{}",path);
      }
   }
}