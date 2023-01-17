use bevy_ecs::{prelude::Entity, system::CommandQueue};
use fnv::FnvHasher;
use std::{collections::{HashMap, HashSet},hash::{Hash, Hasher},sync::Arc};

use crate::{
    render_info::{DrawCallInfo, PanelInfo},
    types::UIZOrder,
    SpriteAllocator,
};

use super::components::{panel::Panel, rect2d::Rect2D, sprite::Sprite};
use seija_core::log;

use seija_app::ecs::prelude::*;
use seija_asset::{AssetServer, Assets};
use seija_render::{
    material::{Material, MaterialDef, MaterialDefineAsset},
    resource::Mesh,
};
use seija_transform::{
    hierarchy::{Children, Parent},
    Transform,
};

#[derive(Default)]
pub(crate) struct UISystemData {
    baseui: Option<Arc<MaterialDef>>,
    panel_infos: HashMap<Entity, PanelInfo>,
}

pub(crate) fn on_after_start(world: &mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();

    let mut h_baseui = server
        .load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None)
        .unwrap();
    h_baseui.forget(); //常驻

    let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
    let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
    if let Some(mut ui_data) = world.get_resource_mut::<UISystemData>() {
        ui_data.baseui = Some(arc_mat_define);
    }
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

   1. 获取所有dirty Sprite
   2. 计算出所有变动的TopPanel
   3. 对TopPanel的ZOrder进行重排
   4. 进行UIDrawcall的比对更新
   5. 根据ZOrder进行UIDrawcall的Z位置调整

*/

/*
进行UIDrawcall的比对算法
Tick:1
  Panel0 = [s0,Split,s2,s3,s4,s5]
Tick:2
  Panel0 = [s0,Split,s2,Split,s3,s4,s5]
  1. entity id做hash
       [hash(s0),hash(s2,s3,s4),hash(s5)]
       [hash(s0),hash(s2),hash(s3,s4,s5)]
  2. 重新遍历一遍同Hash的复用，不同Hash的删除生成新的(处理增删)
      复用:hash(s0) 新生成:hash(s2),hash(s3,s4,s5)
  3. 遍历Update的Entity,从已经更新的Drawcall中找到需要更新的更新一下
*/


pub(crate) fn update_render_system(world: &mut World) {
    let mut update_sprites = world
        .query_filtered::<Entity, Or<(Changed<Sprite>, Changed<Rect2D>, Changed<Transform>)>>();
    let mut remove_sprites = world.removed::<Sprite>();

    let mut panels = world.query::<(Entity, &Panel)>();
    let mut parents = world.query::<&Parent>();
    let mut zorders = world.query::<&mut UIZOrder>();
    let mut childrens = world.query::<&Children>();
    let mut sprites = world.query::<(&Sprite, &Rect2D, &Transform)>();

    //计算出所有变动的Top Panel(穿透到最顶层)
    let mut dirty_top_panels: HashSet<Entity> = HashSet::default();
    let mut dirty_sprites: HashSet<Entity> = HashSet::default();
    for e in update_sprites.iter(world) {
        dirty_sprites.insert(e);
        if let Some(e) = calc_top_panel(world, e, &mut panels, &mut parents) {
            dirty_top_panels.insert(e);
        }
    }
    //对TopPanel的ZOrder进行重排
    fill_ui_zorders(world, dirty_top_panels, &mut zorders, &mut childrens);

    //计算出所有变动的Panel
    let mut dirty_panels: HashSet<Entity> = Default::default();
    //增 + 改
    for entity in update_sprites.iter(world) {
        fill_dirty_panel(world, entity, &mut panels, &mut parents, &mut dirty_panels);
    }
    //TODO 删

    
    let mut delete_drawcall_info:Vec<DrawCallInfo> = vec![];
    let mut update_render_panels:Vec<Entity> = vec![];
    world.resource_scope(|world: &mut World, mut system_data: Mut<UISystemData>| {
        //开始比对PanelInfo
        let mut fst_create: Vec<Entity> = Vec::new();
        let material_def = system_data.baseui.as_ref().unwrap().clone();
        let sprite_alloc = world.get_resource::<SpriteAllocator>().unwrap();
        
        //比对更新Panel的Drawcall
        for panel_entity in dirty_panels.iter() {
            if let Some(panel_info) = system_data.panel_infos.get_mut(&panel_entity) {
                let panel_drawcalls = PanelInfo::scan_drawcalls(world,*panel_entity,&mut childrens,&mut sprites,&mut panels);
                let mut last_cache_dcs:HashMap<u64,DrawCallInfo> = Default::default();
                for drawcall_info in panel_info.drawcalls.drain(..) {
                    last_cache_dcs.insert(drawcall_info.key, drawcall_info);
                }
                
                for drawcall in panel_drawcalls {
                    let mut hasher = FnvHasher::default();
                    drawcall.iter().for_each(|e| e.hash(&mut hasher));
                    let drawcall_key = hasher.finish();
                    if last_cache_dcs.contains_key(&drawcall_key) {
                        let is_dirty = drawcall.iter().any(|e| dirty_sprites.contains(e));
                        if is_dirty {
                            let drawcall_info = DrawCallInfo::build_from(&world,drawcall,&mut sprites,&mut zorders,sprite_alloc);
                            panel_info.drawcalls.push(drawcall_info);
                        } else {
                            if let Some(drawcall_info) = last_cache_dcs.remove(&drawcall_key) {
                                panel_info.drawcalls.push(drawcall_info);
                            }
                        }
                    } else {
                       let drawcall_info = DrawCallInfo::build_from(&world,drawcall,&mut sprites,&mut zorders,sprite_alloc);
                       panel_info.drawcalls.push(drawcall_info);
                    }
                }
                for (_,last_draw_call) in last_cache_dcs {
                    delete_drawcall_info.push(last_draw_call);
                }
                update_render_panels.push(*panel_entity);
            } else {
                fst_create.push(*panel_entity);
            }
        }
        
        
        
        for delete_dc in delete_drawcall_info {
            if let Some(render_entity) = delete_dc.render_entity {
                log::error!("despawn:{:?}",render_entity);
                world.entity_mut(render_entity).despawn();
            }
        }
       
        let mut asset_meshs = unsafe { world.get_resource_unchecked_mut::<Assets<Mesh>>().unwrap() };
        let mut asset_materials = unsafe { world.get_resource_unchecked_mut::<Assets<Material>>().unwrap() };
        let server = world.get_resource::<AssetServer>().unwrap().clone();
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, world);
        for update_panel_id in update_render_panels {
            if let Some(panel_info) = system_data.panel_infos.get_mut(&update_panel_id) {
                for drawcall_info in panel_info.drawcalls.iter_mut() {
                    let mesh: Mesh = drawcall_info.mesh.take().unwrap().into();
                    let h_mesh = asset_meshs.add(mesh);
                    let material = Material::from_def(material_def.clone(), &server).unwrap();
                    let h_mat = asset_materials.add(material);
                    let render_entity = commands.spawn().insert(Transform::default()).insert(h_mesh.clone()).insert(h_mat.clone()).id();
                    log::error!("diff create:{:?}",&render_entity);
                    drawcall_info.render_entity = Some(render_entity);
                }
            }
        }
        queue.apply(world);


        //处理首次创建的Panel
        process_fst_create(world,fst_create,&mut panels,&mut childrens,&mut zorders,&mut sprites,material_def.clone(),&mut system_data);
    });
}



fn process_fst_create(
    world: &mut World,
    mut fst_create: Vec<Entity>,
    mut panels: &mut QueryState<(Entity, &Panel)>,
    mut childrens: &mut QueryState<&Children>,
    zorders: &mut QueryState<&mut UIZOrder>,
    mut sprites: &mut QueryState<(&Sprite, &Rect2D, &Transform)>,
    material_def: Arc<MaterialDef>,
    system_data:&mut UISystemData
) {
    let mut fst_panel_infos: Vec<PanelInfo> = vec![];
    let sprite_alloc = world.get_resource::<SpriteAllocator>().unwrap();
    //扫描并生成PanelInfo
    for panel_entity in fst_create.drain(..) {
        let mut drawcall_infos = vec![];
        let drawcalls = PanelInfo::scan_drawcalls(world,panel_entity,&mut childrens,&mut sprites,&mut panels);
        log::info!("scan_drawcalls:{}", drawcalls.len());
        for drawcall in drawcalls {
            let mut meshs = vec![];
            let mut sprite_entitys: Vec<Entity> = vec![];
            for sprite_id in drawcall {
                if let Ok((sprite, rect2d, trans)) = sprites.get(world, sprite_id) {
                    if let Some(k) = sprite.sprite_index {
                        if let Some(info) = sprite_alloc.get_sprite_info(k) {
                            let global = trans.global();
                            let zorder = zorders.get(world, sprite_id).unwrap().value;
                            let mesh2d = sprite.build(rect2d,info.uv.clone(),&global.matrix(),&info.rect,zorder as f32 * 0.01f32);
                            meshs.push(mesh2d);
                            sprite_entitys.push(sprite_id);
                        }
                    }
                }
            }
            let info = DrawCallInfo::create(meshs, sprite_entitys);
            drawcall_infos.push(info);
        }
        let panel_info = PanelInfo::create(panel_entity, drawcall_infos);
        fst_panel_infos.push(panel_info);
    }

    //根据PanelInfo里的DrawcallInfo创建实际渲染物体
    let mut asset_meshs = unsafe { world.get_resource_unchecked_mut::<Assets<Mesh>>().unwrap() };
    let mut asset_materials = unsafe { world.get_resource_unchecked_mut::<Assets<Material>>().unwrap() };
    let server = world.get_resource::<AssetServer>().unwrap();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    for panel in fst_panel_infos.iter_mut() {
        for drawcall_info in panel.drawcalls.iter_mut() {
            let mesh: Mesh = drawcall_info.mesh.take().unwrap().into();
            let h_mesh = asset_meshs.add(mesh);
            let material = Material::from_def(material_def.clone(), &server).unwrap();
            let h_mat = asset_materials.add(material);
            let render_entity = commands.spawn().insert(Transform::default()).insert(h_mesh).insert(h_mat).id();
            log::error!("create render:{:?}",&render_entity);
            drawcall_info.render_entity = Some(render_entity);
        }
    }
    queue.apply(world);
    for info in fst_panel_infos.drain(..) {
      system_data.panel_infos.insert(info.panel_id, info);
    }
}

fn fill_dirty_panel(
    world: &World,
    entity: Entity,
    panels: &mut QueryState<(Entity, &Panel)>,
    parents: &mut QueryState<&Parent>,
    dirty_panels: &mut HashSet<Entity>,
) {
    let mut cur_entity: Entity = entity;
    let mut last_panel: Option<Entity> = None;
    while let Ok(parent) = parents.get(world, cur_entity) {
        cur_entity = parent.0;
        if let Ok((e, panel)) = panels.get(world, cur_entity) {
            last_panel = Some(e);
            if !panel.is_static {
                dirty_panels.insert(e);
                return;
            }
        }
    }

    if let Some(last) = last_panel.take() {
        dirty_panels.insert(last);
    }
}

fn calc_top_panel(
    world: &World,
    entity: Entity,
    panels: &mut QueryState<(Entity, &Panel)>,
    parents: &mut QueryState<&Parent>,
) -> Option<Entity> {
    let mut cur_entity: Entity = entity;
    let mut top_panel_entity: Option<Entity> = None;
    while let Ok(parent) = parents.get(world, cur_entity) {
        cur_entity = parent.0;
        if panels.get(world, cur_entity).is_ok() {
            top_panel_entity = Some(cur_entity);
        }
    }
    top_panel_entity
}

fn fill_ui_zorders(
    world: &mut World,
    panels: HashSet<Entity>,
    zorders: &mut QueryState<&mut UIZOrder>,
    childrens: &mut QueryState<&Children>,
) {
    for panel_entity in panels.iter() {
        if let Ok(zorder) = zorders.get(world, *panel_entity) {
            let value = zorder.value;
            _fill_ui_zorders(world, *panel_entity, value + 1, zorders, childrens);
        } else {
            _fill_ui_zorders(world, *panel_entity, 1, zorders, childrens);
        }
    }
}

fn _fill_ui_zorders(
    world: &mut World,
    entity: Entity,
    number: i32,
    zorders: &mut QueryState<&mut UIZOrder>,
    childrens: &mut QueryState<&Children>,
) {
    let mut now_index = number;
    if let Ok(childs) = childrens.get(world, entity) {
        let children_ids = childs.children().iter().map(|v| *v).collect::<Vec<_>>();
        for child_entity in children_ids {
            if let Ok(mut zorder) = zorders.get_mut(world, child_entity) {
                zorder.last = zorder.value;
                zorder.value = now_index;
            } else {
                world.entity_mut(child_entity).insert(UIZOrder {
                    last: -1,
                    value: now_index,
                });
            }
            log::info!("set zorder{:?}={}", child_entity, now_index);
            now_index += 1;
            _fill_ui_zorders(world, child_entity, now_index, zorders, childrens);
        }
    }
}
