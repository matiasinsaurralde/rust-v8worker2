use std::ffi::{CString, CStr};
use libc::size_t;
use std::os::raw::{c_void};
use std::str::Utf8Error;

use {
    worker_new, worker_load, worker_terminate_execution,
    worker_last_exception, worker_dispose, worker_send_bytes,
    worker
};

pub struct V8Worker {
    // w: Unique<worker>,
    w: *mut worker
}

pub fn new() -> V8Worker
{
  unsafe {
    let worker_ptr = worker_new(1);
    V8Worker{w: worker_ptr}
  }
}

impl V8Worker {
  // int worker_load(worker* w, char* name_s, char* source_s);
  pub fn load(&mut self, script_name: String, code: String) {
    let c_script_name = CString::new(script_name).unwrap();
    let c_code = CString::new(code).unwrap();
    unsafe {
      worker_load(self.w, c_script_name.as_ptr(), c_code.as_ptr());
    }
  }

  // const char* worker_last_exception(worker* w);
  pub fn last_exception(&mut self) -> Result<&str, Utf8Error> {
      let v = unsafe { worker_last_exception(self.w) };
      let v = unsafe { CStr::from_ptr(v) };
      let v = v.to_str();
      v
  }

  // int worker_send_bytes(worker* w, void* data, size_t len);
  pub fn send_bytes(&mut self, data: &[u8]) {
    let c_data = CString::new(data).unwrap();
    let datalen = data.len() as size_t;
    unsafe {
      worker_send_bytes(self.w, c_data.as_ptr() as *mut c_void, datalen);
    };
  }

  pub fn dispose(&mut self) {
    unsafe {
      worker_dispose(self.w);
    }
  }

  pub fn terminate_execution(&mut self) {
    unsafe {
      worker_terminate_execution(self.w);
    }
  }
}