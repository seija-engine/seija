use bevy_ecs::world::World;
use lite_clojure_eval::Variable;
use crate::RenderContext;
use anyhow::Result;
use super::{super::errors::Errors, IUpdateNode};

pub struct CameraNode {
    ubo_name:String,
}

impl CameraNode {
    pub fn from_args(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let name = args.get(0)
                                          .and_then(Variable::cast_string)
                                          .ok_or(Errors::TypeCastError("string"))?;
        let br_names = name.borrow();
        Ok(Box::new(CameraNode { ubo_name:br_names.clone()  }))
    }
}

impl IUpdateNode for CameraNode {
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext) {
       
    }
}