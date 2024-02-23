use anyhow::Result;
use rapl_lib::ffi::{start_rapl, stop_rapl};
use std::ffi::CString;

pub const CONFIG: bincode::config::Configuration = bincode::config::standard();

fn main() -> Result<()> {
    let test_function = CString::new("TestFunction").expect("CString::new failed");
    // Call start and stop rapl 500 times

    // Get current time in milliseconds
    let start = std::time::Instant::now();

    for _ in 0..10000 {
        unsafe { start_rapl(test_function.as_ptr()) };
        unsafe { stop_rapl(test_function.as_ptr()) };
    }

    let end = start.elapsed().as_millis();

    println!("Time elapsed: {}ms", end);

    // Sleep for 5 seconds to allow writing to CSV
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
