use bevy_ecs::world::World;
use anyhow::Result;
use lite_clojure_frp::FRPSystem;
use crate::RenderContext;
use super::elems::{UniformElement, ElementNode, TextureElement};

pub trait IElement {
    fn init(&mut self,_world:&mut World,_ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()> { Ok(()) }
    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()>;
    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()>;
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
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        for elem in self.elems.iter_mut() {
            elem.opt_element_mut(|v| {
                if let Err(err) = v.init(world,ctx,frp_sys) {
                    log::error!("element init error:{:?}",&err);
                };
            });
        }
        Ok(())
    }

    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        for elem in self.elems.iter_mut() {
            elem.opt_element_mut(|v| {
                if let Err(err) = v.active(world,ctx,frp_sys) {
                    log::error!("element active error:{:?}",&err);
                };
            });
        }
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        for elem in self.elems.iter_mut() {
            elem.opt_element_mut(|v| {
                if let Err(err) = v.deactive(world,ctx,frp_sys) {
                    log::error!("element deactive error:{:?}",&err);
                };
            });
        }
        Ok(())
    }

    
}


pub enum CompElement {
    Unifrom(UniformElement),
    Component(FRPComponent),
    Node(ElementNode),
    Texture(TextureElement)
}

impl CompElement {
    pub fn opt_element_mut<F>(&mut self,mut f:F) where F:FnMut(&mut dyn IElement) {
        match self {
            CompElement::Unifrom(uniform) => { f(uniform) },
            CompElement::Component(frp) => { f(frp) },
            CompElement::Node(node) => { f(node); },
            CompElement::Texture(texture) => { f(texture) }
        }
    }

}

