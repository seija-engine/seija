use crate::{dsl_frp::{frp_comp::{IElement, FRPComponent}, FRPDSLSystem, builder::FRPCompBuilder, system::ElementCreator, errors::Errors}, RenderContext};
use anyhow::{Result,anyhow};
use lite_clojure_eval::{Variable, EvalRT};
use lite_clojure_frp::{FRPSystem, DynamicID};
use seija_app::ecs::prelude::*;
pub struct IfCompElement {
    pub dynamic_id:DynamicID,
    true_comp_fn:(Variable,Vec<Variable>),
    else_comp_fn:Option<(Variable,Vec<Variable>)>,
    pub true_comp:Option<FRPComponent>,
    pub else_comp:Option<FRPComponent>,
    cur_bool_value:bool
}

impl IfCompElement {
    pub fn new(dynamic_id:DynamicID,true_comp_var:Variable,else_comp_var:Option<Variable>) -> Result<IfCompElement> {
        let true_comp_fn = Self::conv_fnvar(true_comp_var)?;
        let else_comp_fn = if let Some(var) = else_comp_var { 
            Some(Self::conv_fnvar(var)?)  
        } else { None };
        
        Ok(IfCompElement {
            dynamic_id,
            true_comp_fn,
            else_comp_fn,
            true_comp:None,
            else_comp:None,
            cur_bool_value:false
        })
    }

    fn conv_fnvar(var:Variable) -> Result<(Variable,Vec<Variable>)> {
        let array = var.cast_vec().ok_or(Errors::TypeCastError("vector"))?;
        let fn_name = array.borrow().first().ok_or(Errors::TypeCastError("func vector"))?.clone();
        let mut params:Vec<Variable> = vec![];
        for param in array.borrow().iter().skip(1) {
            params.push(param.clone());
        }
        Ok((fn_name,params))
    }
}

impl IElement for IfCompElement {
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem,vm:&mut EvalRT,creator:&ElementCreator) -> Result<()> {
        let mut builder = FRPCompBuilder::new();
        let builder_mut = &mut builder;
        FRPDSLSystem::set_global_vars(vm,builder_mut, world, frp_sys);
        vm.invoke_func2(&self.true_comp_fn.0 ,self.true_comp_fn.1.clone()).map_err(|err| anyhow!("eval err:{:?}",&err))?;
        let mut true_comp = builder.build(creator, vm)?;
        true_comp.init(world, ctx, frp_sys, vm, creator)?;
        let else_comp = if let Some((fn_name,params)) = self.else_comp_fn.take() {
            let mut builder = FRPCompBuilder::new();
            let builder_mut = &mut builder;
            FRPDSLSystem::set_global_vars(vm,builder_mut, world, frp_sys);
            vm.invoke_func2(&fn_name ,params).map_err(|err| anyhow!("eval err:{:?}",&err))?;
            let mut else_comp = builder.build(creator, vm)?;
            else_comp.init(world, ctx, frp_sys, vm, creator)?;
            Some(else_comp)
        } else { None };
       
        

        self.true_comp = Some(true_comp);
        self.else_comp = else_comp;
        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        let bool_value = frp_sys.dynamics.get(&self.dynamic_id)
                                               .ok_or(anyhow!("if-comp bool err"))?
                                               .get_value().cast_bool().unwrap_or(true);
        self.cur_bool_value = bool_value;
        if bool_value {
            if let Some(true_comp) = self.true_comp.as_mut() {
                true_comp.active(world, ctx, frp_sys)?;
            }
        } else {
            if let Some(else_comp) = self.else_comp.as_mut() {
                else_comp.active(world, ctx, frp_sys)?;
            }
        }
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        let bool_value = frp_sys.dynamics.get(&self.dynamic_id).ok_or(anyhow!("if-comp bool err"))?
                                               .get_value().cast_bool().unwrap_or(true);
        if self.cur_bool_value != bool_value {
            if self.cur_bool_value {
                if let Some(true_comp) = self.true_comp.as_mut() {
                    true_comp.deactive(world, ctx, frp_sys)?;
                }
                if let Some(else_comp) = self.else_comp.as_mut() {
                    else_comp.active(world, ctx, frp_sys)?;
                }
            } else {
                if let Some(true_comp) = self.true_comp.as_mut() {
                    true_comp.active(world, ctx, frp_sys)?;
                }
                if let Some(else_comp) = self.else_comp.as_mut() {
                    else_comp.deactive(world, ctx, frp_sys)?;
                }
            }
            self.cur_bool_value = bool_value;
        }
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        if self.cur_bool_value {
            if let Some(true_comp) = self.true_comp.as_mut() {
                true_comp.deactive(world, ctx, frp_sys)?;
            }
        } else {
            if let Some(else_comp) = self.else_comp.as_mut() {
                else_comp.deactive(world, ctx, frp_sys)?;
            }
        }
        Ok(())
    }
}