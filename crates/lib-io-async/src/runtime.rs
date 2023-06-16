use tokio::runtime::Runtime;
use crossbeam_channel::{Sender,Receiver,unbounded};
use crate::http::{HttpEnv, AsyncHttpEvent};
pub struct AsyncRuntime {
   pub(crate) tokio_runtime:Runtime,
   pub receiver:Receiver<AsyncEvent>,
   pub sender:Sender<AsyncEvent>,

   pub(crate) _http_env:HttpEnv,
}

#[derive(Debug)]
pub enum AsyncEvent {
    Http(AsyncHttpEvent)
}

#[no_mangle]
pub extern "C" fn _ia_create_runtime() -> *mut AsyncRuntime {
   let (sender,receiver) = unbounded::<AsyncEvent>();
   let tokio_runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
   Box::into_raw(Box::new(AsyncRuntime {
      tokio_runtime:tokio_runtime,
      sender,
      receiver,
      _http_env:HttpEnv::new()
   }))
}



#[no_mangle]
pub unsafe extern "C" fn _ia_runtime_poll_event(runtime:&mut AsyncRuntime) {
   for event in runtime.receiver.try_iter() {
      match event {
          AsyncEvent::Http(http_event) => { runtime._http_env.on_async_event(http_event) }
      }
   }
}