mod pipeline_cache; 
pub mod render_bindings;
use bevy_ecs::prelude::{Mut, Query, Res, ResMut, World};
pub use pipeline_cache::{PipelineCache,PipelineKey,RenderPipelines};
use seija_asset::{Assets, Handle};

use crate::{material::{Material, MaterialStorage}, RenderContext, resource::Mesh};

pub fn update_pipeline_cache(mut pipeline_cache:ResMut<PipelineCache>,
                             ctx:Res<RenderContext>,
                             query:Query<(&Handle<Mesh>,&Handle<Material>)>,
                             meshs:Res<Assets<Mesh>>,materials:Res<MaterialStorage>) {
    let mats = materials.mateials.read();
    
    

    for (mesh,material) in query.iter() {
        let mesh = meshs.get(&mesh.id).unwrap();
        let mat = mats.get(&material.id).unwrap();
        pipeline_cache.update(mesh, &mat.def,&ctx);
    }
}