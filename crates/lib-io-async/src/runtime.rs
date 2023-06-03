use reqwest::{Response, Error};
use tokio::runtime::Runtime;
use crossbeam_channel::{Sender,Receiver,unbounded};
pub struct AsyncRuntime {
   _tokio_runtime:Runtime,
   pub receiver:Receiver<AsyncEvent>,
   pub sender:Sender<AsyncEvent>
}

pub enum AsyncEvent {
    HttpResp(Result<Response,Error>)
}

#[no_mangle]
pub extern "C" fn _ia_create_runtime() -> *mut AsyncRuntime {
   let (sender,receiver) = unbounded::<AsyncEvent>();
   let tokio_runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
   Box::into_raw(Box::new(AsyncRuntime {
      _tokio_runtime:tokio_runtime,
      sender,
      receiver
   }))
}



#[no_mangle]
pub extern "C" fn _ia_runtime_poll_event(runtime:&mut AsyncRuntime) {
   for event in runtime.receiver.iter() {
   }
}