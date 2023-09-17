use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use sysinfo::{CpuExt, System, SystemExt};
use std::fs;

struct SystemInfo {
    cpu: String,
    distro: String,
    motherboard: String,
    kernel: String,
}

lazy_static! {
    static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

pub(crate) fn main() {
    {
        let mut sys: MutexGuard<System> = SYS.lock().unwrap();
        sys.refresh_all();
    }
    let cpu: String = get_cpu_name();
    let distro: String = get_distro();
    let motherboard: String = get_motherboard();
    let kernel: String = get_kernel();

    let system_info: SystemInfo = SystemInfo {
        cpu,
        distro,
        motherboard,
        kernel,
    };

    let distro: String = "Distro: ".to_owned() + &*system_info.distro;
    let cpu: String = "CPU: ".to_owned() + &*system_info.cpu;
    let motherboard: String = "Motherboard: ".to_owned() + &*system_info.motherboard;
    let kernel: String = "Kernel: ".to_owned() + &*system_info.kernel;

    let final_fetch: String = "".to_owned()
        + &*distro + "\n"
        + &*cpu + "\n"
        + &*motherboard + "\n"
        + &*kernel;

    println!("{}", final_fetch);
}

fn get_cpu_name() -> String {
    let sys: MutexGuard<System> = SYS.lock().unwrap();
    sys.global_cpu_info().brand().to_string()
}

fn get_distro() -> String {
    let sys: MutexGuard<System> = SYS.lock().unwrap();
    sys.name().unwrap_or(String::from("Unknown"))
}

#[cfg(target_os = "linux")]
fn get_motherboard() -> String {
    fs::read_to_string("/sys/class/dmi/id/board_name")
        .unwrap_or(String::from("Unknown"))
        .trim()
        .to_string()
}


fn get_kernel() -> String {
    let sys: MutexGuard<System> = SYS.lock().unwrap();
    sys.kernel_version().unwrap_or(String::from("Unknown"))
}