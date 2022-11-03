use crate::RenderContext;
use super::elems::UniformElem;

pub trait IElement {
    fn active(&mut self,ctx:&mut RenderContext);
    fn deactive(&mut self,ctx:&mut RenderContext);
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
}

impl IElement for FRPComponent {
    fn active(&mut self,ctx:&mut RenderContext) {
        for elem in self.elems.iter_mut() {
            elem.view_element_mut(ctx,|v,ctx| v.active(ctx));
        }
    }

    fn deactive(&mut self,ctx:&mut RenderContext) {
        for elem in self.elems.iter_mut() {
            elem.view_element_mut(ctx,|v,ctx| v.deactive(ctx));
        }
    }
}


pub enum CompElement {
    Unifrom(UniformElem),
    Component(FRPComponent)
}

impl CompElement {
    pub fn view_element_mut(&mut self,ctx:&mut RenderContext,f:fn(&mut dyn IElement,&mut RenderContext)) {
        match self {
            CompElement::Unifrom(uniform) => {
                f(uniform,ctx)
            },
            CompElement::Component(frp) => {
                f(frp,ctx)
            },
        }
    }


}

