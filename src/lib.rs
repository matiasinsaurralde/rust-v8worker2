extern crate libc;
use libc::size_t;
use std::ffi::CString;
use std::os::raw::{c_char, c_void, c_int};

#[repr(C)]
pub struct worker {
}

#[link(name = "v8worker")]
extern {
    fn v8_init();
    fn worker_new(table_index: u64) -> *const worker;
    fn worker_dispose(w: *const worker);
    fn worker_load(worker: *const worker, name_s: *const c_char, source_s: *const c_char) -> c_int;
    fn worker_send_bytes(worker: *const worker, data: *const c_void, len: size_t) -> c_int;
}

#[no_mangle]
pub extern fn recvCb(buf: *const c_char, len: c_int, index: c_int) {
  println!("TestCall runs");
}

pub fn init() {
    unsafe {
        v8_init();
    }
}

pub fn v8_new_worker(_table_index: u64) -> *const worker {
    unsafe {
        return worker_new(_table_index);
    }
}

pub fn v8_worker_dispose(w: *const worker) {
    unsafe {
        worker_dispose(w);
    }
}

pub fn v8_worker_load(w: *const worker, name: String, source: String) {
    let c_name = CString::new(name).unwrap();
    let c_source = CString::new(source).unwrap();
    unsafe {
        worker_load(w, c_name.as_ptr(), c_source.as_ptr());
    }
}

pub fn v8_worker_send_bytes(w: *const worker, data: &[u8]) {
    let c_data = CString::new(data).unwrap();
    let datalen = data.len() as size_t;
    unsafe {
        worker_send_bytes(w, c_data.as_ptr() as *mut c_void, datalen);
    }
}