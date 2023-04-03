use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use bevy_ecs::{prelude::{Component, Entity}, system::{Query, Commands}};
use seija_asset::{HandleId, Assets, AssetServer};
use seija_core::math::Mat4;
use seija_render::{resource::{Mesh, MeshAttributeType, Indices}, wgpu::PrimitiveTopology, material::Material};
use seija_transform::{hierarchy::Children, Transform};

use crate::{render::UIRender2D, system::UIRenderRoot};
#[derive(Component,Default)]
pub struct Canvas {
    draw_calls:Vec<UIDrawCall>,
}


impl Canvas {
    pub fn update_drawcall(entity:Entity,
                           children:&Query<&Children>,
                           uirenders:&Query<&UIRender2D>,
                           canvases:&mut Query<&mut Canvas>,
                           meshes:&mut Assets<Mesh>,
                           materials:&mut Assets<Material>,
                           commands:&mut Commands,
                           ui_roots:&UIRenderRoot,
                           asset_server:&AssetServer,
                           parent_mat4:&Mat4) {
        let entity_group = ScanDrawCall::scan_entity_group(entity, children, uirenders, canvases);
        if let Ok(canvas) = canvases.get_mut(entity) {
            for (index,draw_entitys) in entity_group.iter().enumerate() {
                let mut hasher = DefaultHasher::default();
                draw_entitys.hash(&mut hasher);
                let hash_key = hasher.finish();
                if let Some(cur_drawcall) = canvas.draw_calls.get(index) {
                    if cur_drawcall.hash_key == hash_key {
                        continue;
                    }
                }
                commands.entity(entity).despawn();
                UIDrawCall::form_entity(draw_entitys,uirenders,meshes,materials,ui_roots,commands,asset_server,parent_mat4);
            }   
        }

    }
}


struct UIDrawCall {
    entity:Entity,
    hash_key:u64
}

impl UIDrawCall {
    pub fn form_entity(entitys:&Vec<Entity>,
                       render2ds:&Query<&UIRender2D>,
                       meshes:&mut Assets<Mesh>,
                       materials:&mut Assets<Material>,
                       ui_roots:&UIRenderRoot,
                       commands:&mut Commands,
                       asset_server:&AssetServer,
                       parent_mat4:&Mat4) {
        let mut positons:Vec<[f32;3]> = vec![];
        let mut uvs:Vec<[f32;2]> = vec![];
        let mut indexs:Vec<u32> = vec![];
        let mut index_offset = 0;
        let mut texture = None;
        for entity in entitys.iter() {
            if let Ok(render2d) = render2ds.get(*entity) {
                texture = Some(render2d.texture.clone());
                for vert in render2d.mesh2d.points.iter() {
                    positons.push(vert.pos.into());
                    uvs.push(vert.uv.into());
                }
                indexs.extend(render2d.mesh2d.indexs.iter().map(|v| v + index_offset));
                index_offset += render2d.mesh2d.points.len() as u32;
            }
        }


        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set(MeshAttributeType::POSITION, positons);
        mesh.set(MeshAttributeType::UV0, uvs);
        mesh.set_indices(Some(Indices::U32(indexs)));
        mesh.build();
        let h_mesh = meshes.add(mesh);
        let mut new_material = Material::from_def(ui_roots.baseui.clone(), asset_server).unwrap();
        new_material.texture_props.set("mainTexture", texture.unwrap().clone());
        let h_material = materials.add(new_material);
        let t = Transform::from_matrix(parent_mat4.clone());
        commands.spawn((h_mesh,h_material,t));
        
    }
}

#[derive(Component,Default)]
pub struct ZOrder {
    pub value:u32,
}

impl ZOrder {
    pub fn update(start:u32,entity:Entity,zorders:&mut Query<&mut ZOrder>,children:&Query<&Children>,commands:&mut Commands) {
        let mut z_index = start;
        if let Ok(child_comp) = children.get(entity) {
            for child_entity in child_comp.iter() {
                Self::_update(&mut z_index, *child_entity, zorders, children,commands);
            }
        }
    }

    fn _update(zindex:&mut u32,entity:Entity,zorders:&mut Query<&mut ZOrder>,children:&Query<&Children>,commands:&mut Commands) {
        if let Ok(mut zorder) = zorders.get_mut(entity) {
            zorder.value = *zindex;
            *zindex += 1;
        } else {
            commands.entity(entity).insert(ZOrder { value: *zindex });
            *zindex += 1;
        }

        if let Ok(child_comp) = children.get(entity) {
            for child_entity in child_comp.iter() {
                Self::_update(zindex, *child_entity, zorders, children,commands);
            }
        }
    }
}


struct ScanDrawCall {
    cur_texture:Option<HandleId>,
    entity_group:Vec<Vec<Entity>>,
    cache:Vec<Entity>,
}

impl ScanDrawCall {
    pub fn scan_entity_group(entity:Entity,children:&Query<&Children>,uirenders:&Query<&UIRender2D>,canvases:&Query<&mut Canvas>) -> Vec<Vec<Entity>> {
        let mut scan_drawcall = ScanDrawCall { entity_group:vec![],cache:vec![], cur_texture:None };
        if let Ok(render2d) = uirenders.get(entity) {
            scan_drawcall.cur_texture = Some(render2d.texture.id);
            scan_drawcall.cache.push(entity);
        }
        if let Ok(child_comp) = children.get(entity) {
            for child_entity in child_comp.iter() {
                scan_drawcall._scan_entity_group(*child_entity, children,uirenders,canvases);
            }
        }
        
        scan_drawcall.emit();
        scan_drawcall.entity_group
    }

    fn push(&mut self,entity:Entity) {
        self.cache.push(entity);
    }

    fn emit(&mut self) {
        if self.cache.len() > 0 {
            self.entity_group.push(self.cache.clone());
            self.cache.clear();
        }
    }

    fn _scan_entity_group(&mut self,entity:Entity,children:&Query<&Children>,uirenders:&Query<&UIRender2D>,canvases:&Query<&mut Canvas>) {
        if canvases.contains(entity) {
            self.emit();
            return;
        }
        if let Ok(render2d) = uirenders.get(entity) {
            match self.cur_texture {
                None => self.cur_texture = Some(render2d.texture.id),
                Some(id) => {
                    if id != render2d.texture.id {
                        self.emit();
                        self.cur_texture = Some(render2d.texture.id);
                    }
                }
            }
            self.push(entity);
        }
        if let Ok(child_comp) = children.get(entity) {
            for child_entity in child_comp.iter() {
                self._scan_entity_group(*child_entity, children,uirenders,canvases);
            }
        }
    }
}
