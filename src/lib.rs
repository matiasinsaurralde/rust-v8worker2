extern crate bytes;

pub mod worker;
mod handler;
mod binding;

extern crate libc;

use std::mem;
use std::os::raw::{c_void, c_int};
use bytes::Bytes;

pub fn new_handler() -> handler::Handler {
    let h = handler::new();
    h
}

#[test]
fn test_wrapper() {
    let mut _h = new_handler();
    _h.init();
    let _recv_cb = move |data: Bytes| {
        data
    };
    let mut worker = worker::Worker::new(_recv_cb);
    worker.load("code.js".to_string(), "V8Worker2.send(new ArrayBuffer(10))".to_string());
}

#[no_mangle]
pub extern fn recv(_buf: *mut c_void, _len: c_int, raw_cb: *mut fn(Bytes) -> Bytes) -> binding::buf_s {
    let _contents: *mut u8;
    let out: binding::buf_s;
    unsafe {
        _contents = mem::transmute(_buf);
        let slice: &[u8] = std::slice::from_raw_parts(_contents, _len as usize);
        let slice_bytes = Bytes::from(slice);
        let data = (*raw_cb)(slice_bytes);
        let data_len = data.len() as usize;
        out = binding::buf_s{
            data: data.as_ptr() as *mut c_void,
            len: data_len,
        };
        out
    }
}
