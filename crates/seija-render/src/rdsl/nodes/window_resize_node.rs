use std::{ptr::NonNull, ops::Deref};

use bevy_ecs::prelude::World;
use lite_clojure_eval::Variable;

use crate::{IUpdateNode, rdsl::atom::Atom, resource::RenderResourceId, RenderContext};
#[derive(Default)]
pub struct WindowReSizeNode {
    texture_ptr:Option<*mut Atom<RenderResourceId>>
}

impl IUpdateNode for WindowReSizeNode {
    fn update_params(&mut self,params:Vec<Variable>) {
        dbg!(&params);
       if let Some(ptr) = params[0].cast_userdata() {
            self.texture_ptr = Some(ptr as *mut Atom<RenderResourceId>);
       }
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        if let Some(ptr) = self.texture_ptr {
            let mut_atom = unsafe { &mut *ptr };
            
           
        }
    }
}

impl WindowReSizeNode {
    pub fn _update(&self) {
        
    }
}