use glam::Vec3;

pub trait IAABB {
    fn min(&self) -> Vec3;
    fn max(&self) -> Vec3;

    #[inline]
    fn center(&self) -> Vec3 {
        let two = 2f32;
        self.min() + self.dim() / two
    }

    #[inline]
    fn dim(&self) -> Vec3 {
        self.max() - self.min()
    }
}