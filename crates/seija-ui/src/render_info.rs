use std::hash::{Hash, Hasher};

use bevy_ecs::{world::World, prelude::Entity, query::QueryState};
use fnv::FnvHasher;
use seija_core::math::Vec4;
use seija_transform::{hierarchy::Children, Transform};
use crate::{components::{sprite::Sprite, panel::Panel, rect2d::Rect2D}, mesh2d::{Mesh2D, Vertex2D}, types::UIZOrder, SpriteAllocator};

#[derive(Debug)]
pub struct PanelInfo {
   pub(crate) panel_id:Entity,
   pub(crate) drawcalls:Vec<DrawCallInfo>
}

/*example 1
Panel0                ZOrder(0)
   Sprite0            ZOrder(1)
   Panel1(dynamic)    ZOrder(2)
     Sprite1          ZOrder(3)
   Sprite2            ZOrder(4)
   Panel2             ZOrder(5)
     Sprite3          ZOrder(6)
   Sprite4            ZOrder(7)
*/

impl PanelInfo {
    pub fn create(entity:Entity,infos:Vec<DrawCallInfo>) -> Self {
        PanelInfo {panel_id:entity, drawcalls: infos }
    }

    pub fn scan_drawcalls(world:&World,entity:Entity,childrens:&mut QueryState<&Children>,
                          sprites:&mut QueryState<(&Sprite,&Rect2D,&Transform)>,
                          panels:&mut QueryState<(Entity,&Panel)>) -> Vec<Vec<Entity>> {
        let mut scan_drawcall = ScanDrawCall::default();
        Self::visit_element(world,&mut scan_drawcall, entity, childrens, sprites, panels);
        scan_drawcall.emit();
        scan_drawcall.dcs
    }

    fn visit_element(world:&World,scan_drawcall:&mut ScanDrawCall,entity:Entity,childrens:&mut QueryState<&Children>,
                    sprites:&mut QueryState<(&Sprite,&Rect2D,&Transform)>,panels:&mut QueryState<(Entity,&Panel)>) {
        if let Ok(comp_child) = childrens.get(world, entity) {
            for child in comp_child.iter() {
                if let Ok(_) = sprites.get(world, *child) {
                    scan_drawcall.push(*child);
                } else if let Ok(panel) = panels.get(world, *child) {
                    if !panel.1.is_static {
                        scan_drawcall.emit();
                    } else {
                        Self::visit_element(world, scan_drawcall, *child, childrens, sprites, panels);
                    }
                }
            }
        }
    }
}

#[derive(Default,Debug)]
struct ScanDrawCall {
    caches:Vec<Entity>,
    dcs:Vec<Vec<Entity>>
}

impl ScanDrawCall {
    pub fn push(&mut self,entity:Entity) {
        self.caches.push(entity);
    }

    pub fn emit(&mut self) {
        let take_list = self.caches.drain(..).collect::<Vec<_>>();
        self.dcs.push(take_list);
    }
}
#[derive(Debug,Default)]
pub struct DrawCallInfo {
    pub key:u64,
    pub(crate) mesh:Option<Mesh2D>,
    pub sprites:Vec<Entity>,
    pub render_entity:Option<Entity>
}

impl DrawCallInfo {
    pub fn create(meshs:Vec<Mesh2D>,sprites:Vec<Entity>) -> Self {
        let mut points:Vec<Vertex2D> = vec![];
        let mut indexs:Vec<u32> = vec![];
        let mut index_offset = 0u32;
        
        for mesh in meshs {
            points.extend(mesh.points.iter());
            indexs.extend(mesh.indexs.iter().map(|v| v + index_offset));
            index_offset += mesh.points.len() as u32;
        }
        
        let mut hasher = FnvHasher::default();
        for sprite_id in sprites.iter() {
            sprite_id.hash(&mut hasher);
        }
        let key = hasher.finish();
        DrawCallInfo {
            key,
            mesh:Some(Mesh2D {points,indexs,color:Vec4::ONE }),
            sprites,
            render_entity:None
        }
    }


    pub fn build_from(world:&World,entitys:Vec<Entity>,
                      sprites: &mut QueryState<(&Sprite, &Rect2D, &Transform)>,
                      zorders:&mut QueryState<&mut UIZOrder>,
                      sprite_alloc:&SpriteAllocator) -> DrawCallInfo {
        let mut meshs = vec![];
        let mut sprite_entitys: Vec<Entity> = vec![];
        for entity in entitys {
            if let Ok((sprite, rect2d, trans)) = sprites.get(world, entity) {
                if let Some(k) = sprite.sprite_index {
                    if let Some(info) = sprite_alloc.get_sprite_info(k) {
                        let global = trans.global();
                        let zorder = zorders.get(world, entity).unwrap().value;
                        let mesh2d = sprite.build(rect2d,info.uv.clone(),&global.matrix(),&info.rect,zorder as f32 * 0.01f32);
                        meshs.push(mesh2d);
                        sprite_entitys.push(entity);
                    }
                }
            }
        }
        DrawCallInfo::create(meshs, sprite_entitys)
       
    }
}