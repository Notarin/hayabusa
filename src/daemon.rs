use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
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
    disks: Vec<Disk>,
}

#[derive(Clone)]
struct Disk {
    name: String,
    used: u64,
    total: u64,
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
    let disks: Vec<Disk> = get_disks();

    let system_info: SystemInfo = SystemInfo {
        cpu,
        distro,
        motherboard,
        kernel,
        gpus,
        total_memory,
        used_memory,
        disks,
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
    let disks: String = system_info.disks.iter().cloned().map(|disk| {
        let used_parsed = disk.used as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_parsed = disk.total as f64 / 1024.0 / 1024.0 / 1024.0;
        format!("Disk: {}: {:.2} GiB/{:.2} GiB", disk.name, used_parsed, total_parsed)
    }).collect::<Vec<String>>().join("\n");


    let final_fetch: String = "".to_owned()
        + &*distro + "\n"
        + &*cpu + "\n"
        + &*motherboard + "\n"
        + &*kernel + "\n"
        + &*gpus + "\n"
        + &*memory + "\n"
        + &*disks;

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
    use std::fs;
    fs::read_to_string("/sys/class/dmi/id/board_name")
        .unwrap_or(String::from("Unknown"))
        .trim()
        .to_string()
}

#[cfg(target_os = "windows")]
fn get_motherboard() -> String {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let local_machine_key: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path: &str = r"SYSTEM\HardwareConfig\Current";

    match local_machine_key.open_subkey(path) {
        Ok(sub_key) => {
            match sub_key.get_value("BaseBoardProduct") {
                Ok(name) => name,
                Err(_) => String::from("Unknown"),
            }
        }
        Err(_) => String::from("Unknown"),
    }
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

fn get_disks() -> Vec<Disk> {
    let sys: MutexGuard<System> = SYS.lock().unwrap();
    let sys_disks: &[sysinfo::Disk] = sys.disks();
    let mut disks: Vec<Disk> = Vec::new();
    for disk in sys_disks {
        let name: String = disk.mount_point().to_string_lossy().to_string();
        let used: u64 = disk.total_space() - disk.available_space();
        let total: u64 = disk.total_space();
        let disk: Disk = Disk {
            name,
            used,
            total,
        };
        disks.push(disk);
    }
    disks
}