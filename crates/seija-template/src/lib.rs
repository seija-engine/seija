mod types;
mod loader;
mod inst;
mod creator;
pub use creator::{TComponentCreator,FromTComponentFunc};
use seija_app::{IModule, App};
use seija_asset::AddAsset;
pub use types::{TComponent,TEntity,Template};
pub use loader::{read_tmpl_entity};
pub use inst::{instance_template_sync};

pub struct TemplateModule;

impl IModule for TemplateModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<Template>();
        //TODO
        //app.add_asset_loader(Template::TYPE_UUID, TemplateLoader);
        app.init_resource::<creator::TComponentCreator>();
        app.add_t_component("Transform",creator::tcomponent_transform);
    }
}

pub trait AddTComponent {
    fn add_t_component(&mut self,name:&str,func:FromTComponentFunc);
}

impl AddTComponent for App {
    fn add_t_component(&mut self,name:&str,func:FromTComponentFunc) {
        let mut data = self.world.get_resource_mut::<creator::TComponentCreator>().unwrap();
        data.add(name,func);
    }
}
