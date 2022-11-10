use crate::{dsl_frp::frp_comp::IElement, RenderContext};
use anyhow::Result;
use lite_clojure_eval::Variable;
use lite_clojure_frp::{FRPSystem, DynamicID};
use seija_app::ecs::prelude::*;
pub struct IfCompElement {
    pub dynamic_id:DynamicID,
    pub true_comp_var:Variable,
    pub else_comp_var:Option<Variable>
}

impl IfCompElement {
    
}

impl IElement for IfCompElement {
    fn init(&mut self,_world:&mut World,_ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()> {
        
        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()> {
        
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()> {
        Ok(())
    }
}