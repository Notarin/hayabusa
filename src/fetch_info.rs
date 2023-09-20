use std::sync::{Mutex, MutexGuard};
use reqwest::Client;
use std::time::Duration;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use std::net::IpAddr;
use local_ip_address::local_ip;
use gfx_hal::adapter::Adapter;
use gfx_backend_vulkan::Backend;
use gfx_hal::Instance;
use lazy_static::lazy_static;

lazy_static! {
    pub (crate) static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

pub(crate) struct SystemInfo {
    pub(crate) cpu: String,
    pub(crate) distro: String,
    pub(crate) motherboard: String,
    pub(crate) kernel: String,
    pub(crate) gpus: Vec<String>,
    pub(crate) memory: Memory,
    pub(crate) disks: Vec<Disk>,
    pub(crate) local_ip: String,
    pub(crate) public_ip: String,
}

#[derive(Clone)]
pub(crate) struct Disk {
    pub(crate) name: String,
    pub(crate) used: u64,
    pub(crate) total: u64,
}

pub(crate) struct Memory {
    pub(crate) used: u64,
    pub(crate) total: u64,
}

pub(crate) async fn get_cpu_name() -> String {
    let sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.global_cpu_info().brand().to_string()
}

pub(crate) async fn get_distro() -> String {
    let sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.name().unwrap_or(String::from("Unknown"))
}

#[cfg(target_os = "linux")]
pub(crate) async fn get_motherboard() -> String {
    use std::fs;
    fs::read_to_string("/sys/class/dmi/id/board_name")
        .unwrap_or(String::from("Unknown"))
        .trim()
        .to_string()
}

#[cfg(target_os = "windows")]
pub(crate) async fn get_motherboard() -> String {
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

pub(crate) async fn get_kernel() -> String {
    let sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.kernel_version().unwrap_or(String::from("Unknown"))
}

pub(crate) async fn get_gpus() -> Vec<String> {

    let instance: gfx_backend_vulkan::Instance =
        Instance::create("hayabusa", 1).expect("Failed to create Vulkan instance");
    let adapters: Vec<Adapter<Backend>> = instance.enumerate_adapters();

    let mut names: Vec<String> = Vec::new();

    for adapter in adapters {
        names.push(adapter.info.name.to_string());
    }

    names
}

pub(crate) async fn get_total_memory() -> u64 {
    let sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.total_memory()
}

pub(crate) async fn get_used_memory() -> u64 {
    let sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.used_memory()
}

pub(crate) async fn get_disks() -> Vec<Disk> {
    let sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
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

pub(crate) async fn get_local_ip_address() -> String {
    let local_ip: IpAddr = local_ip().expect("Failed to get local IP address");
    local_ip.to_string()
}

pub(crate) async fn get_public_ip_address() -> String {
    let client: Client = Client::builder().timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build reqwest client");
    match client.get("https://ident.me").send().await {
        Ok(res) => res.text().await.expect("Failed to get public IP address"),
        Err(_) => "Unknown".to_string(),
    }
}
