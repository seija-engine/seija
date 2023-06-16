use reqwest::{Method, Url, Response};
use bytes::Bytes;
use std::ffi::CStr;
use reqwest::{Client,RequestBuilder,Error};
use crate::runtime::{AsyncRuntime, AsyncEvent};

type RespCallFn = extern fn(i32,bool,i32);
type RespReadEndCallFn = extern fn(i32,i64,*const u8);
#[derive(Debug)]
pub enum AsyncHttpEvent {
  OnResp(usize,Result<Response,Error>),
  OnRespReadBytes(usize,Result<Bytes,Error>)
}
pub struct HttpEnv {
  pub(crate) resp_call:RespCallFn,
  pub(crate) resp_read_end_call:RespReadEndCallFn,
  pub(crate) handle_list:Vec<HttpHandle>,
}

#[no_mangle]
pub unsafe extern "C" fn _ia_set_http_cb_list(runtime:&mut AsyncRuntime,callf:RespCallFn,readcallf:RespReadEndCallFn) {
  runtime._http_env.resp_call = callf;
  runtime._http_env.resp_read_end_call = readcallf;
}

extern fn stub_resp(_:i32,_:bool,_:i32) {}
extern fn stub_resp_read_end(_:i32,_:i64,_:*const u8) {}
impl HttpEnv {
  pub fn new() -> HttpEnv {
    HttpEnv { 
      resp_call:stub_resp,
      resp_read_end_call:stub_resp_read_end,
      handle_list:vec![]
    }
  }

  pub(crate) fn on_async_event(&mut self,event:AsyncHttpEvent) {
     match event {
         AsyncHttpEvent::OnResp(index, resp) => {
            {
              let cur_handle = &mut self.handle_list[index];
              cur_handle.resp = Some(resp);
            }
            if let Some(resp_ref) = self.handle_list[index].resp.as_ref() {
              self.call_resp(index as i32, resp_ref);
            }
         },
         AsyncHttpEvent::OnRespReadBytes(index, bytes) => {
            {
              let cur_handle = &mut self.handle_list[index];
              cur_handle.bytes = Some(bytes);
            }
            if let Some(bytes_ref) = self.handle_list[index].bytes.as_ref() {
              self.call_resp_bytes(index as i32, bytes_ref);
            }
         }
     }
  }

  pub(crate) fn call_resp(&self,idx:i32,resp:&Result<Response,Error>) {
     let f = self.resp_call;
     match resp {
         Err(_) =>  { f(idx,true,0)  }
         Ok(resp) =>  {
           let status_code = resp.status().as_u16() as i32;
           f(0,false,status_code)
         }
     }
  }

  pub(crate) fn call_resp_bytes(&self,idx:i32,resp:&Result<Bytes,Error>) {
    let f = self.resp_read_end_call;
    match resp {
        Err(_) =>  { f(idx,0,std::ptr::null()) }
        Ok(bytes) => { f(idx,bytes.len() as i64,bytes.as_ptr()) }
    }
 }
}



//////////////
#[derive(Default)]
pub struct HttpHandle {
  pub request:Option<RequestBuilder>,
  pub resp:Option<Result<Response,Error>>,
  pub bytes:Option<Result<Bytes,Error>>
}

impl HttpHandle { }

#[no_mangle]
pub unsafe extern "C" fn _ia_http_create(runtime:&mut AsyncRuntime) -> i32 {
  let handle = HttpHandle::default();
  runtime._http_env.handle_list.push(handle);
  (runtime._http_env.handle_list.len() - 1) as i32
}
#[no_mangle]
pub unsafe extern "C" fn _ia_http_set_request(runtime:&mut AsyncRuntime,index:i32,method:i8,url_cstr:*const i8) -> i32 {
   let url_str = CStr::from_ptr(url_cstr).to_str();
   if url_str.is_err() {  return 1; };
   let url = Url::parse(url_str.unwrap());
   if url.is_err() { return 2; }
   if runtime._http_env.handle_list.len() <= index as usize { return 3; }
   let cur_handle = &mut runtime._http_env.handle_list[index as usize];
   let client = Client::new();
   let builder = client.request(i82method(method), url.unwrap());
   cur_handle.request = Some(builder);
   0
}
#[no_mangle]
pub unsafe extern "C" fn _ia_http_send(runtime:&mut AsyncRuntime,index:i32) -> i32 {
  if runtime._http_env.handle_list.len() <= index as usize { return 1; }
  let cur_handle = &mut runtime._http_env.handle_list[index as usize];
  if let Some(req) = cur_handle.request.take() {
    let resp_future = req.send();
    let clone_sender = runtime.sender.clone();
    runtime.tokio_runtime.spawn(async move {
      let resp = resp_future.await;
      let _ = clone_sender.send(AsyncEvent::Http(AsyncHttpEvent::OnResp(index as usize,resp)));
    });
    return 0;
  }
  2
}
#[no_mangle]
pub unsafe extern "C" fn _ia_http_read_bytes(runtime:&mut AsyncRuntime,index:i32) -> i32 {
  if runtime._http_env.handle_list.len() <= index as usize { return 1; }
  let cur_handle = &mut runtime._http_env.handle_list[index as usize];
  match cur_handle.resp.take() {
    Some(Ok(resp)) => {
      let clone_sender = runtime.sender.clone();
      runtime.tokio_runtime.spawn(async move {
        let bytes = resp.bytes().await;
        let _ = clone_sender.send(AsyncEvent::Http(AsyncHttpEvent::OnRespReadBytes(index as usize,bytes)));
      });
      0
    }
    _ => 1
  }
}

#[no_mangle]
pub unsafe extern "C" fn _ia_http_close(runtime:&mut AsyncRuntime,index:i32) {
  runtime._http_env.handle_list.remove(index as usize);
}

fn i82method(num:i8) -> Method {
  match num {
      0 => Method::GET,
      1 => Method::POST,
      2 => Method::CONNECT,
      3 => Method::PATCH,
      4 => Method::OPTIONS,
      5 => Method::DELETE,
      6 => Method::HEAD,
      7 => Method::PUT,
      8 => Method::TRACE,
      _ => Method::POST
  }
}