use crate::rapl;
use std::ffi::{c_char, CStr};

// # Safety
//
// This function is unsafe because it dereferences the `id` pointer.
#[no_mangle]
pub unsafe extern "C" fn start_rapl(id: *const c_char) {
    let id_cstr = unsafe { CStr::from_ptr(id) };
    let id_string = String::from_utf8_lossy(id_cstr.to_bytes()).to_string();
    rapl::start_rapl(id_string);
}

// # Safety
//
// This function is unsafe because it dereferences the `id` pointer.
#[no_mangle]
pub unsafe extern "C" fn stop_rapl(id: *const c_char) {
    let id_cstr = unsafe { CStr::from_ptr(id) };
    let id_string = String::from_utf8_lossy(id_cstr.to_bytes()).to_string();
    rapl::stop_rapl(id_string);
}