use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, sync::Arc};

use bevy_ecs::{prelude::{Component, Entity}, system::{Query, Commands}};
use seija_asset::{HandleId, Assets, AssetServer, Handle};
use seija_core::{math::{Mat4, Vec4, Vec4Swizzles}, info::EStateInfo};
use seija_render::{resource::{Mesh, MeshAttributeType, Indices}, material::{Material, MaterialDef}};
use seija_transform::{hierarchy::{Children, Parent}, Transform};
use crate::{render::UIRender2D, system::UIRenderRoot};
use  wgpu::PrimitiveTopology;
pub const Z_SCALE: f32 = 0.00001;

#[derive(Component,Default)]
pub struct Canvas {
    pub(crate) draw_calls:Vec<UIDrawCall>,
    pub is_clip:bool,
}

impl Canvas {
    pub fn new(is_clip:bool) -> Canvas {
        Canvas {
            is_clip,
            ..Default::default()
        }
    }

    pub fn update_drawcall(canvas_entity:Entity,
                           infos:&Query<&EStateInfo>,
                           children:&Query<&Children>,
                           uirenders:&Query<&UIRender2D>,
                           canvases:&mut Query<&mut Canvas>,
                           zorders:&Query<&ZOrder>,
                           trans:&Query<&Transform>,
                           parents:&Query<&Parent>,
                           meshes:&mut Assets<Mesh>,
                           materials:&mut Assets<Material>,
                           commands:&mut Commands,
                           ui_roots:&mut UIRenderRoot,
                           asset_server:&AssetServer) {
        let is_canvas_active = infos.get(canvas_entity).map(|v| v.is_active_global()).unwrap_or(true);
        if !is_canvas_active {
            if let Ok(mut canvas) = canvases.get_mut(canvas_entity) {
                for old_drawcall in canvas.draw_calls.drain(..) {
                    ui_roots.despawn_next_frame.push(old_drawcall.entity);
                }
            }
            /*
            if let Ok(canvas) = canvases.get_mut(canvas_entity) {
                for drawcall in canvas.draw_calls.iter() {
                   if let Ok(drawcall_state) = infos.get(drawcall.entity) {
                      drawcall_state.set_active(false);
                   } else {
                      let state = EStateInfo::default();
                      state.set_active(false);
                      commands.entity(drawcall.entity).insert(state);
                   }
                }
            }*/
            return;
        }
        let entity_group = ScanDrawCall::scan_entity_group(canvas_entity,infos,children, uirenders, canvases);
        entity_group.iter().flatten().for_each(|entity| {
            ui_roots.entity2canvas.insert(*entity, canvas_entity);
        });
        if let Ok(mut canvas) = canvases.get_mut(canvas_entity) {
            //TODO 这里尝试重用drawcall
            for old_drawcall in canvas.draw_calls.drain(..) {
                ui_roots.despawn_next_frame.push(old_drawcall.entity);
            }
            for draw_entitys in entity_group.iter() {
                let new_drawcall = UIDrawCall::form_entity(
                    canvas_entity,
                    draw_entitys,
                    uirenders,
                    meshes,
                    materials,
                    commands,
                    asset_server,
                    zorders,trans,parents);
                canvas.draw_calls.push(new_drawcall);
            }   
        }

    }
}



pub(crate) struct UIDrawCall {
   pub(crate) entity:Entity,
   pub fst_entity:Entity
}
/*
Canvas       Z:0
  R1         Z:1
  R2         Z:2
  R3[Canvas] Z:3
   R30       Z:4
   R31       Z:5
  R4         Z:6
  R5         Z:7
  R6         Z:8

[R1,R2]
[R4,R5,R6] 
*/

impl UIDrawCall {
    pub fn form_entity(canvas_entity:Entity,
                       entitys:&Vec<Entity>,
                       render2ds:&Query<&UIRender2D>,
                       meshes:&mut Assets<Mesh>,
                       materials:&mut Assets<Material>,
                       commands:&mut Commands,
                       asset_server:&AssetServer,
                       zorders:&Query<&ZOrder>,
                       trans:&Query<&Transform>,
                       parents:&Query<&Parent>) -> UIDrawCall {
        let fst_zorder = zorders.get(entitys[0]).map(|v| v.value).unwrap_or_default();
        let mut positons:Vec<[f32;3]> = vec![];
        let mut uvs:Vec<[f32;2]> = vec![];
        let mut colors:Vec<[f32;4]> = vec![];
        let mut indexs:Vec<u32> = vec![];
        let mut index_offset = 0;
        let mut texture = None;
        let mut material_def:Option<Arc<MaterialDef>> = None;
        let mut custom_mat:Option<Handle<Material>> = None;
        for (index,entity) in entitys.iter().enumerate() {
            if let Ok(render2d) = render2ds.get(*entity) {
                let mat4 = calc_trans(trans, parents, *entity,Some(canvas_entity));
                let z_value:f32 = index as f32 * Z_SCALE;
                texture = render2d.texture.clone();
                material_def = Some(render2d.mat_def.clone());
                for vert in render2d.mesh2d.points.iter() {
                    let mut pos4 = Vec4::new(vert.pos.x, vert.pos.y, z_value, 1f32);
                    pos4 = mat4 * pos4;
                    positons.push(pos4.xyz().into());
                    uvs.push(vert.uv.into());
                    colors.push(render2d.mesh2d.color.into());
                }
                indexs.extend(render2d.mesh2d.indexs.iter().map(|v| v + index_offset));
                index_offset += render2d.mesh2d.points.len() as u32;
                if let Some(mat) = render2d.custom_mat.as_ref() {
                    custom_mat = Some(mat.clone());
                }
            }
        }


        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set(MeshAttributeType::POSITION, positons);
        mesh.set(MeshAttributeType::UV0, uvs);
        mesh.set(MeshAttributeType::COLOR, colors);
        mesh.set_indices(Some(Indices::U32(indexs)));
        mesh.build();
        let h_mesh = meshes.add(mesh);

        
        let h_material = if let Some(custom_mat) = custom_mat.take() {
            custom_mat
        } else {
            let mut new_material = Material::from_def(material_def.unwrap().clone(), asset_server).unwrap();
            if let Some(texture) = texture {
                new_material.texture_props.set("mainTexture", texture.clone());
            }
            materials.add(new_material)
        };

        let canvas_t = trans.get(canvas_entity).unwrap();
        let mut t = Transform::from_matrix(canvas_t.global().matrix());
        t.local.position.z += fst_zorder as f32 * Z_SCALE;
        let mut clone_global = canvas_t.global().clone();
        clone_global.position.z += fst_zorder as f32 * Z_SCALE;
        t.set_global(clone_global);
        
        let drawcall_entity = commands.spawn((h_mesh,h_material,t)).id();
        let mut hasher = DefaultHasher::default();
        entitys.hash(&mut hasher);
        let hash_key = hasher.finish();
        UIDrawCall {
            entity:drawcall_entity,
            fst_entity:entitys[0]
        }
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
    cur_texture:Option<Option<HandleId>>,
    //cur_canvas:Option<Entity>,
    entity_group:Vec<Vec<Entity>>,
    cache:Vec<Entity>,
}

impl ScanDrawCall {
    pub fn scan_entity_group(entity:Entity,infos:&Query<&EStateInfo>,children:&Query<&Children>,
                             uirenders:&Query<&UIRender2D>,canvases:&Query<&mut Canvas>) -> Vec<Vec<Entity>> {
        let mut scan_drawcall = ScanDrawCall { entity_group:vec![],cache:vec![], cur_texture:None  };
        if let Ok(render2d) = uirenders.get(entity) {
            scan_drawcall.cur_texture = Some(render2d.texture.as_ref().map(|v| v.id));
            scan_drawcall.cache.push(entity);
        }
        if let Ok(child_comp) = children.get(entity) {
            for child_entity in child_comp.iter() {
                scan_drawcall._scan_entity_group(*child_entity,infos,children,uirenders,canvases);
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
    /*
    Root [Canvas] 0v0
      V0          1v0
    Stack[Canvas] 2v0
      V1          3V0
      V2          4V0
      V3          5V0
      
    */
    fn _scan_entity_group(&mut self,entity:Entity,infos:&Query<&EStateInfo>,children:&Query<&Children>,uirenders:&Query<&UIRender2D>,canvases:&Query<&mut Canvas>) {
        if canvases.contains(entity) {
            self.emit();
            return;
        }
        if let Ok(render2d) = uirenders.get(entity) {
            let is_active = infos.get(entity).map(|v| v.is_active_global()).unwrap_or(true);
            if is_active {
                match self.cur_texture {
                    None => self.cur_texture = Some(render2d.texture.as_ref().map(|v| v.id)),
                    Some(id) => {
                        if id != render2d.texture.as_ref().map(|v| v.id) {
                            self.emit();
                            self.cur_texture = Some(render2d.texture.as_ref().map(|v| v.id));
                        }
                    }
                }
                self.push(entity);
            }
        }
        if let Ok(child_comp) = children.get(entity) {
            for child_entity in child_comp.iter() {
                self._scan_entity_group(*child_entity,infos,children,uirenders,canvases);
            }
        }
    }
}

fn calc_trans(trans:&Query<&Transform>,parents:&Query<&Parent>,child_entity:Entity,parent_entity:Option<Entity>) -> Mat4 {
    let mut cur_entity = child_entity;
    let mut cur_mat = trans.get(child_entity).map(|t| t.local.matrix()).unwrap_or(Mat4::IDENTITY);
    if Some(cur_entity) == parent_entity {
        return Mat4::IDENTITY;
    }
    while let Ok(parent) = parents.get(cur_entity) {
        cur_entity = parent.0;
        if Some(cur_entity) == parent_entity {
            return cur_mat;
        }
        if let Ok(t) = trans.get(cur_entity) {
            cur_mat = cur_mat * t.local.matrix();
        }        
    }
    cur_mat
}