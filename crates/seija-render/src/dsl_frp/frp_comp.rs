use bevy_ecs::world::World;
use anyhow::Result;
use crate::RenderContext;
use super::elems::{UniformElem, ElementNode};

pub trait IElement {
    fn init(&mut self,_world:&mut World,_ctx:&mut RenderContext) -> Result<()> { Ok(()) }
    fn active(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()>;
    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()>;
    fn update(&mut self,_world:&mut World,_ctx:&mut RenderContext) -> Result<()> { Ok(()) }
}

pub struct FRPComponent {
    name:String,
    elems:Vec<CompElement>
}

impl FRPComponent {
    pub fn new(name:String) -> Self {
        FRPComponent { 
            name,
            elems:vec![] 
        }
    }

    pub fn add_element(&mut self,element:CompElement) {
        self.elems.push(element);
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        for elem in self.elems.iter_mut() {
            match elem {
                CompElement::Node(node) => {
                   if let Err(err) = node.update(world, ctx) {
                      log::error!("node {} update error:{}",self.name.as_str(),&err);
                   }
                }
                _ => {}
            }
        }
    }
}

impl IElement for FRPComponent {
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        for elem in self.elems.iter_mut() {
            elem.opt_element_mut(world,ctx,|v,w,ctx| {
                if let Err(err) = v.init(w,ctx) {
                    log::error!("element init error:{:?}",&err);
                };
            });
        }
        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        for elem in self.elems.iter_mut() {
            elem.opt_element_mut(world,ctx,|v,w,ctx| {
                if let Err(err) = v.active(w,ctx) {
                    log::error!("element active error:{:?}",&err);
                };
            });
        }
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        for elem in self.elems.iter_mut() {
            elem.opt_element_mut(world,ctx,|v,w,ctx| {
                if let Err(err) = v.deactive(w,ctx) {
                    log::error!("element deactive error:{:?}",&err);
                };
            });
        }
        Ok(())
    }

    
}


pub enum CompElement {
    Unifrom(UniformElem),
    Component(FRPComponent),
    Node(ElementNode)
}

impl CompElement {
    pub fn opt_element_mut(&mut self,world:&mut World,ctx:&mut RenderContext,
                            f:fn(&mut dyn IElement,world:&mut World,&mut RenderContext)) {
        match self {
            CompElement::Unifrom(uniform) => {
                f(uniform,world,ctx)
            },
            CompElement::Component(frp) => {
                f(frp,world,ctx)
            },
            CompElement::Node(node) => {
                f(node,world,ctx);
            }
        }
    }


}

