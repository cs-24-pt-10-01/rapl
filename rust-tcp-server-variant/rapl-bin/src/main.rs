use anyhow::Result;
use serde::Deserialize;
use std::{ffi::CString, io::Read, net::TcpStream};

pub const CONFIG: bincode::config::Configuration = bincode::config::standard();

#[derive(Debug, Deserialize)]
struct OutputData {
    timestamp_start: u128,
    timestamp_end: u128,
    pp0_start: u64,
    pp0_end: u64,
    pp1_start: u64,
    pp1_end: u64,
    pkg_start: u64,
    pkg_end: u64,
    dram_start: u64,
    dram_end: u64,
}

fn main() -> Result<()> {
    let func_cstring = CString::new("My.CSharp.Function").unwrap();
    //unsafe { rapl_string_test(func_cstring.as_ptr()) };

    //start_rapl_iter();

    // Connect to the RAPL library
    // Start tcp client and then connect
    let mut stream = TcpStream::connect("127.0.0.1:80").unwrap();
    //stream.set_nodelay(true).unwrap();
    //stream.set_nonblocking(true).unwrap();

    // Get the data sent from the RAPL library

    // Loop as designed for macrobenchmarks
    loop {
        let mut data = [0; 100];
        println!("Reading data from RAPL library... 1");
        stream.read_exact(&mut data).unwrap();
        println!("Data length {}", data.len());
        println!("Data {:?}", data);
        println!("Finished reading data from RAPL library... 1");

        //let output: OutputData = bincode::serde::decode_from_std_read(&mut stream, CONFIG).unwrap();
        //println!("{:?}", output);
        //println!("{:?}", output);

        // Sleep for 10 milliseconds
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
