mod types;
mod loader;
mod inst;
mod component;
use std::collections::HashMap;
pub use component::{ITComponentOpt,TComponentManager};
use component::{TransformTemplateOpt};
use loader::TemplateLoader;
use seija_app::{IModule, App, ecs::world::World};
use seija_asset::AddAsset;
use smol_str::SmolStr;
pub mod errors;
mod reader;
pub use types::{TComponent,TEntity,Template};


pub struct TemplateModule;

impl IModule for TemplateModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<Template>();
        app.add_asset_loader::<Template,TemplateLoader>();
        app.add_resource(CacheTComponentOpts(HashMap::default()));
    
        app.add_tcomponent_opt("Transform", TransformTemplateOpt)
    }

    fn start(&self,world:&mut World) {
        let caches = world.remove_resource::<CacheTComponentOpts>().unwrap();
        let mgr = TComponentManager::new(caches.0);
        world.insert_resource(mgr);
    }
}

struct CacheTComponentOpts(HashMap<SmolStr,Box<dyn ITComponentOpt>>);

pub trait AddTComponent {
    fn add_tcomponent_opt(&mut self,name:&str,func:impl ITComponentOpt);
}

impl AddTComponent for App {
    fn add_tcomponent_opt(&mut self,name:&str,func:impl ITComponentOpt) {
        let mut data = self.world.get_resource_mut::<CacheTComponentOpts>().unwrap();
        data.0.insert(SmolStr::new(name), Box::new(func));
    }
}
