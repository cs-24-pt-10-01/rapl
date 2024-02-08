use crate::rapl;
use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn start_rapl() {
    rapl::start_rapl();
}

#[no_mangle]
pub extern "C" fn stop_rapl() {
    rapl::stop_rapl();
}

#[no_mangle]
pub unsafe extern "C" fn rapl_string_test(id: *const c_char) {
    let id_cstr = unsafe { CStr::from_ptr(id) };
    let id_string = String::from_utf8_lossy(id_cstr.to_bytes()).to_string();
    println!("Rust: {}", id_string);
}

#[no_mangle]
pub extern "C" fn start_rapl_iter() {
    rapl::start_rapl_iter();
}

#[no_mangle]
pub extern "C" fn stop_rapl_iter() {
    //rapl::stop_rapl_iter();
}

// Interval for reading the RAPL data, equivalent to sampling thing from jRAPL
#[no_mangle]
pub extern "C" fn set_rapl_read_interval() {
    //rapl::stop_rapl_iter();
}

// Interval for updating the RAPL data aka sending to clients
#[no_mangle]
pub extern "C" fn set_loggger_update_interval() {
    //rapl::start_rapl_iter();
}
