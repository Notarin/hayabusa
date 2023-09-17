use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use sysinfo::{CpuExt, System, SystemExt};
use std::fs;
use gfx_backend_vulkan::Backend;
use gfx_hal::adapter::Adapter;
use gfx_backend_vulkan as back;
use gfx_hal::Instance;


struct SystemInfo {
    cpu: String,
    distro: String,
    motherboard: String,
    kernel: String,
    gpus: Vec<String>,
    total_memory: u64,
    used_memory: u64,
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
    let gpus: Vec<String> = get_gpus();
    let total_memory: u64 = get_total_memory();
    let used_memory: u64 = get_used_memory();

    let system_info: SystemInfo = SystemInfo {
        cpu,
        distro,
        motherboard,
        kernel,
        gpus,
        total_memory,
        used_memory,
    };

    let distro: String = "Distro: ".to_owned() + &*system_info.distro;
    let cpu: String = "CPU: ".to_owned() + &*system_info.cpu;
    let motherboard: String = "Motherboard: ".to_owned() + &*system_info.motherboard;
    let kernel: String = "Kernel: ".to_owned() + &*system_info.kernel;
    let gpus: String = "GPUs: ".to_owned() + &*system_info.gpus.join("\n");
    let total_memory_parsed = system_info.total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_memory_parsed = system_info.used_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let memory: String = "".to_owned()
        + "Memory: "
        + &*format!("{:.2} GiB/{:.2} GiB", used_memory_parsed, total_memory_parsed);


    let final_fetch: String = "".to_owned()
        + &*distro + "\n"
        + &*cpu + "\n"
        + &*motherboard + "\n"
        + &*kernel + "\n"
        + &*gpus + "\n"
        + &*memory + "\n";

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

fn get_gpus() -> Vec<String> {

    let instance: gfx_backend_vulkan::Instance =
        back::Instance::create("hayabusa", 1).unwrap();
    let adapters: Vec<Adapter<Backend>> = instance.enumerate_adapters();

    let mut names: Vec<String> = Vec::new();

    for adapter in adapters {
        names.push(adapter.info.name.to_string());
    }

    names
}

fn get_total_memory() -> u64 {
    let sys: MutexGuard<System> = SYS.lock().unwrap();
    sys.total_memory()
}

fn get_used_memory() -> u64 {
    let sys: MutexGuard<System> = SYS.lock().unwrap();
    sys.used_memory()
}