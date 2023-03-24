use bevy_ecs::{world::World, prelude::Component};
use seija_asset::Handle;
use seija_render::{RenderContext, pipeline::render_bindings::{BindGroupLayoutBuilder, BindGroupBuilder}, resource::Texture};
use seija_render::wgpu;
use seija_core::{anyhow, OptionExt};
use seija_core::log;

use crate::mesh2d::Mesh2D;

#[derive(Component,Debug)]
pub struct UIRender2D {
   pub texture:Handle<Texture>,
   pub mesh2d:Mesh2D,
}

pub fn update_ui_render(world:&mut World,ctx:&mut RenderContext) {
    
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