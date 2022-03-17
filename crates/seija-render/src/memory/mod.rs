mod uniform_buffer_def;
mod uniform_buffer;

pub use uniform_buffer::{TypedUniformBuffer,UniformBuffer};
pub use uniform_buffer_def::{UniformBufferDef,PropInfo,PropInfoList};

pub fn align_num_to(num:u64,align:u64) -> u64 {
    let mut align_num = align;
    while num > align_num {
        align_num += align;
    }
    align_num
}