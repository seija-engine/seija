use reqwest::{Response, Error, Method};
use std::ffi::CStr;
use reqwest::{Client};
use crate::runtime::{AsyncRuntime, AsyncEvent};

#[no_mangle]
pub unsafe extern "C" fn _ia_easy_http_get(runtime:&mut AsyncRuntime,uri:*const i8) {
   let c_uri = CStr::from_ptr(uri).to_str().unwrap();
   let resp_future = reqwest::get(c_uri);
   let clone_sender = runtime.sender.clone();
   let hh = tokio::spawn(async move {
     let resp:Result<Response,Error> = resp_future.await;
     let _ = clone_sender.send(AsyncEvent::HttpResp(resp));
   });
   
}


#[no_mangle]
pub extern "C" fn _ia_create_http_client() -> *mut Client {
  Box::into_raw(Box::new(Client::default()))
}

#[repr(C)]
struct uv_http_handle {

}

#[test]
fn test_ptr() {
  let hh = tokio::spawn(async move {});
  
}