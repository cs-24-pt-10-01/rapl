use anyhow::Result;
use rapl_lib::ffi::{start_rapl, stop_rapl};
use std::{ffi::CString, thread, time::Duration};

fn main() -> Result<()> {
    // Get current time in milliseconds
    let start = std::time::Instant::now();

    // spawn 10 threads to run the benchmark function
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let test_function = CString::new(format!("test_function {}", i)).unwrap();

                for _ in 0..100000 {
                    unsafe { start_rapl(test_function.as_ptr()) };
                    unsafe { stop_rapl(test_function.as_ptr()) };
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let end = start.elapsed().as_millis();

    println!("Start and stop rapl benchmark: {}ms", end);

    // Sleep for 5 seconds to allow writing to CSV
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
