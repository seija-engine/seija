use std::{sync::Arc, collections::{HashSet, HashMap}};
use bevy_ecs::{world::World, system::{Resource, 
    SystemParam, Query, Commands, Res, ResMut}, prelude::{Entity, EventWriter, EventReader}, query::{Or, Changed, ChangeTrackers}};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{math::{Vec3, Vec4}, info::EStateInfo};
use seija_render::{material::{MaterialDefineAsset, MaterialDef, Material},
                   resource::{ Mesh, Texture, ImageInfo, TextureDescInfo, BufferId}};
use seija_transform::{hierarchy::{Parent, Children}, Transform, events::HierarchyEvent};
use spritesheet::SpriteSheet;
use glyph_brush::{GlyphBrush, GlyphBrushBuilder,FontId,BrushAction};
use crate::{components::{sprite::Sprite, rect2d::Rect2D, canvas::{Canvas, ZOrder, Z_SCALE}, input::{InputTextSystemData, Input}}, 
            render::{UIRender2D, WriteFontAtlas}, 
            mesh2d::Vertex2D, text::{Text, Font, glyph_to_mesh, write_font_texture}, types::Box2D};
use wgpu::TextureFormat;
#[derive(Resource)]
pub struct UIRenderRoot {
    pub(crate) baseui:Arc<MaterialDef>,
    pub(crate) basetext:Arc<MaterialDef>,
    pub(crate) caret_mat_def:Arc<MaterialDef>,
    pub(crate) text_brush:GlyphBrush<Vec<Vertex2D>>,
    pub(crate) font_texture:Handle<Texture>,
    pub(crate) font_caches:HashMap<Handle<Font>,FontId>,
    pub(crate) font_buffer:Option<BufferId>,

    pub(crate) entity2canvas:HashMap<Entity,Entity>,
    pub(crate) despawn_next_frame:Vec<Entity>,
}

pub(crate) fn on_ui_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut h_baseui = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
    let mut h_basetext = server.load_sync::<MaterialDefineAsset>(world, "materials/text.mat.clj", None).unwrap();
    let mut h_caret = server.load_sync::<MaterialDefineAsset>(world, "materials/inputCaret.mat.clj", None).unwrap();

    let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
    let arc_mat_define = mats.get(&h_baseui.id).unwrap().define.clone();
    let arc_text_mat_define = mats.get(&h_basetext.id).unwrap().define.clone();
    let arc_caret_mat_define = mats.get(&h_caret.id).unwrap().define.clone();
    //常驻
    h_baseui.forget();
    h_basetext.forget();
    h_caret.forget();

    let font_texture = create_font_texture(world);
    world.insert_resource(UIRenderRoot {
        baseui:arc_mat_define,
        basetext:arc_text_mat_define,
        text_brush:GlyphBrushBuilder::using_fonts(vec![])
                    .cache_redraws(false)
                    .initial_cache_size((1024, 1024)).build(),
        font_caches:HashMap::default(),
        font_texture,
        font_buffer:None,
        despawn_next_frame:vec![],
        entity2canvas:HashMap::default(),
        caret_mat_def:arc_caret_mat_define
    });
    world.insert_resource(InputTextSystemData::default());
}

fn create_font_texture(world:&mut World) -> Handle<Texture> {
    let image_info = ImageInfo {width:1024,height:1024,format:TextureFormat::R8Unorm,data:vec![0u8;1024 * 1024] };
    let mut texture_desc = TextureDescInfo::default();
    texture_desc.desc.label = "font_texture".into();
    let font_texture = Texture::create_image(image_info, texture_desc);
    let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
    let h_texture = textures.add(font_texture);
    h_texture
}

#[derive(SystemParam)]
pub struct RenderMeshParams<'w,'s> {
    pub(crate) update_sprites:Query<'w,'s,Entity,Or<(Changed<Sprite>,Changed<Rect2D>)>>,
    pub(crate) update_texts:Query<'w,'s,Entity,Or<(Changed<Text>,Changed<Rect2D>)>>,
    pub(crate) render2d:Query<'w,'s,&'static mut UIRender2D>,
    pub(crate) font_assets:Res<'w,Assets<Font>>,
    pub(crate) sprites:Query<'w,'s,(&'static Sprite,&'static Rect2D)>,
    pub(crate) texts:Query<'w,'s,(&'static Text,&'static Rect2D)>,
    pub(crate) spritesheets:Res<'w,Assets<SpriteSheet>>,
    pub(crate) textures:ResMut<'w,Assets<Texture>>,
    pub(crate) ui_roots:ResMut<'w,UIRenderRoot>,
    pub(crate) commands:Commands<'w,'s>,
    pub(crate) canvases:Query<'w,'s,&'static Canvas>,
    pub(crate) parents:Query<'w,'s,&'static Parent>,
    pub(crate) zorders:Query<'w,'s,&'static mut ZOrder>,
    pub(crate) children:Query<'w,'s,&'static Children>,
    pub(crate) write_font_atlas:EventWriter<'w,'s,WriteFontAtlas>
}

pub fn update_render_mesh_system(mut params:RenderMeshParams,server:Res<AssetServer>) {
    let mut top_changed_canvas:HashSet<Entity> = HashSet::default();
    //更新Input的Mesh 
    /*
    for (entity,input,rect2d) in params.update_inputs.iter() {
        if true {  
            if let Ok(mut render2d) = params.render2d.get_mut(entity) {
                render2d.mesh2d = Input::build_caret_mesh(rect2d,0f32);
                render2d.texture = server.get_asset("texture:white").unwrap().make_weak_handle().typed();
            } else {
                let mesh = Input::build_caret_mesh(rect2d,0f32);
                let h_white = server.get_asset("texture:white").unwrap().make_weak_handle().typed();
                let r2d = UIRender2D {
                    mat:params.ui_roots.baseui.clone(),
                    mesh2d : mesh,
                    texture:h_white
                };
                params.commands.entity(entity).insert(r2d);
            }
        }
    }*/

    //更新Sprite的Mesh
    for entity in params.update_sprites.iter() {
        if let Ok((sprite,rect)) = params.sprites.get(entity) {
            if let Some(atlas) = sprite.atlas.as_ref().map(|h| params.spritesheets.get(&h.id).unwrap()) {
               if let Some(render2d) = sprite.build_render(rect,atlas,params.ui_roots.baseui.clone()) {
                    if let Ok(mut render) = params.render2d.get_mut(entity) {
                        render.mesh2d = render2d.mesh2d;
                        render.texture = render2d.texture;
                    } else {
                        params.commands.entity(entity).insert(render2d);
                    }
               }
            }

            if let Some(top_canvas_entity) = find_top_canvas(entity, &params.parents, &params.canvases) {
                top_changed_canvas.insert(top_canvas_entity);
            }
        }
    }
    
    //更新Text的Mesh
    for entity in params.update_texts.iter() {
        if let Ok((text,rect)) = params.texts.get(entity) {
            if let Some(h_font) = text.font.as_ref() {
                //更新字体缓存
                if !params.ui_roots.font_caches.contains_key(h_font) {
                   params.font_assets.get(&h_font.id).map(|font| {
                      let font_id = params.ui_roots.text_brush.add_font(font.asset.clone());
                      params.ui_roots.font_caches.insert(h_font.clone(),font_id);
                   });
                }
               
                let section = text.build_section(rect);
                params.ui_roots.text_brush.queue(section);
            }
            let font_texture = params.textures.get_mut(&params.ui_roots.font_texture.id).unwrap();
            
            let action = params.ui_roots.text_brush.process_queued(|r,bytes| {
                write_font_texture(font_texture,r,bytes);
                params.write_font_atlas.send(WriteFontAtlas { rect:r });
            },glyph_to_mesh);
            match action {
                Ok(BrushAction::Draw(verts)) => {
                   let mesh2d = Text::build_mesh(verts,text.color);
                   if let Ok(mut render) = params.render2d.get_mut(entity) {
                      render.texture = Some(params.ui_roots.font_texture.clone());
                      render.mesh2d = mesh2d;
                    } else {
                        let render2d = UIRender2D {
                            mat_def:params.ui_roots.basetext.clone(),
                            texture:Some(params.ui_roots.font_texture.clone()),
                            mesh2d,
                            custom_mat:None
                        };
                        params.commands.entity(entity).insert(render2d);
                    }
                }
                Ok(BrushAction::ReDraw) => {}
                Err(err) => {
                    log::error!("text brush error:{:?}",err);
                }
            }

            if let Some(top_canvas_entity) = find_top_canvas(entity, &params.parents, &params.canvases) {
                top_changed_canvas.insert(top_canvas_entity);
            }
        }
    }
    //TODO 这里如果是已有渲染元素单纯改变层级，ZOrder刷新会有问题
    //刷新ZOrder
    if !top_changed_canvas.is_empty() {
        for top_entity in top_changed_canvas {
            let start_z = params.zorders.get(top_entity).map(|z| z.value).unwrap_or(0);
            ZOrder::update(start_z,top_entity, &mut params.zorders, &params.children,&mut params.commands);
        }
    }
}

#[derive(SystemParam)]
pub struct CanvasRenderParams<'w,'s> {
    pub(crate) update_render2ds:Query<'w,'s,Entity,Changed<UIRender2D>>,
    pub(crate) update_trans:Query<'w,'s,Entity,Changed<Transform>>,
    pub(crate) tree_events:EventReader<'w,'s,HierarchyEvent>,
    pub(crate) render2d:Query<'w,'s,&'static UIRender2D>,
    pub(crate) canvases:Query<'w,'s,&'static mut Canvas>,
    pub(crate) parents:Query<'w,'s,&'static Parent>,
    pub(crate) zorders:Query<'w,'s,&'static ZOrder>,
    pub(crate) transforms:Query<'w,'s,&'static Transform>,
    pub(crate) children:Query<'w,'s,&'static Children>,
    pub(crate) meshes:ResMut<'w,Assets<Mesh>>,
    pub(crate) materials:ResMut<'w,Assets<Material>>,
    pub(crate) asset_server:Res<'w,AssetServer>,
    pub(crate) commands:Commands<'w,'s>,
    pub(crate) ui_roots:ResMut<'w,UIRenderRoot>,
    pub(crate) changed_infos:Query<'w,'s,(Entity,ChangeTrackers<EStateInfo>)>,
    pub(crate) state_infos:Query<'w,'s,&'static EStateInfo>
}

pub fn update_canvas_render(mut params:CanvasRenderParams) {
   
    
    let mut changed_canvas:HashSet<Entity> = HashSet::default();
    //处理hierarchy事件
    for event in params.tree_events.iter() {
        match event {
            HierarchyEvent::SetParent { entity,.. } => {
                visit_children(*entity, &params.children, &mut |ve: Entity| {
                    if let Some(canvas_entity) = params.ui_roots.entity2canvas.remove(&ve) {
                        changed_canvas.insert(canvas_entity);
                    }
                    if let Some(canvas_entity) = find_canvas(ve, &params.parents, &params.canvases) {
                        changed_canvas.insert(canvas_entity);
                    }
                });
            },
            HierarchyEvent::Delete(cur_entity,all_entity,_) => {
                for delete_entity in all_entity.iter() {
                    if let Some(canvas_entity) = params.ui_roots.entity2canvas.remove(delete_entity) {
                        changed_canvas.insert(canvas_entity);
                    }
                }
                if let Some(canvas_entity) = find_canvas(*cur_entity, &params.parents, &params.canvases) {
                    changed_canvas.insert(canvas_entity);
                }
            }
            _ => {}
        }
    }

    //处理Active变化
    for (entity,track) in params.changed_infos.iter() {
        if track.is_changed() && !track.is_added() {
            if let Some(canvas_entity) = find_canvas(entity, &params.parents, &params.canvases) {
                changed_canvas.insert(canvas_entity);
            }
        }
    }
   
    //处理Transform变化
    for entity in params.update_trans.iter() {
        if !params.canvases.contains(entity) {
            if let Some(canvas_entity) = find_canvas(entity, &params.parents, &params.canvases) {
                changed_canvas.insert(canvas_entity);
            }
        } 
    }

    //处理渲染元素更新
    for entity in params.update_render2ds.iter() {
        if let Some(canvas_entity) = find_canvas(entity, &params.parents, &params.canvases) {
            changed_canvas.insert(canvas_entity);
        }
    }

    for entity in changed_canvas.iter() {
        Canvas::update_drawcall(*entity,&params.state_infos,
             &params.children,
             &mut params.render2d,
             &mut params.canvases,
             &params.zorders,
             &params.transforms,
             &params.parents,
             &mut params.meshes,
             &mut params.materials,
             &mut params.commands,
             &mut params.ui_roots,
             &params.asset_server);
    }
    for del_entity in params.ui_roots.despawn_next_frame.drain(..) {
        params.commands.entity(del_entity).despawn();
    }
}


pub fn update_canvas_trans(world:&mut World) {
   let mut update_canvas:Vec<Entity> = Vec::new();
   let mut update_trans = world.query_filtered::<(Entity,&Canvas),Or<(Changed<Transform>,Changed<ZOrder>)>>();
   for (entity,_) in update_trans.iter(world) {
        update_canvas.push(entity);
   }
   //这里已经过了Transform的system了，所以如果修改位置需要修改Global
   //因为drawcall必然没有子节点，所以直接修改drawcall的Global位置是没问题的
   let mut canvaes = world.query::<&Canvas>();
   let mut zorders = world.query::<&ZOrder>();
   let mut trans = world.query::<&mut Transform>();
   for entity in update_canvas.iter() {
     if let Ok(canvas) = canvaes.get(world, *entity) {
        if let Ok(canvas_t) = trans.get(world, *entity) {
            for draw_call in canvas.draw_calls.iter() {
                if let Ok(mut drawcall_t) = unsafe { trans.get_unchecked(world, draw_call.entity) } {
                    if let Ok(zorder) = zorders.get(world,draw_call.fst_entity) {
                        drawcall_t.local.position.x = canvas_t.global().position.x;
                        drawcall_t.local.position.y = canvas_t.global().position.y;
                        drawcall_t.global.position.x = canvas_t.global().position.x;
                        drawcall_t.global.position.y = canvas_t.global().position.y;
                        let zvalue = canvas_t.global().position.z + zorder.value as f32 * Z_SCALE;
                        drawcall_t.local.position.z = zvalue;
                        drawcall_t.global.position.z = zvalue;
                    }
                }
            }
        }
     }
   }
}


#[derive(SystemParam)]
pub struct ClipParams<'w,'s> {
    pub(crate) children:Query<'w,'s,&'static Children>,
    pub(crate) update_trans:Query<'w,'s,Entity,Changed<Transform>>,
    pub(crate) add_canvas:Query<'w,'s,Entity,Changed<Canvas>>,
    pub(crate) infos:Query<'w,'s,(&'static Canvas,&'static Transform,&'static Rect2D)>,
    pub(crate) parents:Query<'w,'s,&'static Parent>,
    pub(crate) hmats:Query<'w,'s,&'static Handle<Material>>,
    pub(crate) materials:ResMut<'w,Assets<Material>>,
}

pub(crate) fn update_ui_clips(mut params:ClipParams) {
    let mut changed_clip_canvas:HashSet<Entity> = HashSet::new();
    for entity in params.add_canvas.iter() {
        visit_children(entity, &params.children, &mut |ve:Entity| {
            if params.infos.contains(ve) {
               changed_clip_canvas.insert(ve);
            }
         });
    }
    for entity in params.update_trans.iter() {
      visit_children(entity, &params.children, &mut |ve:Entity| {
         if params.infos.contains(ve) {
            changed_clip_canvas.insert(ve);
         }
      });
    }
    
    for entity in changed_clip_canvas.iter() {
        if let Some(cur_box) = calc_box2d(*entity, &params) {
            if let Ok((canvas,_,_)) = params.infos.get(*entity) {
                for drawcall in canvas.draw_calls.iter() {
                    if let Ok(hmat) = params.hmats.get(drawcall.entity) {
                       if let Some(mat) = params.materials.get_mut(&hmat.id) {
                        let clip_rect = Vec4::new(cur_box.lt.x, cur_box.lt.y, cur_box.rb.x, cur_box.rb.y);
                        //println!("set clipRect:{:?} {:?}",entity,drawcall.entity);
                        mat.props.set_i32("isClip", 1, 0);
                        mat.props.set_float4("clipRect",clip_rect , 0);
                        //mat.props.set_float4("color", Vec4::new(1f32, 0f32, 0f32, 1f32), 0);
                       }
                    }
                }
            }
        }
    }
}

fn calc_box2d(entity:Entity,params:&ClipParams) -> Option<Box2D> {
    let mut cur_entity = Some(entity);
    let mut cur_box = Box2D::max();
    let mut has_clip = false;
    while let Some(loop_entity) = cur_entity {
        if let Ok((canvas,t,rect)) = params.infos.get(loop_entity) {
            if canvas.is_clip {
                let mut lt = Vec3::new(rect.left(),rect.top(),1f32);
                let mut rb = Vec3::new(rect.right(),rect.bottom(),1f32);
                lt = t.global().mul_vec3(lt);
                rb = t.global().mul_vec3(rb);
                let now_box = Box2D::new(lt.x,lt.y,rb.x,rb.y);
                cur_box = cur_box.intersection(&now_box);
                has_clip = true;
            }
        }
        cur_entity = params.parents.get(loop_entity).ok().map(|v| v.0);
    }
    if has_clip { Some(cur_box) } else { None }
}

fn find_top_canvas(entity:Entity,parents:&Query<&Parent>,canvases:&Query<&Canvas>) -> Option<Entity> {
    let mut cur_entity = Some(entity);
    let mut last_canvas:Option<Entity> = None;
    while let Some(entity) = cur_entity {
        if canvases.contains(entity) {
            last_canvas = Some(entity);
        }
        if let Ok(parent) = parents.get(entity) {
            cur_entity = Some(parent.0);
        } else {
            cur_entity = None;
        }   
    }
    last_canvas
}

fn find_canvas(entity:Entity,parents:&Query<&Parent>,canvases:&Query<&mut Canvas>) -> Option<Entity> {
    let mut cur_entity = Some(entity);
    while let Some(entity) = cur_entity {
        if canvases.contains(entity) {
            return Some(entity);
        }
        if let Ok(parent) = parents.get(entity) {
            cur_entity = Some(parent.0);
        } else {
            cur_entity = None;
        }   
    }
    None
}

fn visit_children<F>(entity:Entity,children:&Query<&Children>,visit:&mut F) where F:FnMut(Entity) {
    visit(entity);
    if let Ok(childs) = children.get(entity) {
        for child in childs.iter() {
            visit_children(*child,children,visit);
        }
    }
}