use bevy_ecs::prelude::{Added, Entity, Query, RemovedComponents, Res, ResMut};
use glam::Mat4;
use seija_asset::{Assets, Handle};
use seija_core::info::EInfo;
use seija_geometry::{bound::Relation, Frustum};
use seija_transform::Transform;

use crate::{camera::camera::Camera, material::Material, resource::Mesh};

use super::{system::IdOrName, view_list::ViewEntity, QuerySystem, ViewQuery, scene_octree_mgr::SceneOctreeMgr};

const CAMERA_TYPE: u32 = 1u32;

pub(crate) fn camera_query_check_add(
    mut system: ResMut<QuerySystem>,
    add_cameras: Query<Entity, Added<Camera>>,
    remove_cameras: RemovedComponents<Camera>,
) {
    for add_camera in add_cameras.iter() {
        let eid = add_camera.to_bits();

        system.add_query(IdOrName::Id(eid), CAMERA_TYPE);
    }
    for rm in remove_cameras.iter() {
        system.rmove_query(IdOrName::Id(rm.to_bits()));
    }
}

pub(crate) fn camera_query_update(
    system: Res<QuerySystem>,
    query: Query<(
        Entity,
        &Transform,
        &Handle<Material>,
        &Handle<Mesh>,
        Option<&EInfo>,
    )>,
    mats: Res<Assets<Material>>,
    meshs: Res<Assets<Mesh>>,
    cameras: Query<(&Camera, &Transform)>,
) {
    for view_query in system.querys.iter() {
        if view_query.typ == CAMERA_TYPE {
            update_camera_normal_query(view_query, &query, &mats, &cameras, &meshs);
        }
    }
}

fn update_camera_normal_query(
    view_query: &ViewQuery,
    query: &Query<(
        Entity,
        &Transform,
        &Handle<Material>,
        &Handle<Mesh>,
        Option<&EInfo>,
    )>,
    materials: &Assets<Material>,
    cameras: &Query<(&Camera, &Transform)>,
    meshs: &Assets<Mesh>,
) -> Option<()> {
    let mut write_list = view_query.list.write();
    write_list.clear();
    let id = Entity::from_bits(view_query.key.cast_id()?);
    let (camera, t) = cameras.get(id).ok()?;
    let camera_position = t.global().position;
    if camera.cull_type == -1 {
        for (entity, t, hmat, _, info) in query.iter() {
            if let Some(info) = info {
                if info.layer & camera.layer < 1 {
                    continue;
                }
            }
            let position = t.global().position;
            let dis_order = (camera_position - position).length_squared();
            let mat = materials.get(&hmat.id)?;
            write_list.add_entity(
                mat.order,
                ViewEntity {
                    entity,
                    order: dis_order,
                },
            );
        }
    } else if camera.cull_type == 0 {
        let proj_view = camera.projection.matrix() * t.global().matrix().inverse();
        if let Some(fru) = Frustum::from_matrix4(&proj_view) {
            for (entity, t, hmat, hmesh, info) in query.iter() {
                let position = t.global().position;
                let dis_order = (camera_position - position).length_squared();
                let mat = materials.get(&hmat.id)?;

                if let Some(info) = info {
                    if info.layer & camera.layer < 1 {
                        continue;
                    }
                }
                if let Some(aabb) = meshs.get(&hmesh.id).and_then(|v| v.aabb.as_ref()) {
                    let real_aabb = aabb.transform(&t.global().matrix());

                    if fru.contains(&real_aabb) != Relation::Out {
                        write_list.add_entity(
                            mat.order,
                            ViewEntity {
                                entity,
                                order: dis_order,
                            },
                        );
                    }
                } else {
                    write_list.add_entity(
                        mat.order,
                        ViewEntity {
                            entity,
                            order: dis_order,
                        },
                    );
                }
            }
        } else {
            log::error!("frustum::from_matrix4 error");
        }
    }
    write_list.sort();
    Some(())
}

pub(crate) fn update_camera_octree_query(
    mut octree_mgr:ResMut<SceneOctreeMgr>,
    system: Res<QuerySystem>,
    query: Query<(
        Entity,
        &Transform,
        &Handle<Material>,
        &Handle<Mesh>,
        Option<&EInfo>,
    )>,
    mats: Res<Assets<Material>>,
    meshs: Res<Assets<Mesh>>,
    cameras: Query<(&Camera, &Transform)>,
) {
    for view_query in system.querys.iter() {
        if view_query.typ != CAMERA_TYPE { continue; }
        if let Some(id) = view_query.key.cast_id() {
            let id = Entity::from_bits(id);
            if let Ok((camera,t)) = cameras.get(id) {
                if camera.cull_type == 1 {
                    let proj_view = camera.projection.matrix() * t.global().matrix().inverse();
                    update_camera_octree_query_(&mut octree_mgr,view_query,&query,&mats,&cameras,&meshs,proj_view,id);
                }
            }
        }
    }
}

fn update_camera_octree_query_(
    octree_mgr:&mut SceneOctreeMgr,
    view_query: &ViewQuery,
    query: &Query<(Entity,&Transform,&Handle<Material>,&Handle<Mesh>,Option<&EInfo>)>,
    materials: &Assets<Material>,
    cameras: &Query<(&Camera, &Transform)>,
    meshs: &Assets<Mesh>,
    proj_view:Mat4,
    camera_id:Entity
) -> Option<()> {
    let fur_aabb = Frustum::create_aabb(&proj_view);
    let node_id = if octree_mgr.has(camera_id.id()) {
        octree_mgr.update(camera_id, Some(fur_aabb))
    } else {
       Some(octree_mgr.add(camera_id, Some(fur_aabb)))
    };
    let node_id = node_id?;

    Some(())
}
