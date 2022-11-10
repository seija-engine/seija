use bevy_ecs::world::World;
use anyhow::Result;
use lite_clojure_eval::Variable;
use lite_clojure_frp::{DynamicID, FRPSystem};
use seija_asset::Assets;
use crate::{RenderContext, resource::{TextureDescInfo, Texture, RenderResourceId}};
pub mod camera_node;
pub mod transform_node;
pub mod window_resize_node;
pub mod if_comp;
use super::frp_comp::IElement;
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
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
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