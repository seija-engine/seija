use bevy_ecs::world::World;
use anyhow::Result;
use lite_clojure_eval::EvalRT;
use lite_clojure_frp::FRPSystem;
use crate::RenderContext;
use super::{system::ElementCreator};

pub trait IElement {
    fn init(&mut self,_world:&mut World,_ctx:&mut RenderContext,
            _frp_sys:&mut FRPSystem,_vm:&mut EvalRT,_elem_creator:&ElementCreator) -> Result<()> { Ok(()) }
    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()>;
    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()>;
    fn update(&mut self,_world:&mut World,_ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()> { Ok(()) }
}

pub struct FRPComponent {
    name:String,
    elems:Vec<Box<dyn IElement>>
}

impl FRPComponent {
    pub fn new(name:String) -> Self {
        FRPComponent { 
            name,
            elems:vec![] 
        }
    }

    pub fn add_element(&mut self,element:Box<dyn IElement>) {
        self.elems.push(element);
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) {
        for elem in self.elems.iter_mut() {
            elem.update(world, ctx, frp_sys);
        }
    }
}

impl IElement for FRPComponent {
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem,vm:&mut EvalRT,elem_creator:&ElementCreator) -> Result<()> {
        for elem in self.elems.iter_mut() {
            if let Err(err) = elem.init(world,ctx,frp_sys,vm,elem_creator) {
                log::error!("element init error:{:?}",&err);
            };
        }
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.update(world, ctx, frp_sys);

        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        for elem in self.elems.iter_mut() {
            if let Err(err) = elem.active(world,ctx,frp_sys) {
                log::error!("element active error:{:?}",&err);
            };
        }
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        for elem in self.elems.iter_mut() {
            if let Err(err) = elem.deactive(world,ctx,frp_sys) {
                log::error!("element deactive error:{:?}",&err);
            };
        }
        Ok(())
    }   
}

