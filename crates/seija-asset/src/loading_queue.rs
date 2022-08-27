use std::{sync::Arc};

use bevy_ecs::world::World;
use downcast_rs::DowncastSync;
use seija_core::smol::Task;
use seija_core::smol_str::SmolStr;
use seija_core::{smol,anyhow::Result};
use crate::asset::TypeLoader;
use crate::{AssetDynamic, HandleId};
use crate::{server::{AssetServer}};

pub(crate) struct LoadContext {
    hid:HandleId,
    uri:SmolStr,
    loader:Arc<TypeLoader>,
    pub(crate) touch_task:Option<Task<Box<dyn DowncastSync>>>,
    pub(crate) touch_data:Option<Box<dyn DowncastSync>>,
    pub(crate) load_task:Option<Task<Result<Box<dyn AssetDynamic>>>>,
    pub is_finish:bool,
}

impl LoadContext {
    pub fn new(uri:SmolStr,hid:HandleId,loader:Arc<TypeLoader>) -> Self {
        LoadContext {
            hid,
            uri,
            loader,
            touch_task:None,
            touch_data:None,
            load_task:None,
            is_finish:false
        }
    }
}
#[derive(Default)]
pub(crate) struct AssetLoadingQueue {
    loadings:Vec<LoadContext>,
}

impl AssetLoadingQueue {
    pub fn push_uri(&mut self,uri:SmolStr,hid:HandleId,loader:Arc<TypeLoader>,world:&mut World) {
        let server = world.get_resource::<AssetServer>().unwrap().clone();
        let mut load_context = LoadContext::new(uri,hid,loader.clone());
        if let Some(touch) = loader.async_touch {
            let touch_task = smol::spawn(touch(server));
            load_context.touch_task = Some(touch_task);
            self.loadings.push(load_context);
            return;
        }
        if let Some(prepare) = loader.perpare {
            load_context.touch_data = prepare(world,None);   
        }
        let load_fn = loader.async_load;
        let load_task = smol::spawn(load_fn(server,load_context.touch_data.as_mut()));
        load_context.load_task = Some(load_task);
        self.loadings.push(load_context);
    }


    pub fn update(&mut self,world:&mut World) {
        let mut count:i32 = self.loadings.len() as i32 - 1;
        while count >= 0 {
            let load_ctx = &mut self.loadings[count as usize];

            if  load_ctx.touch_task.as_ref().map(|v| v.is_finished()).unwrap_or(false) {
                let task = load_ctx.touch_task.take().unwrap();
                let mut touch_data = smol::block_on(task);
                if let Some(perpare) = load_ctx.loader.perpare {
                    perpare(world,Some(&mut touch_data));
                }
                load_ctx.touch_data = Some(touch_data);
                let server = world.get_resource::<AssetServer>().unwrap();
                let load_fn = load_ctx.loader.async_load;
                let load_task = smol::spawn(load_fn(server.clone(),load_ctx.touch_data.as_mut()));
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
                    },
                }
            }
            if load_ctx.is_finish {
                self.loadings.remove(count as usize);
            }
            count -= 1;
        }
    }
}