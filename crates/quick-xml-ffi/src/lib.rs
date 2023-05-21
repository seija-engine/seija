use quick_xml::{ events::{attributes::Attributes, Event}, Reader, };
use std::{ ffi::{CStr, CString}, pin::Pin, io::BufReader, fs::File, };

pub struct CommonData {
    last_error: CString,
    cur_attr: Option<Attributes<'static>>,
}

#[repr(C)]
pub struct StringReader {
    common:CommonData,
    _bytes: Pin<Box<Vec<u8>>>,
    reader: Reader<&'static [u8]>,
}

#[repr(C)]
pub struct FileReader {
    common:CommonData,
    buffer:Vec<u8>,
    reader: Reader<BufReader<File>>,
}

#[no_mangle]
pub extern "C" fn xml_reader_from_file(path: *mut i8) -> *mut FileReader  {
    let c_str = unsafe { assert!(!path.is_null());  CStr::from_ptr(path) };
    
    match c_str.to_str() {
        Ok(path_str) => {
            let reader = Reader::from_file(path_str);
            match reader {
                Ok(v) => {
                    let file_reader = Box::new(FileReader {
                        reader: v,
                        buffer:vec![],
                        common:CommonData { last_error: CString::default(), cur_attr: None }
                    });
                    return Box::into_raw(file_reader);
                },
                Err(err) => {
                    eprintln!("read xml {:?} err:{:?}",c_str,err);

                    return std::ptr::null_mut(); 
                }
            }
        },
        Err(_) => { return std::ptr::null_mut(); } 
    }
}

#[no_mangle]
pub extern "C" fn xml_reader_from_string(cstr: *mut i8) -> *mut StringReader {
    let c_str = unsafe {
        assert!(!cstr.is_null());
        CStr::from_ptr(cstr)
    };
    let clone_bytes = c_str.to_bytes().to_owned();
    let pin_bytes = Pin::new(Box::new(clone_bytes));
    let len = pin_bytes.len();
    let array_ptr = pin_bytes.as_ptr();
   
    let mut reader = Box::new(StringReader {
        _bytes: pin_bytes,
        reader: Reader::from_reader(unsafe { &*std::ptr::slice_from_raw_parts(array_ptr, len) }),
        common:CommonData { last_error: CString::default(), cur_attr: None }
    });
    reader.reader.trim_text(true);
    Box::into_raw(reader)
}



fn process_event<'a>(event:&quick_xml::Result<Event<'a>>,common:&mut CommonData,out_type: &mut u8,name_len: &mut i32,name_ptr: &mut *const u8) -> bool {
    match event {
        Ok(event) => {
            match event {
                Event::Start(e) => {
                    *out_type = 1u8;
                    let name = e.name();
                    *name_ptr = name.0.as_ptr();
                    *name_len = name.0.len() as i32;
                    //这里都是借用的StringReader的bytes，所以可以转换成static的
                    let attrs = unsafe {
                        std::mem::transmute::<Attributes<'_>, Attributes<'static>>(e.html_attributes())
                    };
                    common.cur_attr = Some(attrs);
                }
                Event::End(e) => {
                    *out_type = 2u8;
                    let name = e.name();
                    *name_ptr = name.0.as_ptr();
                    *name_len = name.0.len() as i32;
                }
                Event::Empty(e) => {
                    *out_type = 3u8;
                    let name = e.name();
                    *name_ptr = name.0.as_ptr();
                    *name_len = name.0.len() as i32;
                    let attrs = unsafe {
                        std::mem::transmute::<Attributes<'_>, Attributes<'static>>(e.html_attributes())
                    };
                    common.cur_attr = Some(attrs);
                }
                Event::Text(text) => {
                    *name_ptr = text.as_ptr();
                    *name_len = text.len() as i32;
                    *out_type = 4u8; 
                }
                Event::Comment(_) => { *out_type = 5u8; }
                Event::Eof => { *out_type = 6u8; }
                _ => { *out_type = 7u8; }
            }
            return true;
        }
        Err(err) => {
            let err_string = err.to_string();
            common.last_error = CString::new(err_string.as_str()).unwrap();
            return false;
        }
    }
}

#[no_mangle]
pub extern "C" fn xml_get_last_error(common:&mut CommonData) -> *const i8 {
    common.last_error.as_ptr()
}

#[no_mangle]
pub extern "C" fn string_reader_read_event(
    reader: &mut StringReader,
    out_type: &mut u8,
    name_len: &mut i32,
    name_ptr: &mut *const u8,
) -> bool {
    let event = reader.reader.read_event();
    process_event(&event, &mut reader.common, out_type, name_len, name_ptr)
}

#[no_mangle]
pub extern "C" fn file_reader_read_event(
    reader: &mut FileReader,
    out_type: &mut u8,
    name_len: &mut i32,
    name_ptr: &mut *const u8,
) -> bool {
    let event = reader.reader.read_event_into(&mut reader.buffer);
    process_event(&event, &mut reader.common, out_type, name_len, name_ptr)
}


#[no_mangle]
pub extern "C" fn xml_reader_read_attr(
    reader: &mut CommonData,
    is_err: &mut bool,
    kl: &mut i32,
    kptr: &mut *const u8,
    vl: &mut i32,
    vptr: &mut *const u8,
) -> bool {
    *is_err = false;
    if let Some(attrs) = &mut reader.cur_attr {
        match attrs.next() {
            Some(Ok(attr)) => {
                let key = attr.key;
                *kl = key.0.len() as i32;
                *kptr = key.0.as_ptr();
                *vl = attr.value.len() as i32;
                *vptr = attr.value.as_ptr();
                return true;
            }
            Some(Err(err)) => {
                let err_string = err.to_string();
                reader.last_error = CString::new(err_string.as_str()).unwrap();
                *is_err = true;
                return false;
            }
            None => {}
        }
    }
    false
}


#[no_mangle]
pub unsafe extern "C" fn xml_reader_release_string(string:*mut StringReader) {
    let _ = Box::from_raw(string);
}

#[no_mangle]
pub unsafe extern "C" fn xml_reader_release_file(file:*mut FileReader) {
    let _ = Box::from_raw(file);
}
