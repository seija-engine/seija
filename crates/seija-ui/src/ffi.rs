use bevy_ecs::{prelude::{Entity, Events}, world::World, event::ManualEventReader};
use num_enum::FromPrimitive;
use seija_app::App;
use seija_asset::{AssetServer, Handle, HandleId};
use seija_core::{math::{Vec4, Vec2}, TypeUuid};
use seija_render::RenderConfig;
use spritesheet::SpriteSheet;

use crate::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    event::{UIEventSystem, EventNode, UIEvent},
    types::Thickness,
    update_ui_render, UIModule, layout::{comps::{Orientation, StackLayout, FlexLayout, FlexItem}, types::{LayoutElement, CommonView, UISize, SizeValue, TypeElement, FreeLayoutItem}}, text::{Text, Font},
};

#[no_mangle]
pub unsafe extern "C" fn ui_add_module(app_ptr: &mut App) {
    app_ptr.add_module(UIModule);
}

#[no_mangle]
pub unsafe extern "C" fn render_config_set_ui(config: &mut RenderConfig) {
    config.pre_render_updates = vec![update_ui_render];
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_ui_canvas(world: &mut World, entity_id: u64, typ: u8) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(UICanvas {
        scale_mode: typ.into(),
    });
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_ui_system(world: &mut World, entity_id: u64) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(UIEventSystem::default());
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_rect2d(world: &mut World, entity_id: u64, rect: &mut Rect2D) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(rect.clone());
}

#[no_mangle]
pub unsafe extern "C" fn entity_get_rect2d(world: &mut World, entity_id: u64,is_mut:bool) -> *mut Rect2D {
    let entity = Entity::from_bits(entity_id);
    if is_mut {
        match world.entity_mut(entity).get_mut::<Rect2D>() {
            None => std::ptr::null_mut(),
            Some(mut v) => v.as_mut() as *mut Rect2D 
        }
    } else {
        match world.entity(entity).get::<Rect2D>() {
            None => std::ptr::null_mut(),
            Some(v) => v as *const Rect2D as *mut Rect2D 
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_canvas(world: &mut World, entity_id: u64,is_clip:bool) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(Canvas::new(is_clip));
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_sprite_simple(
    world: &mut World,
    entity_id: u64,
    index: i32,
    atlas_id: u64,
    color: &mut Vec4,
) {
    let handle_id = HandleId::new(SpriteSheet::TYPE_UUID, atlas_id);
    let ref_sender = world.get_resource::<AssetServer>().clone().unwrap().get_ref_sender();
    let handle = if index >= 0 {
        Some(Handle::strong(handle_id, ref_sender))
    } else {
        None
    };
    let entity = Entity::from_bits(entity_id);
    let sprite = Sprite::simple(index as usize, handle, color.clone());
    world.entity_mut(entity).insert(sprite);
}


#[no_mangle]
pub unsafe extern "C" fn entity_add_sprite_slice(
    world: &mut World,
    entity_id: u64,
    index: i32,
    atlas_id: u64,
    thickness: &mut Thickness,
    color: &mut Vec4,
) {
    let handle_id = HandleId::new(SpriteSheet::TYPE_UUID, atlas_id);
    let ref_sender = world.get_resource::<AssetServer>().clone().unwrap().get_ref_sender();
    let handle = if index >= 0 {
        Some(Handle::strong(handle_id, ref_sender))
    } else {
        None
    };
    let entity = Entity::from_bits(entity_id);

    let sprite = Sprite::sliced(
        index as usize,
        handle,
        thickness.clone(),
        color.clone(),
    );
    world.entity_mut(entity).insert(sprite);
}

#[no_mangle]
pub unsafe extern "C" fn entity_get_sprite(world: &mut World,entity_id:u64,is_mut:bool) -> *mut Sprite {
    let entity = Entity::from_bits(entity_id);
    if !is_mut {
        if let Some(sprite) = world.get::<Sprite>(entity) {
            return sprite as *const Sprite as *mut Sprite;
        }
    } else {
        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            return sprite.as_mut() as *mut Sprite;
        }
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn sprite_set_sprite(world: &mut World,sprite:&mut Sprite,index: i32,atlas_id: u64) {
    if index < 0 {
        sprite.atlas = None;
        sprite.sprite_index = 0;
        return;
    }
    let handle_id = HandleId::new(SpriteSheet::TYPE_UUID, atlas_id);
    let ref_sender = world.get_resource::<AssetServer>().clone().unwrap().get_ref_sender();
    let atlas = Handle::strong(handle_id, ref_sender);
    sprite.atlas = Some(atlas);
    sprite.sprite_index = index as usize;
}

#[no_mangle]
pub unsafe extern "C" fn sprite_set_color(sprite:&mut Sprite,color:&Vec4) {
    //println!("sprite set color {:?}",color);
    sprite.info.color = color.clone();
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_event_node(world: &mut World,entity_id:u64,node:&EventNode) {
    let entity = Entity::from_bits(entity_id);
    let mut event_node = EventNode::default();
    event_node.event_type = node.event_type;
    event_node.stop_bubble = node.stop_bubble;
    event_node.stop_capture = node.stop_capture;
    event_node.use_capture = node.use_capture;
    world.entity_mut(entity).insert(event_node);
}

#[no_mangle]
pub unsafe extern "C" fn entity_remove_event_node(world: &mut World, entity_id: u64) -> bool {
    let entity = Entity::from_bits(entity_id);
    if let Some(mut entity_mut) = world.get_entity_mut(entity) {
       return entity_mut.remove::<EventNode>().is_some();
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn read_ui_events(world: &mut World,f:extern fn(entity:u64,typ:u32,user_key_ptr:*const i8)) {
    let events = world.get_resource_mut::<Events<UIEvent>>().unwrap();
    let mut reader:ManualEventReader<UIEvent> = events.get_reader();
    for event in reader.iter(&events) {
        if let Some(user_key) = &event.user_key {
           //TODO 考虑优化这里
           let c_string = std::ffi::CString::new(user_key.as_str()).unwrap();
           f(event.entity.to_bits(),event.event_type.bits(),c_string.as_ptr());
        } else {
            f(event.entity.to_bits(),event.event_type.bits(),std::ptr::null());
        }
       
    }
}

////////layout/////////
#[repr(C)]
pub struct FFIUISize {
    typ_width:u8,
    typ_height:u8,
    width:f32,
    height:f32
}

impl Into<UISize> for &FFIUISize {
    fn into(self) -> UISize {
        let width = match self.typ_width {
            0 => SizeValue::Auto,
            1 => SizeValue::PixelFromRect,
            _ => SizeValue::Pixel(self.width)
        };
        let height = match self.typ_height {
            0 => SizeValue::Auto,
            1 => SizeValue::PixelFromRect,
            _ => SizeValue::Pixel(self.height)
        };
        UISize { width , height }
    }
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_stack(world: &mut World,entity_id:u64,spacing:f32,ori:u8,view:&CommonView,ui_size:&FFIUISize) {
    let orientation = Orientation::from_primitive(ori);
    let mut layout = LayoutElement::create_stack(spacing, orientation);
    layout.common.margin = view.margin.clone();
    layout.common.padding = view.padding.clone();
    layout.common.use_rect_size = view.use_rect_size;
    layout.common.hor = view.hor;
    layout.common.ver = view.ver;
    layout.common.ui_size = ui_size.into();
    log::info!("view:{:?}",&layout.common);
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(layout);
}

#[no_mangle]
pub unsafe extern "C" fn entity_get_stack(world: &mut World,entity_id:u64) -> *mut StackLayout {
    let entity = Entity::from_bits(entity_id);
    let mut emut = world.entity_mut(entity);
    let elem = emut.get_mut::<LayoutElement>();
    match elem  {
        Some(mut v) => {
            match &mut v.typ_elem {
                TypeElement::Stack(stack) => stack as *mut StackLayout,
                _ => std::ptr::null_mut()
            }
        },
        None => std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_free_layout(world: &mut World,entity_id:u64,view:&CommonView,ui_size:&FFIUISize) {
    let mut layout = LayoutElement::create_free();
    layout.common.margin = view.margin.clone();
    layout.common.padding = view.padding.clone();
    layout.common.use_rect_size = view.use_rect_size;
    layout.common.hor = view.hor;
    layout.common.ver = view.ver;
    layout.common.ui_size = ui_size.into();
    log::info!("free layout:{:?}",&layout.common);
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(layout);
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_layout_freeitem(world: &mut World,entity_id:u64,x:f32,y:f32) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(FreeLayoutItem { pos:Vec2::new(x, y) });
}

#[no_mangle]
pub unsafe extern "C" fn entity_get_layout_freeitem(world: &mut World,entity_id:u64) -> *mut FreeLayoutItem {
    let entity = Entity::from_bits(entity_id);
    if let Some(mut item) = world.entity_mut(entity).get_mut::<FreeLayoutItem>() {
        return item.as_mut() as *mut FreeLayoutItem;
    }
    return std::ptr::null_mut();
}


#[no_mangle]
pub unsafe extern "C" fn entity_add_commonview(world: &mut World,entity_id:u64,view:&CommonView,ui_size:&FFIUISize) {
    let mut layout = LayoutElement::create_view();
    layout.common.margin = view.margin.clone();
    layout.common.padding = view.padding.clone();
    layout.common.use_rect_size = view.use_rect_size;
    layout.common.hor = view.hor;
    layout.common.ver = view.ver;
    layout.common.ui_size = ui_size.into();
    log::info!("add commonview:{:?}",&layout.common);
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(layout);
}

#[no_mangle]
pub unsafe extern "C" fn entity_get_commonview(world: &mut World,entity_id:u64) -> *mut LayoutElement {
    let entity = Entity::from_bits(entity_id);
    let mut emut = world.entity_mut(entity);
    let elem = emut.get_mut::<LayoutElement>();
    match elem {
        Some(mut v) => v.as_mut() as *mut LayoutElement,
        None => std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn entity_set_layout_size_w(view:&mut CommonView,typ:u8,value:f32) {
    match typ {
        0 => view.ui_size.width = SizeValue::Auto,
        1 => view.ui_size.width = SizeValue::PixelFromRect,
        _ => view.ui_size.width = SizeValue::Pixel(value)
    }
}

#[no_mangle]
pub unsafe extern "C" fn entity_set_layout_size_h(view:&mut CommonView,typ:u8,value:f32) {
    match typ {
        0 => view.ui_size.height = SizeValue::Auto,
        1 => view.ui_size.height = SizeValue::PixelFromRect,
        _ => view.ui_size.height = SizeValue::Pixel(value)
    }
}


#[no_mangle]
pub unsafe extern "C" fn entity_add_flex(world: &mut World,entity_id:u64,view:&CommonView,ui_size:&FFIUISize,flex:&FlexLayout) {
    let mut layout = LayoutElement::create_flex(flex.clone());
    layout.common.margin = view.margin.clone();
    layout.common.padding = view.padding.clone();
    layout.common.use_rect_size = view.use_rect_size;
    layout.common.hor = view.hor;
    layout.common.ver = view.ver;
    layout.common.ui_size = ui_size.into();
    let entity = Entity::from_bits(entity_id);
    log::info!("add flex:{:?}",&flex);
    world.entity_mut(entity).insert(layout);
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_flexitem(world: &mut World,entity_id: u64,item:&FlexItem) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(item.clone());
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_text(world: &mut World,entity_id: u64,text:&Text,size:i32,text_str:*mut i8,font_id:u64) {
    let entity = Entity::from_bits(entity_id);
    let text_string = std::ffi::CStr::from_ptr(text_str).to_str().unwrap();
    let ref_sender = world.get_resource::<AssetServer>().clone().unwrap().get_ref_sender();
    let new_text:Text = Text {
        anchor:text.anchor,
        color:text.color.clone(),
        font_size: size as u32,
        text:text_string.into(),
        line_mode: text.line_mode,
        font:Some(Handle::strong(HandleId::new(Font::TYPE_UUID, font_id), ref_sender)),
        is_auto_size: text.is_auto_size
    };
    world.entity_mut(entity).insert(new_text);
}

#[no_mangle]
pub unsafe extern "C" fn entity_get_text(world: &mut World,entity_id: u64) -> *mut Text {
    let entity = Entity::from_bits(entity_id);
    let mut emut = world.entity_mut(entity);
    let text_mut = emut.get_mut::<Text>();
    match text_mut {
        Some(mut ptr) => ptr.as_mut() as *mut Text,
        None => std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn entity_text_setstring(text_mut:&mut Text,cstr:*mut i8) {
    let text_string = std::ffi::CStr::from_ptr(cstr).to_str().unwrap();
    text_mut.text = text_string.into();
}