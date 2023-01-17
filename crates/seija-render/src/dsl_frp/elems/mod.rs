use bevy_ecs::{world::World, prelude::Entity};
use anyhow::Result;
use lite_clojure_eval::{Variable, EvalRT};
use lite_clojure_frp::{DynamicID, FRPSystem};
use seija_asset::Assets;
use seija_core::OptionExt;
use smol_str::SmolStr;
use crate::{RenderContext, resource::{TextureDescInfo, Texture, RenderResourceId}, query::{IdOrName, QuerySystem}};
pub mod camera_node;
pub mod transform_node;
pub mod window_resize_node;
pub mod if_comp;
pub mod draw_pass_node;
pub mod posteffect_item;
pub mod ibl_node;
//pub mod post_stack;
pub mod post_stack_node;
use super::{frp_comp::IElement, system::ElementCreator};
pub struct UniformElement {
    name:String
}

impl UniformElement {
    pub fn new(name:String) -> Self {
        UniformElement { name }
    }
}

impl IElement for UniformElement {
   
    fn active(&mut self,_:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
       log::info!("UniformElem active:{}",self.name.as_str());
       ctx.ubo_ctx.add_uniform(&self.name, &mut ctx.resources);
       Ok(())
    }

    fn deactive(&mut self,_:&mut World,ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        ctx.ubo_ctx.remove_uniform(&self.name);
        Ok(())
    }

    
}

pub trait IUpdateNode {
    fn init(&mut self,_world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> { Ok(()) }
    fn active(&mut self,_world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> { Ok(()) }
    fn deactive(&mut self,_world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> { Ok(()) }
    fn update(&mut self,_world:&mut World,_ctx:&mut RenderContext,_:&mut FRPSystem) -> Result<()> { Ok(()) }
}

pub struct ElementNode {
    node:Box<dyn IUpdateNode>
}

impl ElementNode {
    pub fn new(node:Box<dyn IUpdateNode>) -> ElementNode {
        ElementNode { node }
    }
}

impl IElement for ElementNode {
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem,
            _vm:&mut EvalRT,_elem_creator:&ElementCreator) -> Result<()> {
        self.node.init(world, ctx,frp_sys)
    }
    
    fn active(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
       self.node.active(world, ctx,frp_sys)
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
      self.node.deactive(world, ctx,frp_sys)
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.node.update(world, ctx,frp_sys)
    }
    
}

pub struct TextureElement {
    desc_info:TextureDescInfo,
    //里面的值需要取出来释放掉，再替换
    dyn_texture:DynamicID
}

impl TextureElement {
    pub fn new(desc_info:TextureDescInfo,dyn_texture:DynamicID) -> Self {
        TextureElement { desc_info,dyn_texture }
    }
}

impl IElement for TextureElement {
    fn active(&mut self,world:&mut World,_:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
        let texture = Texture::create_by_desc(self.desc_info.clone());
        let h_texture = textures.add(texture);
        let res_id_ptr = Box::into_raw(Box::new(RenderResourceId::Texture(h_texture))) as *mut u8;
        let userdata = Variable::UserData(res_id_ptr);
        if let Some(dynamic) = frp_sys.dynamics.get_mut(&self.dyn_texture) {
            dynamic.set_value(userdata);
        }
        Ok(())
    }

    fn deactive(&mut self,_:&mut World,_:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        if let Some(dynamic) = frp_sys.dynamics.get_mut(&self.dyn_texture) {
            if let Some(dynamic_ptr) = dynamic.get_value().cast_userdata() {
                unsafe { let _ = Box::from_raw(dynamic_ptr as *mut RenderResourceId); };
            }
            dynamic.set_value(Variable::Nil);
        }
        Ok(())
    }
}

pub struct AddQueryElement {
   pub dynamic_id:DynamicID,
   pub query_name:IdOrName,
   pub query_type:u32
}

impl IElement for AddQueryElement {
    fn active(&mut self,world:&mut World,_:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        let mut query_system = world.get_resource_mut::<QuerySystem>().get()?;
        let query_index = query_system.add_query(self.query_name.clone(), self.query_type);
        if let Some(dynamic) = frp_sys.dynamics.get_mut(&self.dynamic_id) {
            dynamic.set_value(Variable::Int(query_index as i64));
        }
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,_:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()> {
        let mut query_system = world.get_resource_mut::<QuerySystem>().get()?;
        query_system.rmove_query(&self.query_name);
        Ok(())
    }
}


pub struct UniformSetElement {
    pub entity:Option<Entity>,
    pub uniform_name:SmolStr,
    pub prop_name:SmolStr,
    pub dynamic_id:DynamicID
}

impl IElement for UniformSetElement {
    fn active(&mut self,_:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        let dynamic = frp_sys.dynamics.get(&self.dynamic_id).get()?;
        let ptr = dynamic.get_value().cast_userdata().get()? as *mut RenderResourceId;
        let res_id = unsafe { &*ptr };
        match res_id {
            RenderResourceId::Texture(texture) => {
                ctx.ubo_ctx.set_texture(self.entity,&self.uniform_name, &self.prop_name, texture.clone())?;
            },
            _ => {}
        }
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext,_frp_sys:&mut FRPSystem) -> Result<()> {
        Ok(())
    }
}