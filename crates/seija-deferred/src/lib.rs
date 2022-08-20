mod render_plugin;
//mod deferred_light_pass;
use anyhow::{Result};
use bevy_ecs::prelude::{World, Entity};
pub use render_plugin::{create_deferred_plugin};
use seija_app::IModule;

pub struct DeferredQuad(pub Entity);

pub struct DeferredRenderModule {
    pub mat_path:String
}

impl IModule for DeferredRenderModule {
    fn init(&mut self,_app:&mut seija_app::App) {}

    fn start(&self, _world:&mut World) {
       /*
       match self.load_quad(world) {
           Ok(e) => {
              
               world.insert_resource(DeferredQuad(e));
           },
           Err(err) => {
            log::error!("{}",err);
           }
       }*/
    }
}

impl DeferredRenderModule {
    pub fn load_quad(&self,_:&mut World) -> Result<Entity> {
        /* 
        let mats = world.get_resource::<MaterialStorage>().ok_or(RenderErrors::NotFoundMaterialStorage)?;
        let mat_string = std::fs::read_to_string(self.mat_path.as_str())?;
        mats.load_material_def(mat_string.as_str());
        
        let h_mat = mats.create_material("DeferredLightPass")
                                       .ok_or(anyhow!("create deferred mat error"))?;
        let quad_mesh:Mesh = Quad::new(2f32).into();

        let mut meshs = world.get_resource_mut::<Assets<Mesh>>()
                                             .ok_or(RenderErrors::NotFoundAssetsMesh)?;
        let h_quad = meshs.add(quad_mesh);
        let t = Transform::default();
        let mut commands = world.spawn();
        commands.insert(t);
        commands.insert(h_quad);
        commands.insert(h_mat);
        Ok(commands.id())*/
        todo!()
    }

}