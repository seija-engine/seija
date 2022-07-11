use std::marker::PhantomData;

use bevy_ecs::prelude::{World, Entity, With, Changed, Or, Component};
use fixedbitset::FixedBitSet;
use fnv::FnvHashMap;
use seija_core::LogOption;
use seija_transform::Transform;

use crate::{uniforms::backends::IShaderBackend, RenderContext, UniformBuffer, UniformIndex};

pub struct UBOArrayCollect<T:IShaderBackend,ET:'static + Send + Sync> {
    pub ubo_name:String,
    name_index:Option<UniformIndex>,
    backend:Option<T>,

    map_idxs:FnvHashMap<u32,usize>,
    list:Vec<Option<u32>>,
    cache_len:usize,
    max_size:usize,
    _mark:PhantomData<ET>
}

impl<T,ET> UBOArrayCollect<T,ET> where T:IShaderBackend,ET:Component {
    pub fn new(ubo_name:String,max_size:usize) -> Self {
        UBOArrayCollect { 
            ubo_name,
            name_index: None,
            backend: None,
            map_idxs: Default::default(),
            list: vec![],
            cache_len: 0,
            max_size,
            _mark:PhantomData
        }
    }

    pub fn init(&mut self,ctx:&mut RenderContext) {
        self.list = vec![None;self.max_size];
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            match T::from_def(&info.props) {
                Ok(backend) => {
                    self.backend = Some(backend)
                },
                Err(err) => {
                    log::error!("backend error :{}",err);
                }
            }
            self.name_index = Some(ctx.ubo_ctx.get_index(self.ubo_name.as_str()).unwrap())
        }
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext,setter:fn(&T,usize,&ET,&mut UniformBuffer,&Transform)) -> Option<()> {
        //add
        let mut frame_size = 0;
        let mut frame_eids:FixedBitSet = FixedBitSet::with_capacity(self.max_size);
        {
           let mut elems = world.query_filtered::<Entity,(With<ET>,With<Transform>)>();
           for e in elems.iter(world) {
               if !self.map_idxs.contains_key(&e.id()) {
                   self.add_element(e.id());
               }
               frame_eids.insert(e.id() as usize);
               frame_size += 1;
           }
       };
       //let type_ubo = self.name_index
       //                                            .and_then(|index| 
       //                                                       ctx.ubo_ctx.buffers.get_buffer_mut(&index, None)).log_err("get buffer error")?;
       let name_index = self.name_index.as_ref()?;
       let backend = self.backend.as_ref().log_err("get backend error")?;
        //update
        let mut elems = world.query_filtered::<(Entity,&ET,&Transform),Or<(Changed<ET>, Changed<Transform>)>>();
        for (e,elem,t) in elems.iter(world) {
            let index = *self.map_idxs.get(&e.id()).log_err("get index error")?;
            ctx.ubo_ctx.set_buffer(name_index, None, |buffer| {
                setter(backend,index,elem,&mut buffer.buffer,t);
            });
        }

        if self.cache_len != frame_size {
            ctx.ubo_ctx.set_buffer(name_index, None, |buffer| {
                backend.set_count(&mut buffer.buffer, frame_size as i32);
            });
           
            self.cache_len = frame_size;
        }
       Some(())
    }

    fn add_element(&mut self,eid:u32) {
        for idx in 0..self.list.len() {
            if self.list[idx].is_none() {
                self.list[idx] = Some(eid);
                self.map_idxs.insert(eid,idx);
                return;
            }
        }
    }
}