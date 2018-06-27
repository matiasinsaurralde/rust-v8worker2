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
    // Initialize V8 (V8::InitializePlatform and V8::Initialize):
    let mut _h = new_handler();
    _h.init();

    // Setup a callback that receives bytes and returns boxed bytes:
    let cb = |incoming_data: bytes::Bytes| -> Box<bytes::Bytes> {
        println!("Getting data from V8, length is {}", incoming_data.len());
        assert!(incoming_data.len() == 10);

        // Send some stuff to V8, this is not in use at the moment but we still require it:
        let data = Bytes::from(&b"reply"[..]);
        Box::new(data)
    };

    // Initialize a worker with the callback:
    let mut worker = worker::Worker::new(cb);

    // Send an empty ArrayBuffer (V8 -> Rust), the callback will check the length of this buffer:
    worker.load("code.js", "V8Worker2.send(new ArrayBuffer(10))".to_string());
}

#[no_mangle]
pub extern fn recv(_buf: *mut c_void, _len: c_int, raw_cb: *mut fn(Bytes) -> Box<Bytes>) -> *mut binding::buf_s {
    let _contents: *mut u8;
    unsafe {
        // Prepare a bytes object, will be passed to the callback:
        _contents = mem::transmute(_buf);
        let slice: &[u8] = std::slice::from_raw_parts(_contents, _len as usize);
        let slice_bytes = Bytes::from(slice);

        // raw_cb is the callback pointer, trigger it and prepare a buf_s object with its output:
        let data = (*raw_cb)(slice_bytes);
        let data_len = data.len() as usize;
        let boxed_buf_s = Box::new(binding::buf_s{
            data: data.as_ptr() as *mut c_void,
            len: data_len,
        });

        // TODO: develop a mechanism to free the box contents:
        Box::into_raw(boxed_buf_s)
    }
}
