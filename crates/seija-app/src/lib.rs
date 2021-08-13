mod app;
pub use app::{App};

pub trait IModule {
    fn init(&mut self,app:&mut App);
}