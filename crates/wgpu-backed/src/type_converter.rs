use seija_render::core::resource::buffer::BufferUsage;

pub trait WgpuFrom<T> {
    fn from(val: T) -> Self;
}

pub trait WgpuInto<U> {
    fn wgpu_into(self) -> U;
}

impl<T, U> WgpuInto<U> for T where U: WgpuFrom<T> {
    fn wgpu_into(self) -> U {
        U::from(self)
    }
}

impl WgpuFrom<BufferUsage> for wgpu::BufferUsage {
    fn from(val: BufferUsage) -> Self {
        wgpu::BufferUsage::from_bits(val.bits()).unwrap()
    }
}