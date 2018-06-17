#![allow(improper_ctypes)]
extern crate libc;

use libc::size_t;
use std::os::raw::{c_char, c_void, c_int};

use worker::Worker;

#[repr(C)]
pub struct worker {
    _unused: [u8; 0],
}

#[link(name = "binding")]
extern {
    // pub fn worker_version() -> *const c_char;
    // pub fn worker_set_flags(argc: *mut c_int,
    //                        argv: *mut *mut c_char);
    pub fn v8_init();
    pub fn worker_new() -> *mut worker;
    pub fn worker_set_rust_object(w: *mut worker, p: *mut Worker);
    pub fn worker_load(w: *mut worker, name_s: *const c_char,
                       source_s: *const c_char) -> c_int;
    pub fn worker_send_bytes(w: *mut worker,
                             data: *mut c_void,
                             len: size_t)
     -> c_int;
    pub fn worker_dispose(w: *mut worker);
    pub fn worker_terminate_execution(w: *mut worker);

    pub fn worker_last_exception(w: *mut worker)
     -> *const c_char;
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