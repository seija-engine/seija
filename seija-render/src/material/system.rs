use std::collections::{HashMap, HashSet};

use bevy_app::EventReader;
use bevy_asset::{AssetEvent, Assets, Handle};
use bevy_render::{draw::{DrawContext, OutsideFrustum}, mesh::{INDEX_BUFFER_ASSET_INDEX, Indices, Mesh, VERTEX_ATTRIBUTE_BUFFER_ID}, pipeline::{IndexFormat, PrimitiveTopology}, prelude::{Draw, RenderPipelines, Visible}, renderer::{BufferInfo, BufferUsage, RenderResourceBindings, RenderResourceContext, RenderResourceId}};
use bevy_ecs::prelude::*;

use crate::Material;

#[derive(Default)]
pub struct MeshEntities {
    entities: HashSet<Entity>,
}

#[derive(Default)]
pub struct MeshResourceProviderState {
    mesh_entities: HashMap<Handle<Mesh>, MeshEntities>,
}

pub fn update_mesh_to_material(
    mut state: Local<MeshResourceProviderState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    meshes: Res<Assets<Mesh>>,
    mut mesh_events: EventReader<AssetEvent<Mesh>>,
    mut querys:QuerySet<(
        Query<&mut Material,With<Handle<Mesh>>>,
        Query<(Entity,&Handle<Mesh>,&mut Material),Changed<Handle<Mesh>>>
    )>
) {

    let mut changed_meshes:HashSet<Handle<Mesh>> = HashSet::default();
    let render_resource_context = &**render_resource_context;
    for event in mesh_events.iter() {
        match event {
            AssetEvent::Created {ref handle } => {
                changed_meshes.insert(handle.clone_weak());
            }
            _ => todo!()
        }
    };

    for changed_mesh_handle in changed_meshes.iter() {
        if let Some(mesh) = meshes.get(changed_mesh_handle) {
            //创建顶点索引Buffer
            if let Some(data) = mesh.get_index_buffer_bytes() {
                let index_buffer = render_resource_context.create_buffer_with_data(
                    BufferInfo { buffer_usage: BufferUsage::INDEX, ..Default::default() },
                    &data,
                );
                let res_id = RenderResourceId::Buffer(index_buffer);
                render_resource_context.set_asset_resource(changed_mesh_handle,res_id,INDEX_BUFFER_ASSET_INDEX);
            }

            //创建顶点Buffer
            let interleaved_buffer = mesh.get_vertex_buffer_data();
            if !interleaved_buffer.is_empty() {
                let buffer = render_resource_context.create_buffer_with_data(
                    BufferInfo {
                        buffer_usage: BufferUsage::VERTEX,
                        ..Default::default()
                    },
                    &interleaved_buffer,
                );
                let res_id = RenderResourceId::Buffer(buffer);
                render_resource_context.set_asset_resource(changed_mesh_handle,res_id,VERTEX_ATTRIBUTE_BUFFER_ID);
            }

            
            if let Some(mesh_entities) = state.mesh_entities.get_mut(changed_mesh_handle) {
                for entity in mesh_entities.entities.iter() {
                    if let Ok(material) = querys.q0_mut().get_mut(*entity) {
                        update_entity_mesh(render_resource_context,mesh,changed_mesh_handle,material);
                    }
                }
            }
        }        
    }
    
    
    for (entity, handle, material) in querys.q1_mut().iter_mut() {
        let mesh_entities = state.mesh_entities.entry(handle.clone_weak()).or_insert_with(MeshEntities::default);
        mesh_entities.entities.insert(entity);
        if let Some(mesh) = meshes.get(handle) {
            update_entity_mesh(render_resource_context, mesh, handle, material);
        }
    }
}

//把Mesh的Buffer刷入材质球上的pipeline
fn update_entity_mesh(
    render_resource_context: &dyn RenderResourceContext,
    mesh: &Mesh,
    handle: &Handle<Mesh>,
    mut material: Mut<Material>,
) {
    for pipeline in material.pipes.pipelines.iter_mut() {
        pipeline.specialization.primitive_topology = mesh.primitive_topology();
        pipeline.specialization.vertex_buffer_layout = mesh.get_vertex_buffer_layout();
        if let PrimitiveTopology::LineStrip | PrimitiveTopology::TriangleStrip = mesh.primitive_topology() {
            pipeline.specialization.strip_index_format = mesh.indices().map(|indices| indices.into());
        }
    }

    let index_buffer = render_resource_context.get_asset_resource(handle, INDEX_BUFFER_ASSET_INDEX);
    if let Some(RenderResourceId::Buffer(buffer)) = index_buffer {
        let index_format: IndexFormat = mesh.indices().unwrap().into();
        material.pipes.bindings.set_index_buffer(buffer, index_format);
    }

    let vert_buffer = render_resource_context.get_asset_resource(handle, VERTEX_ATTRIBUTE_BUFFER_ID);
    if let Some(RenderResourceId::Buffer(buffer)) = vert_buffer {
        material.pipes.bindings.vertex_attribute_buffer = Some(buffer);
    }
}



pub fn draw_material(
    mut draw_context: DrawContext,
    meshes: Res<Assets<Mesh>>,
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    mut query: Query<
        (&mut Draw, &mut Material, &Handle<Mesh>, &Visible),
        Without<OutsideFrustum>,
    >
) {
    

    for (mut draw, mut material, mesh_handle, visible) in query.iter_mut() {
        if !visible.is_visible {
            continue;
        }
        let mesh = if let Some(mesh) = meshes.get(mesh_handle) {  mesh } else { continue; };

        let index_range = match mesh.indices() {
            Some(Indices::U32(indices)) => Some(0..indices.len() as u32),
            Some(Indices::U16(indices)) => Some(0..indices.len() as u32),
            None => None,
        };

        let pipes:&mut RenderPipelines = &mut material.pipes;
        for pipeline in pipes.pipelines.iter_mut() {
            pipeline.specialization.sample_count = 1;
            if pipeline.dynamic_bindings_generation != pipes.bindings.dynamic_bindings_generation() {
                pipeline.specialization.dynamic_bindings = pipes.bindings.iter_dynamic_bindings().map(|name| name.to_string()).collect();
                pipeline.dynamic_bindings_generation = pipes.bindings.dynamic_bindings_generation();
                for (handle, _) in pipes.bindings.iter_assets() {
                    if let Some(bindings) = draw_context.asset_render_resource_bindings.get_untyped(handle) {
                        for binding in bindings.iter_dynamic_bindings() {
                            pipeline.specialization.dynamic_bindings.insert(binding.to_string());
                        }
                    }
                }
            }
        }

        for pipeline in pipes.pipelines.iter_mut() {
            let render_resource_bindings = &mut [
                &mut pipes.bindings,
                &mut render_resource_bindings,
            ];
            draw_context.set_pipeline(&mut draw,&pipeline.pipeline,&pipeline.specialization).unwrap();

            draw_context.set_bind_groups_from_bindings(&mut draw, render_resource_bindings).unwrap();

            draw_context.set_vertex_buffers_from_bindings(&mut draw, &[&pipes.bindings]).unwrap();

            if let Some(indices) = index_range.clone() {
                draw.draw_indexed(indices, 0, 0..1);
            } else {
                draw.draw(0..mesh.count_vertices() as u32, 0..1)
            }
        }
    }
}