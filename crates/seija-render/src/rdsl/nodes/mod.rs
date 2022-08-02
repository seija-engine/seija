mod transform_node;
mod window_resize_node;
mod camera_node;
mod draw_pass_node;
mod draw_quad;
use wgpu::{CommandEncoder, Color};
pub use window_resize_node::{WindowReSizeNode};
pub use camera_node::{CameraNode};
pub use transform_node::{TransfromNode};
pub use draw_pass_node::{DrawPassNode};
pub use draw_quad::{DrawQuadNode};

use crate::resource::{RenderResources, RenderResourceId};

use super::atom::Atom;

#[derive(Debug,PartialEq, Eq)] 
pub enum CommonError {
    ErrArg,
    ErrInput(usize),
    ErrTargetView,
    ErrDepthView,
    MissMesh,
    MissMaterial,
    ErrUBOIndex,
    TextureNotReady
}

pub fn create_render_pass<'a>(textures:&Vec<*mut Atom<RenderResourceId>>,
                              operations:wgpu::Operations<Color>,
                              depth:Option<*mut Atom<RenderResourceId>>,
                              res:&'a RenderResources,
                              command:&'a mut CommandEncoder) -> Result<wgpu::RenderPass<'a>,CommonError> {
    let mut color_attachments:Vec<wgpu::RenderPassColorAttachment> = vec![];
    for atom in textures.iter() {
        let atom_ref = unsafe { &**atom };
        if !res.is_ready(atom_ref.inner()) {
            return Err(CommonError::TextureNotReady);
        }
        let texture = res.get_texture_view_by_resid(atom_ref.inner()).ok_or(CommonError::ErrTargetView)?;
        color_attachments.push(wgpu::RenderPassColorAttachment {
            view:texture,
            resolve_target:None,
            ops:operations
        });
    }
    let mut depth_view:Option<wgpu::RenderPassDepthStencilAttachment> = None;
    if let Some(atom_depth) = depth {
        let atom_ref = unsafe { &*atom_depth };
        if !res.is_ready(atom_ref.inner()) {
            return Err(CommonError::TextureNotReady);
        }
        let texture_view = res.get_texture_view_by_resid(atom_ref.inner()).ok_or(CommonError::ErrDepthView)?;
        depth_view = Some(wgpu::RenderPassDepthStencilAttachment {
            view:texture_view,
            stencil_ops: None,
            depth_ops: Some(wgpu::Operations {
                load:  wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
        });
    }
    let pass_desc = wgpu::RenderPassDescriptor {
        label:None,
        color_attachments:color_attachments.as_slice(),
        depth_stencil_attachment:depth_view
    };
    let pass = command.begin_render_pass(&pass_desc);
    Ok(pass)
}