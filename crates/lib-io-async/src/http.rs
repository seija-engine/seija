use reqwest::{Response, Error};
use std::ffi::CStr;

use crate::ffi::{AsyncRuntime, AsyncEvent};

#[no_mangle]
pub unsafe extern "C" fn easy_async_http_get(runtime:&mut AsyncRuntime,uri:*const i8) {
   let c_uri = CStr::from_ptr(uri).to_str().unwrap();
   let resp_future = reqwest::get(c_uri);
   let clone_sender = runtime.sender.clone();
   tokio::spawn(async move {
     let resp:Result<Response,Error> = resp_future.await;
     let _ = clone_sender.send(AsyncEvent::HttpResp(resp));
   });
}



