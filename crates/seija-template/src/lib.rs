mod types;
mod loader;
mod inst;
mod creator;
use creator::{IFromTComponent, TComponentCreator};
use loader::TemplateLoader;
use seija_app::{IModule, App};
use seija_asset::AddAsset;
use seija_core::TypeUuid;
pub use types::{TComponent,TEntity,Template};
pub use loader::{read_tmpl_entity};
pub use inst::{instance_template_sync};

pub struct TemplateModule;

impl IModule for TemplateModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<Template>();
        app.add_asset_loader(Template::TYPE_UUID, TemplateLoader);
        app.init_resource::<creator::TComponentCreator>();
        app.add_t_component::<creator::TransformCreator>("Transform");
    }
}

pub trait AddTComponent {
    fn add_t_component<T>(&mut self,name:&str) where T:IFromTComponent + Default;
}

impl AddTComponent for App {
    fn add_t_component<T>(&mut self,name:&str) where T:IFromTComponent + Default {
        let mut data = self.world.get_resource_mut::<TComponentCreator>().unwrap();
        data.add(name,T::default());
    }
}
