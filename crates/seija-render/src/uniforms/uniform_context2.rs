use wgpu::CommandEncoder;

use crate::{UniformInfoSet, resource::RenderResources};

use super::object::UniformObject;
#[derive(Default)]
pub struct UniformContext2 {
    pub info:UniformInfoSet,
    //Name -> Object
    globals:Vec<UniformObject>
    //Name + Entity -> Object
}

impl UniformContext2 {
    pub fn init(&mut self,res:&mut RenderResources) {
        //create global object
        for (_,info) in self.info.globals.iter() {
           let object = UniformObject::new(res, info);
           self.globals.push(object);
        }
    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        for global in self.globals.iter_mut() {
            global.update(res,cmd);
        }
    }
}