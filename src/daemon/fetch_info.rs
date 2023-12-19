use crate::daemon::main::SYSTEM_INFO_MUTEX;
use crate::daemon::package_managers::{get_package_count, Packages};
use gfx_backend_vulkan::Backend;
use gfx_hal::adapter::Adapter;
use gfx_hal::{Instance, UnsupportedBackend};
use lazy_static::lazy_static;
use local_ip_address::local_ip;
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::process::{Command, Output};
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use tokio::spawn;
use tokio::task::JoinHandle;

lazy_static! {
    pub(crate) static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub(crate) hostname: String,
    pub(crate) boot_time: u64,
    pub(crate) packages: Packages,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Disk {
    pub(crate) name: String,
    pub(crate) used: u64,
    pub(crate) total: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Memory {
    pub(crate) used: u64,
    pub(crate) total: u64,
}

pub(crate) async fn fetch_all() -> SystemInfo {
    let cpu_future: JoinHandle<String> = spawn(get_cpu_name());
    let distro_future: JoinHandle<String> = spawn(get_distro());
    let motherboard_future: JoinHandle<String> = spawn(get_motherboard());
    let kernel_future: JoinHandle<String> = spawn(get_kernel());
    let gpus_future: JoinHandle<Vec<String>> = spawn(get_gpus());
    let memory_future: JoinHandle<Memory> = spawn(async {
        Memory {
            used: get_used_memory().await,
            total: get_total_memory().await,
        }
    });
    let disks_future: JoinHandle<Vec<Disk>> = spawn(get_disks());
    let local_ip_future: JoinHandle<String> = spawn(get_local_ip_address());
    let public_ip_future: JoinHandle<String> = spawn(get_public_ip_address());
    let hostname_future: JoinHandle<String> = spawn(get_hostname());
    let boot_time_future: JoinHandle<u64> = spawn(get_boot_time());
    let packages_future: JoinHandle<Packages> = spawn(get_package_count());

    let cpu: String = cpu_future.await.expect("get_cpu_name thread panicked!");
    let distro: String = distro_future.await.expect("get_distro thread panicked!");
    let motherboard: String = motherboard_future
        .await
        .expect("get_motherboard thread panicked!");
    let kernel: String = kernel_future.await.expect("get_kernel thread panicked!");
    let gpus: Vec<String> = gpus_future.await.expect("get_gpus thread panicked!");
    let memory: Memory = memory_future.await.expect("get_memory thread panicked!");
    let disks: Vec<Disk> = disks_future.await.expect("get_disks thread panicked!");
    let local_ip: String = local_ip_future
        .await
        .expect("get_local_ip_address thread panicked!");
    let public_ip: String = public_ip_future
        .await
        .expect("get_public_ip_address thread panicked!");
    let hostname: String = hostname_future
        .await
        .expect("get_hostname thread panicked!");
    let boot_time: u64 = boot_time_future
        .await
        .expect("get_boot_time thread panicked!");
    let packages: Packages = packages_future
        .await
        .expect("get_package_count thread panicked!");

    let system_info: SystemInfo = SystemInfo {
        cpu,
        distro,
        motherboard,
        kernel,
        gpus,
        memory,
        disks,
        local_ip,
        public_ip,
        hostname,
        boot_time,
        packages,
    };
    system_info
}

pub(crate) fn serialize_fetch() -> String {
    let system_info: SystemInfo = SYSTEM_INFO_MUTEX
        .lock()
        .expect("Failed to lock system info mutex")
        .clone()
        .expect("System info has not been initialized");
    let serialized: String =
        serde_yaml::to_string(&system_info).expect("Failed to serialize system info");
    serialized
}

pub(crate) async fn loop_update_system_info() {
    loop {
        // Values that cannot realistically change during runtime are commented out
        //get_cpu_name().await;
        //get_distro().await;
        //get_motherboard().await;
        //get_kernel().await;
        //get_gpus().await;
        get_total_memory().await;
        get_used_memory().await;
        get_disks().await;
        get_local_ip_address().await;
        get_public_ip_address().await;
        get_hostname().await;
        get_boot_time().await;
        get_package_count().await;
    }
}

pub(crate) async fn get_cpu_name() -> String {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_cpu();
    let string: String = sys.global_cpu_info().brand().to_string();
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.cpu = string.clone();
        }
    }
    string
}

pub(crate) async fn get_distro() -> String {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_system();
    let string: String = sys.name().unwrap_or(String::from("Unknown"));
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.distro = string.clone();
        }
    }
    string
}

#[cfg(target_os = "linux")]
pub(crate) async fn get_motherboard() -> String {
    use std::fs;
    let string: String = fs::read_to_string("/sys/class/dmi/id/board_name")
        .unwrap_or(String::from("Unknown"))
        .trim()
        .to_string();

    push_motherboard_value(&string);
    string
}

#[cfg(target_os = "macos")]
pub(crate) async fn get_motherboard() -> String {
    let output_raw: Result<Output, std::io::Error> = Command::new("system_profiler")
        .arg("SPHardwareDataType")
        .output();

    let output: String = match output_raw {
        Err(_) => {
            return "Unknown".to_string();
        }
        Ok(x) => String::from_utf8(x.stdout)
            .expect("non-utf8 response found from call to system_proflier"),
    };

    let string: String = output
        .split("\n")
        .into_iter()
        .filter(|x| x.contains("Model Number:"))
        .map(|y| y.replace("Model Number:", ""))
        .map(|z| z.trim().to_string())
        .collect();

    push_motherboard_value(&string);
    string
}

#[cfg(target_os = "windows")]
pub(crate) async fn get_motherboard() -> String {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let local_machine_key: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path: &str = r"SYSTEM\HardwareConfig\Current";

    let string: String = match local_machine_key.open_subkey(path) {
        Ok(sub_key) => match sub_key.get_value("BaseBoardProduct") {
            Ok(name) => name,
            Err(_) => String::from("Unknown"),
        },
        Err(_) => String::from("Unknown"),
    };

    push_motherboard_value(&string);
    string
}

fn push_motherboard_value(string: &str) {
    let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
        .lock()
        .expect("Failed to lock system info mutex");
    let system_info_option: Option<&mut SystemInfo> = option.as_mut();
    if let Some(system_info) = system_info_option {
        system_info.motherboard = string.to_string();
    }
}

pub(crate) async fn get_kernel() -> String {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_system();
    let string: String = sys.kernel_version().unwrap_or(String::from("Unknown"));
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.kernel = string.clone();
        }
    }
    string
}

pub(crate) async fn get_gpus() -> Vec<String> {
    let instance: Result<gfx_backend_vulkan::Instance, UnsupportedBackend> =
        Instance::create("hayabusa", 1);
    if instance.is_err() {
        return vec![];
    }
    let instance: gfx_backend_vulkan::Instance = instance.unwrap();
    let adapters: Vec<Adapter<Backend>> = instance.enumerate_adapters();

    let mut names: Vec<String> = Vec::new();

    for adapter in adapters {
        names.push(adapter.info.name.to_string());
    }

    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.gpus = names.clone();
        }
    }
    names
}

pub(crate) async fn get_total_memory() -> u64 {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_memory();
    let i: u64 = sys.total_memory();
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.memory.total = i;
        }
    }
    i
}

pub(crate) async fn get_used_memory() -> u64 {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_memory();
    let i: u64 = sys.used_memory();
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.memory.used = i;
        }
    }
    i
}

pub(crate) async fn get_disks() -> Vec<Disk> {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_disks();
    let sys_disks: &[sysinfo::Disk] = sys.disks();
    let mut disks: Vec<Disk> = Vec::new();
    for disk in sys_disks {
        let name: String = disk.mount_point().to_string_lossy().to_string();
        let used: u64 = disk.total_space() - disk.available_space();
        let total: u64 = disk.total_space();
        let disk: Disk = Disk { name, used, total };
        disks.push(disk);
    }
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.disks = disks.clone();
        }
    }
    disks
}

pub(crate) async fn get_local_ip_address() -> String {
    let local_ip: String = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(_) => "Unknown".to_string(),
    };
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.local_ip = local_ip.clone();
        }
    }
    local_ip
}

pub(crate) async fn get_public_ip_address() -> String {
    let client: Client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build reqwest client");
    let string = match client.get("https://ident.me").send().await {
        Ok(res) => res.text().await.expect("Failed to get public IP address"),
        Err(_) => "Unknown".to_string(),
    };
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.public_ip = string.clone();
        }
    }
    string
}

#[cfg(target_os = "linux")]
pub(crate) async fn get_hostname() -> String {
    use std::fs;
    let string: String = fs::read_to_string("/etc/hostname")
        .unwrap_or(String::from("Unknown"))
        .trim()
        .to_string();

    push_hostname(&string);
    string
}

#[cfg(target_os = "macos")]
pub(crate) async fn get_hostname() -> String {
    let output_raw: Result<Output, std::io::Error> = Command::new("hostname").arg("-f").output();

    let out = match output_raw {
        Err(_) => {
            return "Unknown".to_string();
        }
        Ok(x) => {
            String::from_utf8(x.stdout).expect("non-utf8 response found from call to hostname")
        }
    };
    let string = out.trim().to_owned();

    push_hostname(&string);
    string
}

#[cfg(target_os = "windows")]
pub(crate) async fn get_hostname() -> String {
    let output: Output = Command::new("hostname")
        .output()
        .expect("Failed to execute command");

    let hostname: String = String::from_utf8_lossy(&output.stdout).trim().to_string();

    push_hostname(&hostname);
    hostname
}

fn push_hostname(string: &str) {
    let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
        .lock()
        .expect("Failed to lock system info mutex");
    let system_info_option: Option<&mut SystemInfo> = option.as_mut();
    if let Some(system_info) = system_info_option {
        system_info.hostname = string.to_string();
    }
}

pub(crate) async fn get_boot_time() -> u64 {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_system();
    let i: u64 = sys.boot_time();
    i
}
