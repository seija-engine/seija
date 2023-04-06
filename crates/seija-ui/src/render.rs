use std::sync::Arc;
use glyph_brush::Rectangle;
use bevy_ecs::{world::World, prelude::{Component, Events}};
use seija_asset::{Handle, Assets};
use seija_render::{RenderContext, resource::Texture, material::MaterialDef};
use crate::{mesh2d::Mesh2D, system::UIRenderRoot};

#[derive(Component,Debug)]
pub struct UIRender2D {
   pub mat:Arc<MaterialDef>,
   pub texture:Handle<Texture>,
   pub mesh2d:Mesh2D,
}

pub struct WriteFontAtlas {
    pub(crate) rect:Rectangle<u32>
}

pub fn update_ui_render(world:&mut World,ctx:&mut RenderContext) {
     let mut write_atlas = world.get_resource_mut::<Events<WriteFontAtlas>>().unwrap();
     let write_events = write_atlas.drain().collect::<Vec<_>>();
     let textures = world.get_resource::<Assets<Texture>>().unwrap();
     let render_root = world.get_resource::<UIRenderRoot>().unwrap();
     let font_texture = textures.get(&render_root.font_texture.id).unwrap();
     let cache_bytes = font_texture.cast_image_data().unwrap();

     let command = ctx.command_encoder.as_mut().unwrap();
     
     for event in write_events {
        
     }
}



/*
fn set_atlas_layout_and_bindgroup(sprite_alloc:&SpriteAllocator,ctx:&mut RenderContext) -> anyhow::Result<()> {
    let atlas_list = &sprite_alloc.atlas_list;
    
    let mut layout_builder = BindGroupLayoutBuilder::new();
    layout_builder.add_texture_array(atlas_list.len() as u32, 
                                     wgpu::ShaderStages::FRAGMENT, 
                                     wgpu::TextureSampleType::Float { filterable: true });
    layout_builder.add_sampler(true);
    let atlas_layout = layout_builder.build(&ctx.device);

    let mut bind_builder = BindGroupBuilder::new();
    let sample_id = ctx.resources.create_sampler(&wgpu::SamplerDescriptor::default());
    let mut atlas_textures = vec![];
    for atlas in atlas_list.iter() {
        if let Some(texture_id) = atlas.texture.as_ref() {
            atlas_textures.push(texture_id.clone());
        }
    }
    //atlas_textures[0] = atlas_textures[1];
    
    bind_builder.add_texture_array(atlas_textures, sample_id);
    let bind_group = bind_builder.build(&atlas_layout, &ctx.device,&ctx.resources);
   
    ctx.ubo_ctx.set_layout("UIAtlas", atlas_layout)?;
    let index = ctx.ubo_ctx.get_index("UIAtlas").get()?;
    ctx.ubo_ctx.set_bind_group(&index, None, bind_group);
    Ok(())
}*/