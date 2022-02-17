use downcast::Any;

use downcast::{downcast};

mod camera;

pub trait IUniform :Any {
    
}

downcast!(dyn IUniform);

#[derive(Default)]
pub struct UniformContext {
    uniforms:Vec<Box<dyn IUniform>>
}

impl UniformContext {
    pub fn add<T:IUniform>(&mut self,value:T) {
        self.uniforms.push(Box::new(value));
    }
    pub fn get_value<T:'static>(&mut self,idx:usize) -> &mut T {
       let value = &mut self.uniforms[idx];
       let ret:Result<&mut T,_> = value.downcast_mut();
       ret.unwrap()
    }
}


