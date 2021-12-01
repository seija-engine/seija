use std::path::Path;
#[derive(Default)]
pub struct MaterialTask {
    
}


impl MaterialTask {
    pub fn run<P:AsRef<Path>>(&self,path:P) {
    }
}