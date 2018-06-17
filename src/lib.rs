extern crate crossbeam;
extern crate crossbeam_channel; 
#[macro_use]
extern crate lazy_static;
extern crate bytes;

pub mod worker;
mod handler;
mod binding;

use worker::Worker;

extern crate libc;

use std::mem;
use std::os::raw::{c_void, c_int};
use bytes::Bytes;

#[derive(Debug)]
pub struct ChannelData {
}
// unsafe impl Send for ChannelData {};

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
    let mut worker = Worker::new(_recv_cb);
    worker.load("code.js".to_string(), "V8Worker2.send(new ArrayBuffer(10))".to_string());
}

#[no_mangle]
pub extern fn recv(_buf: *mut c_void, _len: c_int, w: *mut Worker) -> binding::buf_s {
    // let _sender = handler::CHANNELS.0.clone();
    // sender.send(ch_data);
    let _contents: *mut u8;
    let data: Bytes;
    unsafe {
        _contents = mem::transmute(_buf);
        let slice: &[u8] = std::slice::from_raw_parts(_contents, _len as usize);
        let slice_bytes = Bytes::from(slice);
        data = (*w).recv(slice_bytes);
    };
    let out: binding::buf_s;
    out = binding::buf_s{
        data: data.as_ptr() as *mut c_void,
        len: data.len(),
    };
    out
}