use anyhow::Result;
use rapl_lib::ffi::{start_rapl, stop_rapl};
use std::ffi::CString;

pub const CONFIG: bincode::config::Configuration = bincode::config::standard();

fn main() -> Result<()> {
    let start = CString::new("start").expect("CString::new failed");
    let stop = CString::new("stop").expect("CString::new failed");
    // Call start and stop rapl 500 times
    for _ in 0..500 {
        unsafe { start_rapl(start.as_ptr()) };
        unsafe { stop_rapl(stop.as_ptr()) };
    }

    Ok(())
}
