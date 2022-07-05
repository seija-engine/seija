use lite_clojure_eval::Variable;

use crate::{IUpdateNode, RenderContext};

#[derive(Default)]
pub struct DrawPassNode {
    
}

impl IUpdateNode for DrawPassNode {
    fn update_params(&mut self,params:Vec<Variable>) {
       
    }

    fn update(&mut self,world:&mut bevy_ecs::prelude::World,ctx:&mut RenderContext) {
        
    }
}