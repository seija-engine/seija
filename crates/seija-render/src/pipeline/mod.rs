mod pipeline_cache; 
pub mod render_bindings;
use bevy_ecs::prelude::{ Query, Res, ResMut};
pub use pipeline_cache::{PipelineCache,PipelineKey,RenderPipelines};
use seija_asset::{Assets, Handle};

use crate::{material::{Material}, RenderContext, resource::Mesh};

pub fn update_pipeline_cache(mut pipeline_cache:ResMut<PipelineCache>,
                             ctx:Res<RenderContext>,
                             query:Query<(&Handle<Mesh>,&Handle<Material>)>,
                             meshs:Res<Assets<Mesh>>,materials:Res<Assets<Material>>) {
  
    
    

    for (mesh,material) in query.iter() {
        if let Some(mesh) = meshs.get(&mesh.id) {
            let mat = materials.get(&material.id).unwrap();
            pipeline_cache.update(mesh, &mat.def,&ctx);
        }
    }
}