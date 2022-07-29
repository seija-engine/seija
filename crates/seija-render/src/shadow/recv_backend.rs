use crate::{UniformInfo, uniforms::UniformContext, UniformIndex};
use anyhow::{Result,anyhow};
#[derive(Default)]
pub struct ShadowRecvBackend {
    name_index:UniformIndex,
    bias_index:usize,
    strength_index:usize,
    shadow_map_index:usize
}

impl ShadowRecvBackend {
    pub fn from_name(name:&str,ubo_ctx:&UniformContext) -> Result<ShadowRecvBackend> {
        let recv_info = ubo_ctx.info.get_info(name).ok_or(anyhow!("not found info {}",name))?;

        let bias_index = recv_info.props.get_offset("bias", 0).ok_or(anyhow!("bias"))?;
        let strength_index = recv_info.props.get_offset("strength", 0).ok_or(anyhow!("strength"))?;
       
        let name_index = ubo_ctx.get_index(name).ok_or(anyhow!("err ubo name {}",name))?;
        Ok(ShadowRecvBackend {
            name_index,
            bias_index,
            strength_index,
            shadow_map_index:0
        })
    }

    pub fn set_bias(&self,ubo_ctx:&mut UniformContext,bias:f32) {
        ubo_ctx.set_buffer(&self.name_index, None, |buffer| {
            buffer.buffer.write_bytes(self.bias_index, bias);
        });
    }

    pub fn set_strength(&self,ubo_ctx:&mut UniformContext,strength:f32) {
        ubo_ctx.set_buffer(&self.name_index, None, |buffer| {
            buffer.buffer.write_bytes(self.strength_index, strength);
        });
    }
}