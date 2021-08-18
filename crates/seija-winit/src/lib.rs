mod window;
use seija_app::{IModule,App};
use winit::event_loop::EventLoop;
pub struct WinitModule;

impl IModule for WinitModule {

    fn init(&mut self,app:&mut App) {

    }

}


pub fn winit_runner(app: App) {
    let event_loop = EventLoop::new();
}
