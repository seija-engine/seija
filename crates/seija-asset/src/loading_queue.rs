use std::{sync::Arc};

use bevy_ecs::world::World;
use downcast_rs::DowncastSync;
use seija_core::smol::Task;
use seija_core::smol_str::SmolStr;
use seija_core::{smol,anyhow::Result};
use crate::asset::AssetLoader;
use crate::{AssetDynamic, HandleId, AssetLoaderParams};
use crate::{server::{AssetServer}};

pub(crate) struct LoadContext {
    hid:HandleId,
    uri:SmolStr,
    loader:Arc<AssetLoader>,
    pub(crate) touch_task:Option<Task<Result<Box<dyn DowncastSync>>>>,
    pub(crate) touch_data:Option<Box<dyn DowncastSync>>,
    pub(crate) load_task:Option<Task<Result<Box<dyn AssetDynamic>>>>,
    pub is_finish:bool,
    pub is_fail:bool,
    pub params:Option<Box<dyn AssetLoaderParams>>
}

impl LoadContext {
    pub fn new(uri:SmolStr,hid:HandleId,loader:Arc<AssetLoader>,params:Option<Box<dyn AssetLoaderParams>>) -> Self {
        LoadContext {
            hid,
            uri,
            loader,
            touch_task:None,
            touch_data:None,
            load_task:None,
            is_finish:false,
            is_fail:false,
            params
        }
    }
}
#[derive(Default)]
pub(crate) struct AssetLoadingQueue {
    loadings:Vec<LoadContext>,
}

impl AssetLoadingQueue {
    pub fn push_uri(&mut self,uri:SmolStr,hid:HandleId,loader:Arc<AssetLoader>,world:&mut World,params:Option<Box<dyn AssetLoaderParams>>) {
        let server = world.get_resource::<AssetServer>().unwrap().clone();
        let mut load_context = LoadContext::new(uri,hid,loader.clone(),params);
        if let Some(touch) = loader.async_touch {
            let touch_task = smol::spawn(touch(server,load_context.uri.clone()));
            
            load_context.touch_task = Some(touch_task);
            self.loadings.push(load_context);
            return;
        }
        if let Some(prepare) = loader.perpare {
            load_context.touch_data = prepare(world,None);   
        }
        let load_fn = loader.async_load;
        let params = load_context.params.take();
        let load_task = smol::spawn(load_fn(server,load_context.uri.clone()
                                                                                   ,load_context.touch_data.take(),params));
        load_context.load_task = Some(load_task);
        self.loadings.push(load_context);
    }


    pub fn update(&mut self,world:&mut World) {
        let mut count:i32 = self.loadings.len() as i32 - 1;
        while count >= 0 {
            let load_ctx = &mut self.loadings[count as usize];

            if  load_ctx.touch_task.as_ref().map(|v| v.is_finished()).unwrap_or(false) {
                let task = load_ctx.touch_task.take().unwrap();
                match smol::block_on(task) {
                    Ok(touch_data) => {
                        if let Some(perpare) = load_ctx.loader.perpare {
                            load_ctx.touch_data = perpare(world,Some(touch_data));
                        } else {
                            load_ctx.touch_data = Some(touch_data);
                        }
                        
                    },
                    Err(err) => {
                        log::error!("async touch error:{:?}",err);
                        load_ctx.is_fail = true;
                        continue;
                    },
                }
               
                
                let server = world.get_resource::<AssetServer>().unwrap();
                let load_fn = load_ctx.loader.async_load;
                let params = load_ctx.params.take();
                let load_task = smol::spawn(load_fn(server.clone(),load_ctx.uri.clone(),load_ctx.touch_data.take(),params));
                load_ctx.load_task = Some(load_task);
                continue;
            }

            if  load_ctx.load_task.as_ref().map(|v| v.is_finished()).unwrap_or(false) {
                let task = load_ctx.load_task.take().unwrap();
                let asset_data = smol::block_on(task);
                match asset_data {
                    Ok(dyn_asset) => {
                        if let Some(server) = world.get_resource::<AssetServer>() {
                            server.add_dyn_asset(&load_ctx.uri,&load_ctx.loader.typ,load_ctx.hid, dyn_asset);
                        }
                        load_ctx.is_finish = true;
                    },
                    Err(err) => {
                        log::error!("load asset error:{:?}",err);
                        load_ctx.is_fail = true;
                    },
                }
            }
            if load_ctx.is_finish || load_ctx.is_fail {
                if load_ctx.is_fail {
                   let server = world.get_resource::<AssetServer>().unwrap();
                   if let Some(info) = server.get_asset(&load_ctx.uri) {
                        info.set_fail();
                   }
                }
                self.loadings.remove(count as usize);
                
            }
            count -= 1;
        }
    }
}