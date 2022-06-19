use wgpu::CommandEncoder;

use crate::{UniformInfoSet, resource::RenderResources};

use super::{object::UniformObject, array_object::ArrayObject};
#[derive(Default)]
pub struct UniformContext2 {
    pub info:UniformInfoSet,

    //Name -> Object
    globals:Vec<UniformObject>,
    
    //Name + Entity -> Object
    components:Vec<ArrayObject>,
}

impl UniformContext2 {
    pub fn init(&mut self,res:&mut RenderResources) {
        //create global object
        for (_,info) in self.info.globals.iter() {
           let object = UniformObject::new(res, info);
           self.globals.push(object);
        }
        //create component object
        for (_,info) in self.info.components.iter() {
           self.components.push(ArrayObject::new(info,res));
        }
    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        for global in self.globals.iter_mut() {
            global.update(res,cmd);
        }
        for comps in self.components.iter_mut() {
            comps.update(res,cmd);
        }
    }
}