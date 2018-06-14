use std::ffi::{CString, CStr};
use libc::size_t;
use std::os::raw::c_void;
use std::str::Utf8Error;
use std::ptr::NonNull;
use std::marker;
use binding;

// Wrapper for the V8 worker pointer, allows sending it over threads
struct WorkerPtr(NonNull<binding::worker>);
unsafe impl marker::Send for WorkerPtr {}

// Worker structure to wrap FFI calls, etc.
pub struct Worker{
  ptr: WorkerPtr
}

impl Worker {
  pub fn new(_id: i32) -> Worker {
    let mut _ptr: *mut binding::worker;
    _ptr = unsafe { binding::worker_new(_id) };
    let wrapper = WorkerPtr(NonNull::new(_ptr).unwrap());
    Worker{
      ptr: wrapper
    }
  }

  pub fn load(&mut self, script_name: String, code: String) {
    let c_script_name = CString::new(script_name).unwrap();
    let c_code = CString::new(code).unwrap();
    unsafe {
      binding::worker_load(self.as_ptr(), c_script_name.as_ptr(), c_code.as_ptr());
    }
  }

  pub fn send_bytes(&mut self, data: &[u8]) {
    let c_data = CString::new(data).unwrap();
    let len = data.len() as size_t;
    unsafe {
      binding::worker_send_bytes(self.as_ptr(), c_data.as_ptr() as *mut c_void, len);
    };
  }

  pub fn last_exception(&mut self) -> Result<&str, Utf8Error> {
    unsafe {
      let v = binding::worker_last_exception(self.as_ptr());
      let v = CStr::from_ptr(v);
      let v = v.to_str();
      v
    }
  }

  pub fn dispose(&mut self) {
    unsafe {
      binding::worker_dispose(self.as_ptr());
    }
  }

  pub fn terminate_execution(&mut self) {
    unsafe {
      binding::worker_terminate_execution(self.as_ptr());
    }
  }

  pub fn as_ptr(&mut self) -> *mut binding::worker {
    unsafe {
      self.ptr.0.as_mut()
    }
  }
}
