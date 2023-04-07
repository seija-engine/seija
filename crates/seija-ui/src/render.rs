use std::{sync::Arc, num::NonZeroU32};
use glyph_brush::Rectangle;
use bevy_ecs::{world::World, prelude::{Component, Events}};
use seija_asset::{Handle, Assets};
use seija_render::{RenderContext, resource::{Texture, BufferId, RenderResources, RenderResourceId}, material::MaterialDef,wgpu};
use crate::{mesh2d::Mesh2D, system::UIRenderRoot};

#[derive(Component,Debug)]
pub struct UIRender2D {
   pub mat:Arc<MaterialDef>,
   pub texture:Handle<Texture>,
   pub mesh2d:Mesh2D,
}

#[derive(Clone, Copy)]
pub struct WriteFontAtlas {
    pub(crate) rect:Rectangle<u32>
}

pub fn update_ui_render(world:&mut World,ctx:&mut RenderContext) {
    let font_buffer_id = check_init_font_buffer(world, ctx);
    let mut write_atlas = world.get_resource_mut::<Events<WriteFontAtlas>>().unwrap();
    let write_events = write_atlas.drain().collect::<Vec<_>>();
    let textures = world.get_resource::<Assets<Texture>>().unwrap();
    let render_root = world.get_resource::<UIRenderRoot>().unwrap();
    let font_texture = textures.get(&render_root.font_texture.id).unwrap();
    let font_texture_size = font_texture.desc().desc.size;
    let cache_bytes = font_texture.cast_image_data().unwrap();
    let texture_id = ctx.resources.get_render_resource(&render_root.font_texture.id, 0).and_then(|v| v.into_texture_id()).unwrap();
    

    ctx.resources.map_buffer(&font_buffer_id, wgpu::MapMode::Write);
    ctx.resources.write_mapped_buffer(&font_buffer_id, 0..cache_bytes.len() as u64, &mut |bytes,_| {
        bytes[0..cache_bytes.len()].copy_from_slice(cache_bytes);
    });
    ctx.resources.unmap_buffer(&font_buffer_id);

    let command = ctx.command_encoder.as_mut().unwrap();
    let aligned_width = RenderResources::get_aligned_texture_size(1024);
    ctx.resources.copy_buffer_to_texture(command,font_buffer_id,0,
                                         NonZeroU32::new((1 * aligned_width) as u32).unwrap(), 
                                         &texture_id,wgpu::Origin3d::default(),0,font_texture_size,None)
}

fn check_init_font_buffer(world:&mut World,ctx:&mut RenderContext) -> BufferId {
    let mut render_root = world.get_resource_mut::<UIRenderRoot>().unwrap();
    if let Some(buffer_id) = render_root.font_buffer.as_ref() {
        return buffer_id.clone();
    }
    let buffer_id = ctx.resources.create_buffer(&wgpu::BufferDescriptor {
        label: Some("UI Font Buffer"),
        size: 1024 * 1024,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
        mapped_at_creation: false,
    });
    render_root.font_buffer = Some(buffer_id.clone());
    buffer_id
}