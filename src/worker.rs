use std::ffi::{CString, CStr};
use libc::size_t;
use std::os::raw::c_void;
use std::str::Utf8Error;
use std::ptr::NonNull;
use std::marker;
use bytes::Bytes;

use binding;

// Wrapper for the V8 worker pointer, allows sending it over threads
struct WorkerPtr(NonNull<binding::worker>);
unsafe impl marker::Send for WorkerPtr {}

// Worker structure to wrap FFI calls, etc.
#[repr(C)]
pub struct Worker {
  ptr: WorkerPtr,
  cb: Box<Fn(Bytes) -> Bytes>,
}

impl Worker {
  pub fn new<F: 'static>(func: F) -> Worker where F: Fn(Bytes) -> Bytes {
    // Initialize a V8 worker:
    let mut _ptr: *mut binding::worker;
    _ptr = unsafe { binding::worker_new() };

    let boxed_cb: Box<Fn(Bytes) -> Bytes + 'static> = Box::new(func);

    // Wrap and store the worker pointer:
    let wrapper = WorkerPtr(NonNull::new(_ptr).unwrap());
    let w = Worker{
      ptr: wrapper,
      cb: boxed_cb,
    };

    // Also set a pointer to our Rust object:
    let mut boxed_worker = Box::new(w);
    unsafe { binding::worker_set_rust_object(_ptr, boxed_worker.as_mut())};
    *boxed_worker
  }

  pub fn load(&mut self, script_name: String, code: String) {
    let c_script_name = CString::new(script_name).unwrap();
    let c_code = CString::new(code).unwrap();
    unsafe {
      binding::worker_load(self.as_ptr(), c_script_name.as_ptr(), c_code.as_ptr());
    }
  }

  pub fn send_bytes(&mut self, data: Bytes) {
    unsafe {
      binding::worker_send_bytes(self.as_ptr(), data.as_ptr() as *mut c_void, data.len());
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

  pub fn recv(&mut self, _data: Bytes) -> Bytes {
    let cb = self.cb.as_mut();
    cb(_data)
  }
}
