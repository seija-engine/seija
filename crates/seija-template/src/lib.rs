mod types;
mod loader;
mod inst;
mod creator;
pub use creator::{TComponentManager,FromTComponentFunc};
use loader::TemplateLoader;
use seija_app::{IModule, App, ecs::world::World};
use seija_asset::AddAsset;
pub use types::{TComponent,TEntity,Template};
pub use loader::{read_tmpl_entity};
pub use inst::{instance_template_sync};

pub struct TemplateModule;

impl IModule for TemplateModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<Template>();
        app.add_asset_loader::<Template,TemplateLoader>();
        app.add_resource(creator::TComponentManager::new());
        app.add_t_component("Transform",creator::tcomponent_transform);
    }

    fn start(&self,world:&mut World) {
        let mut manager = world.get_resource_mut::<creator::TComponentManager>().unwrap();
        manager.start();
    }
}

pub trait AddTComponent {
    fn add_t_component(&mut self,name:&str,func:FromTComponentFunc);
}

impl AddTComponent for App {
    fn add_t_component(&mut self,name:&str,func:FromTComponentFunc) {
        let mut data = self.world.get_resource_mut::<creator::TComponentManager>().unwrap();
        data.add(name,func);
    }
}
