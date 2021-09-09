pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

pub unsafe trait Byteable where Self: Sized { }

impl<T> AsBytes for T where T: Byteable {
    fn as_bytes(&self) -> &[u8] {
        let len = std::mem::size_of_val(self);
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, len) }
    }
}

impl<'a, T> AsBytes for [T] where T: Byteable {
    fn as_bytes(&self) -> &[u8] {
        let len = std::mem::size_of_val(self);
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, len) }
    }
}

unsafe impl<T> Byteable for [T] where  Self: Sized, T: Byteable { }
unsafe impl<T> Byteable for [T; 2] where T: Byteable {}
unsafe impl<T> Byteable for [T; 3] where T: Byteable {}
unsafe impl<T> Byteable for [T; 4] where T: Byteable {}
unsafe impl<T> Byteable for [T; 16] where T: Byteable {}


unsafe impl Byteable for u8 {}
unsafe impl Byteable for u16 {}
unsafe impl Byteable for u32 {}
unsafe impl Byteable for u64 {}
unsafe impl Byteable for usize {}
unsafe impl Byteable for i8 {}
unsafe impl Byteable for i16 {}
unsafe impl Byteable for i32 {}
unsafe impl Byteable for i64 {}
unsafe impl Byteable for isize {}
unsafe impl Byteable for f32 {}
unsafe impl Byteable for f64 {}