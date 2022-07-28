use crate::{UniformInfo};
#[derive(Default)]
pub struct ShadowRecvBackend {
    bias_index:usize,
    strength_index:usize,
    shadow_map_index:usize
}

impl ShadowRecvBackend {
    pub fn from_def(info:&UniformInfo) -> Result<Self,String> {
        let bias_index = info.props.get_offset("bias", 0).ok_or(String::from("bias"))?;
        let strength_index = info.props.get_offset("strength", 0).ok_or(String::from("strength"))?;
       

        Ok(ShadowRecvBackend {
            bias_index,
            strength_index,
            shadow_map_index:0
        })
    }
}