use crossbeam::queue::SegQueue;

use once_cell::sync::OnceCell;
use serde::Serialize;
use std::{
    io::Write,
    net::TcpListener,
    sync::Once,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

// Use the OS specific implementation
#[cfg(target_os = "linux")]
pub mod os_linux;
#[cfg(target_os = "windows")]
pub mod os_windows;

// Import the OS specific functions
#[cfg(target_os = "linux")]
use self::os_linux::{rapl_log_init, read_msr};
#[cfg(target_os = "windows")]
use self::os_windows::{rapl_log_init, read_msr};

#[derive(Error, Debug)]
pub enum RaplError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[cfg(target_os = "windows")]
    #[error("windows error")]
    Windows(#[from] windows::core::Error),
}

#[cfg(intel)]
#[derive(Debug, Serialize)]
struct RaplRegisters {
    pp0: u64,
    pp1: u64,
    pkg: u64,
    dram: u64,
}

#[cfg(amd)]
#[derive(Debug, Serialize)]
struct RaplRegisters {
    core: u64,
    pkg: u64,
}

#[derive(Debug, Serialize)]
struct RaplLog {
    id: String,
    thread_id: usize,
    cpu_type: CPUType,
    timestamp: u128,
    rapl_registers: RaplRegisters,
}

static RAPL_INIT: Once = Once::new();
// TOOD: Bitfield here, use the "bitfield-struct" crate or so. Just check it out at least. Utilize OS specific ver for it
static RAPL_POWER_UNITS: OnceCell<u64> = OnceCell::new();
static RAPL_LOGS: OnceCell<SegQueue<RaplLog>> = OnceCell::new();

pub fn rapl_log(id: String) {
    RAPL_INIT.call_once(|| {
        // Run the OS specific rapl_log_init function, to enable reading MSR registers
        rapl_log_init();

        // Import the MSR RAPL power unit constants per CPU type
        #[cfg(amd)]
        use crate::rapl::amd::MSR_RAPL_POWER_UNIT;
        #[cfg(intel)]
        use crate::rapl::intel::MSR_RAPL_POWER_UNIT;

        // Read power unit and store it in the power units global variable
        let pwr_unit = read_msr(MSR_RAPL_POWER_UNIT).expect("failed to read RAPL power unit");
        RAPL_POWER_UNITS.get_or_init(|| pwr_unit);

        start_rapl_server();
    });

    // Get the current time in milliseconds since the UNIX epoch
    let timestamp_start = get_timestamp_millis();

    // Read the RAPL registers
    let rapl_registers = read_rapl_registers();

    // Get the RAPL logs queue
    let rapl_logs_queue = RAPL_LOGS.get_or_init(|| SegQueue::new());

    // Create a new RAPL log
    let rapl_log = RaplLog {
        id,
        timestamp: timestamp_start,
        rapl_registers: RaplRegisters {
            core: rapl_registers.0,
            pkg: rapl_registers.1,
        },
        thread_id: thread_id::get(),
        cpu_type: get_cpu_type(),
    };

    // Push the RAPL log to the queue
    rapl_logs_queue.push(rapl_log);
}

fn start_rapl_server() {
    // Start TCP server
    std::thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:80").unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            //stream.set_nodelay(true).unwrap();
            //stream.set_nonblocking(true).unwrap();

            println!("Connection established!");

            std::thread::spawn(move || {
                loop {
                    // TODO: Send the RAPL logs to all connected clients

                    // Get the RAPL logs queue
                    let rapl_logs_queue = RAPL_LOGS.get().unwrap();

                    // Create a vector to store the RAPL logs, in order to send it as one big message
                    let mut rapl_logs_vec = Vec::new();
                    while let Some(rapl_log) = rapl_logs_queue.pop() {
                        rapl_logs_vec.push(rapl_log);
                    }

                    // Serialize the RAPL logs vector, then send it to the client
                    let serialized_rapl_logs = bincode::serialize(&rapl_logs_vec).unwrap();
                    stream.write_all(&serialized_rapl_logs).unwrap();
                    stream.flush().unwrap();

                    // Sleep for 500 milliseconds
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            });
        }
    });
}

fn get_timestamp_millis() -> u128 {
    let current_time = SystemTime::now();
    let duration_since_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration_since_epoch.as_millis()
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
enum CPUType {
    Intel,
    AMD,
}

// Get the CPU type based on the compile time configuration
fn get_cpu_type() -> CPUType {
    #[cfg(intel)]
    {
        CPUType::Intel
    }

    #[cfg(amd)]
    {
        CPUType::AMD
    }
}

#[cfg(amd)]
fn read_rapl_registers() -> (u64, u64) {
    use self::amd::{AMD_MSR_CORE_ENERGY, MSR_RAPL_PKG_ENERGY_STAT};

    /*let rapl_registers = RaplRegisters {
        core: read_msr(AMD_MSR_CORE_ENERGY).expect("failed to read CORE_ENERGY"),
        pkg: read_msr(MSR_RAPL_PKG_ENERGY_STAT).expect("failed to read RAPL_PKG_ENERGY_STAT"),
    };*/

    let core = read_msr(AMD_MSR_CORE_ENERGY).expect("failed to read CORE_ENERGY");
    let pkg = read_msr(MSR_RAPL_PKG_ENERGY_STAT).expect("failed to read RAPL_PKG_ENERGY_STAT");

    (core, pkg)
}

#[cfg(intel)]
fn read_rapl_registers() -> (u64, u64, u64, u64) {
    use self::intel::{
        INTEL_MSR_RAPL_DRAM, INTEL_MSR_RAPL_PP0, INTEL_MSR_RAPL_PP1, MSR_RAPL_PKG_ENERGY_STAT,
    };

    let pp0 = read_msr(INTEL_MSR_RAPL_PP0).expect("failed to read PP0");
    let pp1 = read_msr(INTEL_MSR_RAPL_PP1).expect("failed to read PP1");
    let pkg = read_msr(MSR_RAPL_PKG_ENERGY_STAT).expect("failed to read RAPL_PKG_ENERGY_STAT");
    let dram = read_msr(INTEL_MSR_RAPL_DRAM).expect("failed to read DRAM");

    (pp0, pp1, pkg, dram)
}

#[cfg(amd)]
pub mod amd {
    /*
    https://lore.kernel.org/lkml/20180817163442.10065-2-calvin.walton@kepstin.ca/

    "A notable difference from the Intel implementation is that AMD reports
    the "Cores" energy usage separately for each core, rather than a
    per-package total"
     */

    pub const MSR_RAPL_POWER_UNIT: u64 = 0xC0010299; // Similar to Intel MSR_RAPL_POWER_UNIT
    pub const MSR_RAPL_PKG_ENERGY_STAT: u64 = 0xC001029B; // Similar to Intel PKG_ENERGY_STATUS (This is for the whole socket)

    pub const AMD_MSR_CORE_ENERGY: u64 = 0xC001029A; // Similar to Intel PP0_ENERGY_STATUS (PP1 is for the GPU)

    /*
    const AMD_TIME_UNIT_MASK: u64 = 0xF0000;
    const AMD_ENERGY_UNIT_MASK: u64 = 0x1F00;
    const AMD_POWER_UNIT_MASK: u64 = 0xF;
    */
}

#[cfg(intel)]
pub mod intel {
    pub const MSR_RAPL_POWER_UNIT: u64 = 0x606;
    pub const MSR_RAPL_PKG_ENERGY_STAT: u64 = 0x611;

    pub const INTEL_MSR_RAPL_PP0: u64 = 0x639;
    pub const INTEL_MSR_RAPL_PP1: u64 = 0x641;
    pub const INTEL_MSR_RAPL_DRAM: u64 = 0x619;
    /*
    const INTEL_TIME_UNIT_MASK: u64 = 0xF000;
    const INTEL_ENGERY_UNIT_MASK: u64 = 0x1F00;
    const INTEL_POWER_UNIT_MASK: u64 = 0x0F;

    const INTEL_TIME_UNIT_OFFSET: u64 = 0x10;
    const INTEL_ENGERY_UNIT_OFFSET: u64 = 0x08;
    const INTEL_POWER_UNIT_OFFSET: u64 = 0;
    */
}
