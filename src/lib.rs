pub mod v8worker;

extern crate libc;

use libc::size_t;
use std::mem;
use std::os::raw::{c_char, c_void, c_int};

#[repr(C)]
pub struct worker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct buf_s {
    pub data: *mut ::std::os::raw::c_void,
    pub len: size_t,
}

impl Clone for buf_s {
    fn clone(&self) -> Self { *self }
}

type RecvCallbackFn = fn(*mut u8, i32, i32);

pub fn default_recv_callback(_data: *mut u8, _length: i32, _index: i32) {
    println!("default_recv_callback is called!");
}

pub fn new_recv_callback(_data: *mut u8, _length: i32, _index: i32) {
    println!("new_recv_callback is called!");
}

pub static mut CB: RecvCallbackFn = default_recv_callback;

#[link(name = "binding")]
extern {
    pub fn worker_version() -> *const c_char;
    pub fn worker_set_flags(argc: *mut c_int,
                            argv: *mut *mut c_char);
    pub fn v8_init();
    pub fn worker_new(table_index: c_int) -> *mut worker;
    pub fn worker_load(w: *mut worker, name_s: *const c_char,
                       source_s: *const c_char)
     -> c_int;
    pub fn worker_send_bytes(w: *mut worker,
                             data: *mut c_void,
                             len: size_t)
     -> c_int;
    pub fn worker_dispose(w: *mut worker);
    pub fn worker_terminate_execution(w: *mut worker);

    pub fn worker_last_exception(w: *mut worker)
     -> *const c_char;
}

#[test]
fn test_wrapper() {
    unsafe { v8_init() };

    unsafe {
        CB = new_recv_callback
    };

    let mut test_worker = v8worker::new();
    let code = String::from("V8Worker2.print(\"ready\");");
    let script_name = String::from("code.js");
    test_worker.load(script_name, code);

    let code2 = String::from("V8Worker2.send(new ArrayBuffer(10))");
    let script_name2 = String::from("code2.js");
    test_worker.load(script_name2, code2);
    // test_worker.last_exception();
}


#[no_mangle]
pub extern fn recvCb(buf: *mut c_void, _len: c_int, _index: c_int) -> buf_s {
    let contents: *mut u8;
    let length: i32;
    let index: i32;
    unsafe{
        contents = mem::transmute(buf);
        length = _len as i32;
        index = _index as i32;

        // Send data to the recv callback:
        CB(contents, length, index);
        let out: buf_s = mem::uninitialized();
        out
    }
}
