use std::num::NonZeroU32;
use seija_render::{RenderContext};
use seija_core::{log,anyhow::Result, OptionExt};
use crate::SpriteAllocator;
use seija_render::wgpu;

use super::atlas::DynamicAtlas;



pub fn update_sprite_alloc_render(ctx:&mut RenderContext,sprite_allocator:&mut SpriteAllocator) -> bool {
    let has_dirty = check_init_dyn_atlas(&mut sprite_allocator.atlas_list, ctx);
    
    for dyn_atlas in sprite_allocator.atlas_list.iter_mut() {
        if let Err(err) = process_sprite_atlas(dyn_atlas, ctx) {
            log::error!("{:?}",err);
        }
    }

    has_dirty
}

fn process_sprite_atlas(dyn_atlas:&mut DynamicAtlas,ctx:&mut RenderContext) -> Result<()> {
    
    let has_new_sprite = dyn_atlas.used_sprites.iter().any(|v| v.image.is_some());
    if !has_new_sprite { return Ok(()); }

    let desc = wgpu::TextureFormat::Rgba8Unorm.describe();
    let all_size = dyn_atlas.width * dyn_atlas.height * desc.block_size as u32;
    let buffer_id = dyn_atlas.cache_buffer.as_ref().get()?.clone();
    ctx.resources.map_buffer(&buffer_id, wgpu::MapMode::Write);
    for sprite in dyn_atlas.used_sprites.iter_mut() {
        if let Some(image_info) = sprite.image.take() {
           let block_size = image_info.format.describe().block_size as u64;
           ctx.resources.write_mapped_buffer(&buffer_id,0  .. all_size as u64,&mut |bytes,_| {

            for (index,row) in image_info.data.chunks_exact(desc.block_size as usize * image_info.width as usize).enumerate() {
                let mut offset = (index + sprite.rect.y as usize) * dyn_atlas.width as usize * block_size as usize;
                offset = offset + sprite.rect.x as usize * block_size as usize;
                bytes[offset..(offset + image_info.width as usize * block_size as usize)].copy_from_slice(row);
            }

           });
        }
    }
    ctx.resources.unmap_buffer(&buffer_id);

    let command = ctx.command_encoder.as_mut().get()?;
    let texture_id = dyn_atlas.texture.as_ref().get()?;
    
    ctx.resources.copy_buffer_to_texture(command, 
        buffer_id, 
        0,
        NonZeroU32::new(dyn_atlas.width * desc.block_size as u32).unwrap(),
        texture_id, 
        wgpu::Origin3d::default(), 
        0, 
        wgpu::Extent3d {width:dyn_atlas.width,height:dyn_atlas.height,depth_or_array_layers:1 }, 
        None);
    Ok(())
}

fn check_init_dyn_atlas(atlas_list:&mut Vec<DynamicAtlas>,ctx:&mut RenderContext) -> bool {
    let mut has_atlas_dirty = false;
    for dyn_atlas in atlas_list.iter_mut() {
        //create texture
        if dyn_atlas.texture.is_none() {
            let texture_desc = wgpu::TextureDescriptor {
                label:None,
                size:wgpu::Extent3d {
                    width:dyn_atlas.width,
                    height:dyn_atlas.height,
                    depth_or_array_layers:1
                },
                mip_level_count: 1,
                sample_count: 1, 
                dimension: wgpu::TextureDimension::D2, 
                format: wgpu::TextureFormat::Rgba8Unorm, 
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats:&[]
            };
            let mut view_desc = wgpu::TextureViewDescriptor::default();
            view_desc.dimension = Some(wgpu::TextureViewDimension::D2);
            let atlas_texture = ctx.resources.create_texture(&texture_desc, &view_desc);
            dyn_atlas.texture = Some(atlas_texture);
            has_atlas_dirty = true;
        }

        if dyn_atlas.cache_buffer.is_none() {
            let desc = wgpu::TextureFormat::Rgba8Unorm.describe();
            let buffer_size = dyn_atlas.width as u64 * dyn_atlas.height as u64 * desc.block_size as u64;
            let buffer_id = ctx.resources.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:buffer_size,
                usage:wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
                mapped_at_creation:false
            });
            dyn_atlas.cache_buffer = Some(buffer_id);
        }
    }
    has_atlas_dirty
   
}

