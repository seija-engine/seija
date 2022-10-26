mod pipeline_cache; 
pub mod render_bindings;
pub use pipeline_cache::{PipelineCache,PipelineKey,RenderPipelines};


/*
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
}*/