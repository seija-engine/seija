use std::{collections::{HashMap, VecDeque}, sync::Arc};
use bevy_ecs::world::World;
use seija_core::smol;

const MAX_LOADER:usize = 5usize;

pub trait ILoader: Send + Sync {
    fn id(&self) -> u64;
    fn prepare(&self);
}

enum AssetLoaderEvent {
    
}

struct LoaderContext {
    id:u64,
    loader:Arc<Box<dyn ILoader>>
}

#[derive(Default)]
pub struct AssetsLoader {
  pub(crate) queue_loaders:VecDeque<Arc<Box<dyn ILoader>>>,
  pub(crate) process_loaders:HashMap<u64,Arc<Box<dyn ILoader>>>,
}

impl AssetsLoader {
    pub fn new() -> Self {
        AssetsLoader { 
            queue_loaders: VecDeque::new(), 
            process_loaders: HashMap::default() 
        }
    }
}



impl AssetsLoader {
    pub fn add(&mut self,loader:impl ILoader + 'static + Send + Sync) {
        let boxed_loader = Box::new(loader);
        self.queue_loaders.push_back(Arc::new(boxed_loader));        
    }
}

pub(crate) fn update_assets_loader(world:&mut World) {
    let mut assets_loader = world.get_resource_mut::<AssetsLoader>().unwrap();
    if assets_loader.process_loaders.len() < MAX_LOADER {
        if let Some(loader) = assets_loader.queue_loaders.pop_front() {
            let clone_loader = loader.clone();
            assets_loader.process_loaders.insert(loader.id(), loader);
            smol::spawn(async move {
                clone_loader.prepare();
            }).detach();
        }
    }

}