use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use fnv::FnvHasher;
use seija_asset::{Assets, AssetServer};
use seija_core::log;
use bevy_ecs::system::{Res,SystemParam, SystemState, CommandQueue};
use bevy_ecs::prelude::*;
use seija_core::math::Mat4;
use seija_core::time::Time;
use seija_render::material::{Material, MaterialDef, MaterialDefineAsset};
use seija_render::resource::Mesh;
use seija_transform::{Transform, IEntityChildren};
use seija_transform::events::HierarchyEvent;
use seija_transform::hierarchy::{Parent,PreviousParent, Children};
use crate::SpriteAllocator;
use crate::components::panel::Panel;
use crate::components::rect2d::Rect2D;
use crate::components::sprite::Sprite;
use crate::info::{RenderPanelInfo, RenderDrawCall};
use crate::types::UIZOrder;



#[derive(Resource)]
pub struct UIRenderData {
    pub(crate) baseui: Arc<MaterialDef>,
    pub(crate) entity2panel:HashMap<Entity,Entity>,
    pub(crate) render_panels:HashMap<Entity,RenderPanelInfo>,
    pub(crate) next_frame_despawns:Vec<Entity>
}

#[derive(SystemParam)]
pub struct SystemParams<'w,'s> {
   pub(crate) sprite_alloc:Res<'w,SpriteAllocator>,
   pub(crate) update_sprites:Query<'w,'s,Entity,Or<(Changed<Panel>, Changed<Rect2D>, Changed<Transform>)>>,
   pub(crate) update_panels:Query<'w,'s,Entity,Or<(Changed<Panel>, Changed<Rect2D>, Changed<Transform>)>>,
   pub(crate) remove_sprites:RemovedComponents<'w,Sprite>,
   pub(crate) remove_panels:RemovedComponents<'w,Panel>,
   pub(crate) panels:Query<'w,'s,(Entity,&'static Panel)>,
   pub(crate) trans:Query<'w,'s,&'static Transform>,
   pub(crate) parents:Query<'w,'s,(Entity,&'static Parent,Option<&'static PreviousParent>)>,
   pub(crate) zorders:Query<'w,'s,&'static mut UIZOrder>,
   pub(crate) childrens:Query<'w,'s,(Entity,&'static Children)>,
   pub(crate) sprites:Query<'w,'s,(Entity,&'static Sprite, &'static Rect2D)>,
   pub(crate) events:EventReader<'w,'s,HierarchyEvent>,
   pub(crate) render_data:ResMut<'w,UIRenderData>,
   pub(crate) assets:(ResMut<'w,Assets<Mesh>>,ResMut<'w,Assets<Material>>,Res<'w,AssetServer>),
   pub(crate) time:Res<'w,Time>,
   pub(crate) commands: Commands<'w,'s>
}

#[derive(Resource)]
struct CachedQueryState {
    query: SystemState<SystemParams<'static,'static>>
}

impl<'w,'s> SystemParams<'w,'s> {
    pub fn get_top_panel(&self,entity:Entity) -> Option<Entity> {
        let mut cur_entity: Entity = entity;
        let mut top_panel_entity: Option<Entity> = None;
        while let Ok(parent) = self.parents.get(cur_entity) {
            cur_entity = parent.1.0;
            if self.panels.get(cur_entity).is_ok() {
                top_panel_entity = Some(cur_entity);
            }
        }
        top_panel_entity
    }

    pub fn get_render_parent_panel(&self,entity:Entity) -> Option<Entity> {
        let mut cur_entity: Entity = entity;
        let mut last_panel: Option<Entity> = None;
        while let Ok((_,parent,_)) = self.parents.get(cur_entity) {
            cur_entity = parent.0;
            if let Ok((e, panel)) = self.panels.get(cur_entity) {
                last_panel = Some(e);
                if !panel.is_static {
                    return Some(e);
                }
            }
        }
        if let Some(last) = last_panel.take() {
            return Some(last);
        }
        None
    }

    pub fn visit_mut<F>(&mut self,entity:Entity,f:&mut F) where F:FnMut(Entity,&mut Self) {
        f(entity,self);
        if let Ok(childrens) = self.childrens.get(entity) {
            let children_lst = childrens.1.iter().cloned().collect::<Vec<_>>();
            for child_entity in children_lst {
                self.visit_mut(child_entity, f);
            }
        }
    }

    pub fn calc_trans(&self,child_entity:Entity,parent_entity:Entity) -> Mat4 {
        let mut cur_entity = child_entity;
        let mut cur_mat = self.trans.get(child_entity).map(|t| t.local.matrix()).unwrap_or(Mat4::IDENTITY);
        while let Ok((_,parent,_)) = self.parents.get(cur_entity) {
            cur_entity = parent.0;
            if cur_entity == parent_entity {
                return cur_mat;
            }
            if let Ok(t) = self.trans.get(cur_entity) {
                cur_mat = cur_mat * t.local.matrix();
            }

            
        }
        cur_mat
    }
}

pub(crate) fn on_ui_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
    let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
    let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
    //常驻
    h_baseui.forget();
    world.insert_resource(UIRenderData {
        baseui:arc_mat_define,
        entity2panel:Default::default(),
        render_panels:Default::default(),
        next_frame_despawns:vec![]
    });

    let param_state = SystemState::<SystemParams>::from_world(world);
    world.insert_resource(CachedQueryState {
        query:param_state
    });
}

pub(crate) fn ui_update_zorders(world:&mut World) {
    world.resource_scope(|world:&mut World,mut cached_query:Mut<CachedQueryState>| {
        //let mut params = cached_query.query.get_mut(world);
        //UpdateZOrders::default().run(&mut params);
        //cached_query.query.apply(world);
    });
}

//处理Sprite增删改，处理Panel增删改，处理Entity层级变化
pub(crate) fn ui_render_system(world:&mut World) {
    world.resource_scope(|world:&mut World,mut cached_query:Mut<CachedQueryState>| {
        let mut params = cached_query.query.get_mut(world);
        UpdateZOrders::default().run(&mut params);
        
        let mut dirty_collect = DirtyCollect::default();
        dirty_collect.run(&mut params);
        let mut process = ProcessUIDirty::default();
        process.run(&dirty_collect, &mut params);
        cached_query.query.apply(world);
    });
}


#[derive(Default)]
struct UpdateZOrders {
   pub(crate) dirty_top_panels:HashSet<Entity>,
}

impl UpdateZOrders {
    pub fn run(mut self,params:&mut SystemParams) {
        for entity in params.update_sprites.iter() {
            if params.sprites.contains(entity) {
                if let Some(e) = params.get_top_panel(entity) {
                    self.dirty_top_panels.insert(e);
                }
            }
        }
        
        for panel_entity in self.dirty_top_panels {
            let mut now_index = 1;
            if let Ok(panel_zorder) = params.zorders.get(panel_entity) {
                now_index = panel_zorder.value;
            }
            let now_index_ref = &mut now_index;
            params.visit_mut(panel_entity, &mut |entity,fn_params| {
               if let Ok(mut zorder) = fn_params.zorders.get_mut(entity) {
                  zorder.last = zorder.value;
                  zorder.value = *now_index_ref;
               } else {
                  fn_params.commands.entity(entity).insert(UIZOrder {
                        last: -1,
                        value: *now_index_ref
                  });
               }
               *now_index_ref += 1;
            });
        }
    }
}

#[derive(Default,Debug)]
struct DirtyCollect {
    dirty_sprites:HashSet<Entity>,
    dirty_panels:HashSet<Entity>,
    delete_panels:HashSet<Entity>
}

impl DirtyCollect {
    pub fn run(&mut self,params:&mut SystemParams) {
        //Sprite 增 + 改
        for sprite_entity in params.update_sprites.iter() {
            if params.sprites.contains(sprite_entity) {
                self.dirty_sprites.insert(sprite_entity);
                if let Some(entity) = params.get_render_parent_panel(sprite_entity) {
                    log::error!("dirty_panels 0 {:?} frame:{}",sprite_entity,params.time.frame());
                    self.dirty_panels.insert(entity);
                }
            }
        }
        //Sprite 删
        for remove_entity in params.remove_sprites.iter() {
            if let Some(panel_entity) = params.render_data.entity2panel.remove(&remove_entity) {
                if params.render_data.render_panels.contains_key(&panel_entity) {
                    log::error!("dirty_panels 1");
                    self.dirty_panels.insert(panel_entity);
                }
            }
        }

        //Panel 增 + 改
        for update_panel in params.update_panels.iter() {
            if !params.panels.contains(update_panel) {
                continue;
            }
            if let Ok((entity,panel)) = params.panels.get(update_panel) {
                if panel.is_static {
                    //静态Panel,找到Render顶层标记Dirty
                   if let Some(render_panel) = params.get_render_parent_panel(entity) {
                      log::error!("dirty_panels 2");
                      self.dirty_panels.insert(render_panel);
                   }
                } else { 
                    //动态Panel,无需处理?
                }
            }
        }

        //Panel 删
        for remove_panel in params.remove_panels.iter() {
            //动态或者顶层Panel直接删除
            if params.render_data.render_panels.remove(&remove_panel).is_some() {
                self.delete_panels.insert(remove_panel);
            } else {
                //子静态Panel,找到Render顶层标记Dirty
                if let Some(render_panel) = params.get_render_parent_panel(remove_panel) {
                    log::error!("dirty_panels 3");
                    self.dirty_panels.insert(render_panel);
                }
            }
        }

        //Entity层级变化
        
        let mut hierarchy_events = params.events.iter().map(|v| v.clone()).collect::<Vec<_>>();
        for event in hierarchy_events.drain(..) {
            log::error!("parent hcange?");
            match event {
                HierarchyEvent::ParentChange { entity , old_parent, new_parent } => {
                    //对Old进行删除比对
                    if let Some(old_parent) = old_parent {
                        if params.sprites.contains(entity) {
                            if let Some(render_panel) = params.get_render_parent_panel(old_parent) {
                                self.dirty_panels.insert(render_panel);
                                self.dirty_sprites.insert(entity);
                            }
                        } else if params.panels.contains(entity) {
                            if let Ok(is_static) = params.panels.get(entity).map(|v| v.1.is_static) {
                                //静态Panel,标记所属Render Panel Dirty
                                if is_static {
                                    if let Some(render_panel) = params.get_render_parent_panel(old_parent) {
                                        self.dirty_panels.insert(render_panel);
                                    }
                                } else {
                                    //动态Panel移动到新位置无需处理
                                }
                            }

                        }
                    }
                    
                    //对New进行添加比对
                    if let Some(new_parent) = new_parent {
                        if params.sprites.contains(entity) {
                            if let Some(render_panel) = params.get_render_parent_panel(new_parent) {
                                self.dirty_panels.insert(render_panel);
                                self.dirty_sprites.insert(entity);
                            }
                        } else if params.panels.contains(entity) {
                            if let Ok(is_static) = params.panels.get(entity).map(|v| v.1.is_static) {
                                //静态Panel,标记所属Render Panel Dirty
                                if is_static {
                                    if let Some(render_panel) = params.get_render_parent_panel(new_parent) {
                                        self.dirty_panels.insert(render_panel);
                                    }
                                } else {
                                    //动态Panel无需处理?
                                }
                            }
                        }
                    }
                }
            }
        }

    }
}

#[derive(Default)]
struct ProcessUIDirty {

}

impl ProcessUIDirty {
    pub fn run(&mut self,dirty_data:&DirtyCollect,params:&mut SystemParams) {
        for delete_entity in params.render_data.next_frame_despawns.drain(..) {
            log::error!("despawn_recursive:{:?} {}",&delete_entity,params.time.frame());
            params.commands.entity(delete_entity).despawn_recursive();
        } 

        for delete_panel in dirty_data.delete_panels.iter() {
            if let Some(panel_info) = params.render_data.render_panels.remove(delete_panel) {
                for drawcall in panel_info.drawcalls {
                    log::error!("push despawn 0:{:?}",drawcall.render_entity);
                    params.render_data.next_frame_despawns.push(drawcall.render_entity);
                 }
            }
        }

        for dirty_panel_entity in dirty_data.dirty_panels.iter() {
            if params.render_data.render_panels.contains_key(dirty_panel_entity) {
                self.diff_update_panel_render(*dirty_panel_entity,params,dirty_data);
            } else {
                let new_panel_render = self.create_panel_render(*dirty_panel_entity,params);
                params.render_data.render_panels.insert(*dirty_panel_entity, new_panel_render);
            }
        }

        
    }

    fn diff_update_panel_render(&self,panel_entity:Entity,params:&mut SystemParams,dirty_data:&DirtyCollect) {
        let mut new_create_lst:Vec<Vec<Entity>> = vec![];
        let drawcall_lst_entitys = ScanDrawCall::scan(panel_entity, params);
        if let Some(old_panel) = params.render_data.render_panels.get_mut(&panel_entity) {
            let mut last_cache_drawcalls:HashMap<u64,RenderDrawCall> = Default::default();
            for drawcall_info in old_panel.drawcalls.drain(..) {
                last_cache_drawcalls.insert(drawcall_info.key, drawcall_info);
            }

            for drawcall_entitys in drawcall_lst_entitys {
                let mut hasher = FnvHasher::default();
                for entity in drawcall_entitys.iter() {
                    entity.hash(&mut hasher);
                }
                let drawcall_key = hasher.finish();
                if last_cache_drawcalls.contains_key(&drawcall_key) {
                    let is_dirty = drawcall_entitys.iter().any(|e| dirty_data.dirty_sprites.contains(e));
                    if is_dirty {
                        new_create_lst.push(drawcall_entitys);
                    } else {
                        //没变
                        if let Some(drawcall_info) = last_cache_drawcalls.remove(&drawcall_key) {
                            old_panel.drawcalls.push(drawcall_info);
                        }
                    }
                } else {
                    new_create_lst.push(drawcall_entitys);
                }
            }

            for last_cache in last_cache_drawcalls {
                log::error!("push despawn 1:{:?}",&last_cache.1.render_entity);
                params.render_data.next_frame_despawns.push(last_cache.1.render_entity);
            }
        }

        for entity in new_create_lst.iter().flatten() {
            if !params.render_data.entity2panel.contains_key(entity) {
                params.render_data.entity2panel.insert(*entity, panel_entity);
            }
        }
        
        let new_drawcalls = new_create_lst.drain(..).map(|entitys| {
            log::error!("create:{:?}-{:?} {}",&entitys,panel_entity,params.time.frame());
            RenderDrawCall::create(panel_entity,params,&entitys)
           
        }).collect::<Vec<_>>();
        if let Some(old_panel) = params.render_data.render_panels.get_mut(&panel_entity) {
            for drawcall in new_drawcalls {  
                old_panel.drawcalls.push(drawcall);
            }
        }
        

    }

    fn create_panel_render(&self,panel_entity:Entity,params:&mut SystemParams) -> RenderPanelInfo {
        log::error!("first create?");
        let mut drawcalls:Vec<RenderDrawCall> = vec![];
        let drawcall_lst_entitys = ScanDrawCall::scan(panel_entity, params);
        for drawcall_entitys in drawcall_lst_entitys {
            for sprite_entity in drawcall_entitys.iter() {
                params.render_data.entity2panel.insert(*sprite_entity, panel_entity);
            }
            let new_drawcall = RenderDrawCall::create(panel_entity,params, &drawcall_entitys);
            drawcalls.push(new_drawcall);
        }
        RenderPanelInfo { panel_id: panel_entity, drawcalls }
    }
}


#[derive(Default,Debug)]
struct ScanDrawCall {
    caches:Vec<Entity>,
    drawcalls:Vec<Vec<Entity>>
}

impl ScanDrawCall {
    pub fn scan(entity:Entity,params:&SystemParams) -> Vec<Vec<Entity>> {
        let mut scan_info = ScanDrawCall::default();
        scan_info.visit_element(entity, params);
        scan_info.emit();
        scan_info.drawcalls
    }

    fn visit_element(&mut self,entity:Entity,params:&SystemParams) {
        if let Ok(child_comp) = params.childrens.get(entity) {
            for child in child_comp.1.iter() {
                if params.sprites.contains(*child) {
                    self.push(*child);
                } else if let Ok(panel) = params.panels.get(*child) {
                    if panel.1.is_static {
                        self.visit_element(entity, params);
                    } else {
                        self.emit();
                    }
                }
            }
        }
    }

    fn push(&mut self,entity:Entity) {
        self.caches.push(entity);
    }

    fn emit(&mut self) {
        let take_list = self.caches.drain(..).collect::<Vec<_>>();
        self.drawcalls.push(take_list);
    }
}