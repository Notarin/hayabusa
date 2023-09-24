use std::sync::{Mutex, MutexGuard};
use reqwest::Client;
use std::time::Duration;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use local_ip_address::local_ip;
use gfx_hal::adapter::Adapter;
use gfx_backend_vulkan::Backend;
use gfx_hal::Instance;
use lazy_static::lazy_static;
use rlua::{Lua, Table};
use tokio::task::JoinHandle;
use crate::daemon::SYSTEM_INFO_MUTEX;

lazy_static! {
    pub(crate) static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

#[derive(Clone)]
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

#[derive(Clone)]
pub(crate) struct Memory {
    pub(crate) used: u64,
    pub(crate) total: u64,
}

pub(crate) async fn fetch_all() -> SystemInfo {
    let cpu_future: JoinHandle<String> = tokio::spawn(get_cpu_name());
    let distro_future: JoinHandle<String> = tokio::spawn(get_distro());
    let motherboard_future: JoinHandle<String> = tokio::spawn(get_motherboard());
    let kernel_future: JoinHandle<String> = tokio::spawn(get_kernel());
    let gpus_future: JoinHandle<Vec<String>> = tokio::spawn(get_gpus());
    let memory_future: JoinHandle<Memory> = tokio::spawn(async {
        Memory {
            used: get_used_memory().await,
            total: get_total_memory().await,
        }
    });
    let disks_future: JoinHandle<Vec<Disk>> = tokio::spawn(get_disks());
    let local_ip_future: JoinHandle<String> = tokio::spawn(get_local_ip_address());
    let public_ip_future: JoinHandle<String> = tokio::spawn(get_public_ip_address());

    let cpu: String = cpu_future.await.expect("get_cpu_name thread panicked!");
    let distro: String = distro_future.await.expect("get_distro thread panicked!");
    let motherboard: String = motherboard_future.await.expect("get_motherboard thread panicked!");
    let kernel: String = kernel_future.await.expect("get_kernel thread panicked!");
    let gpus: Vec<String> = gpus_future.await.expect("get_gpus thread panicked!");
    let memory: Memory = memory_future.await.expect("get_memory thread panicked!");
    let disks: Vec<Disk> = disks_future.await.expect("get_disks thread panicked!");
    let local_ip: String = local_ip_future.await.expect("get_local_ip_address thread panicked!");
    let public_ip: String = public_ip_future.await.expect("get_public_ip_address thread panicked!");

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
    };
    system_info
}

pub(crate) fn compile_fetch(lua_file: String) -> String {
    let system_info: SystemInfo = SYSTEM_INFO_MUTEX.lock()
        .expect("Failed to lock system info mutex")
        .clone().expect("System info has not been initialized");

    let lua = Lua::new();
    let mut final_fetch: String = "".to_string();

    lua.context(|lua_ctx| {
        let globals: Table = lua_ctx.globals();

        {
            // UNSAFE GLOBALS, DANGER!!! the daemon is intended to be run as a system service
            // which mean root is running this lua, this means that using these globals
            // the user could effectively gain root access.
            // To prevent this we disable the following globals:
            globals.set("os", rlua::Value::Nil).expect("Failed to set os to nil");
            globals.set("io", rlua::Value::Nil).expect("Failed to set io to nil");
            globals.set("debug", rlua::Value::Nil).expect("Failed to set debug to nil");
            globals.set("package", rlua::Value::Nil).expect("Failed to set package to nil");
            globals.set("loadfile", rlua::Value::Nil).expect("Failed to set loadfile to nil");
            globals.set("dofile", rlua::Value::Nil).expect("Failed to set dofile to nil");
            globals.set("load", rlua::Value::Nil).expect("Failed to set load to nil");
            globals.set("assert", rlua::Value::Nil).expect("Failed to set assert to nil");
            globals.set("collectgarbage", rlua::Value::Nil).expect("Failed to set collectgarbage to nil");
            globals.set("getmetatable", rlua::Value::Nil).expect("Failed to set getmetatable to nil");
            globals.set("setmetatable", rlua::Value::Nil).expect("Failed to set setmetatable to nil");
            globals.set("rawequal", rlua::Value::Nil).expect("Failed to set rawequal to nil");
            globals.set("rawget", rlua::Value::Nil).expect("Failed to set rawget to nil");
            globals.set("rawset", rlua::Value::Nil).expect("Failed to set rawset to nil");
            globals.set("require", rlua::Value::Nil).expect("Failed to set require to nil");
            globals.set("module", rlua::Value::Nil).expect("Failed to set module to nil");
            globals.set("package", rlua::Value::Nil).expect("Failed to set package to nil");
            globals.set("loadlib", rlua::Value::Nil).expect("Failed to set loadlib to nil");
            globals.set("print", rlua::Value::Nil).expect("Failed to set print to nil");
            // We also disable the following metamethods:
            globals.set("__index", rlua::Value::Nil).expect("Failed to set __index to nil");
            globals.set("__newindex", rlua::Value::Nil).expect("Failed to set __newindex to nil");
            globals.set("__metatable", rlua::Value::Nil).expect("Failed to set __metatable to nil");
        }

        globals.set("distro", system_info.distro).unwrap();
        globals.set("cpu", &*system_info.cpu).unwrap();
        globals.set("motherboard", &*system_info.motherboard).unwrap();
        globals.set("kernel", &*system_info.kernel).unwrap();
        let gpus_table: rlua::Table = lua_ctx.create_table().unwrap();
        for (index, gpu) in system_info.gpus.iter().enumerate() {
            gpus_table.set(index + 1, gpu.clone()).unwrap();
        }
        globals.set("gpus", gpus_table).unwrap();
        let memory_table: rlua::Table = lua_ctx.create_table().unwrap();
        memory_table.set("used", system_info.memory.used).unwrap();
        memory_table.set("total", system_info.memory.total).unwrap();
        globals.set("memory", memory_table).unwrap();
        let disks_table: rlua::Table = lua_ctx.create_table().unwrap();
        for (index, disk) in system_info.disks.iter().enumerate() {
            let disk_table: rlua::Table = lua_ctx.create_table().unwrap();
            disk_table.set("name", disk.name.clone()).unwrap();
            disk_table.set("used", disk.used).unwrap();
            disk_table.set("total", disk.total).unwrap();
            disks_table.set(index + 1, disk_table).unwrap();
        }
        globals.set("disks", disks_table).unwrap();
        globals.set("local_ip", &*system_info.local_ip).unwrap();
        globals.set("public_ip", &*system_info.public_ip).unwrap();

        let result: String = match lua_ctx.load(&lua_file).exec() {
            Ok(_) => globals.get("result").unwrap(),
            Err(e) => "Failed to execute lua script: ".to_string() + &e.to_string(),
        };
        final_fetch = result;
    });

    final_fetch
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
    }
}

pub(crate) async fn get_cpu_name() -> String {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_cpu();
    let string: String = sys.global_cpu_info().brand().to_string();
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
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
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
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
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.motherboard = string.clone();
        }
    }
    string
}

#[cfg(target_os = "windows")]
pub(crate) async fn get_motherboard() -> String {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let local_machine_key: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path: &str = r"SYSTEM\HardwareConfig\Current";

    let string: String = match local_machine_key.open_subkey(path) {
        Ok(sub_key) => {
            match sub_key.get_value("BaseBoardProduct") {
                Ok(name) => name,
                Err(_) => String::from("Unknown"),
            }
        }
        Err(_) => String::from("Unknown"),
    };

    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.motherboard = string.clone();
        }
    }
    string
}

pub(crate) async fn get_kernel() -> String {
    let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
    sys.refresh_system();
    let string: String = sys.kernel_version().unwrap_or(String::from("Unknown"));
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.kernel = string.clone();
        }
    }
    string
}

pub(crate) async fn get_gpus() -> Vec<String> {

    let instance: gfx_backend_vulkan::Instance =
        Instance::create("hayabusa", 1).expect("Failed to create Vulkan instance");
    let adapters: Vec<Adapter<Backend>> = instance.enumerate_adapters();

    let mut names: Vec<String> = Vec::new();

    for adapter in adapters {
        names.push(adapter.info.name.to_string());
    }

    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
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
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
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
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
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
        let disk: Disk = Disk {
            name,
            used,
            total,
        };
        disks.push(disk);
    }
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
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
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.local_ip = local_ip.clone();
        }
    }
    local_ip
}

pub(crate) async fn get_public_ip_address() -> String {
    let client: Client = Client::builder().timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build reqwest client");
    let string = match client.get("https://ident.me").send().await {
        Ok(res) => res.text().await.expect("Failed to get public IP address"),
        Err(_) => "Unknown".to_string(),
    };
    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.public_ip = string.clone();
        }
    }
    string
}
