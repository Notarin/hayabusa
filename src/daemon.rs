use std::sync::MutexGuard;
use sysinfo::{System, SystemExt};
use tokio::task;
use tokio::task::JoinHandle;
use crate::fetch_info;
use crate::fetch_info::{Disk, Memory, SYS, SystemInfo};

pub(crate) async fn main() {
    {
        let mut sys: MutexGuard<System> = SYS.lock().unwrap();
        sys.refresh_all();
    }

    let cpu_future: JoinHandle<String> = task::spawn(fetch_info::get_cpu_name());
    let distro_future: JoinHandle<String> = task::spawn(fetch_info::get_distro());
    let motherboard_future: JoinHandle<String> = task::spawn(fetch_info::get_motherboard());
    let kernel_future: JoinHandle<String> = task::spawn(fetch_info::get_kernel());
    let gpus_future: JoinHandle<Vec<String>> = task::spawn(fetch_info::get_gpus());
    let memory_future: JoinHandle<Memory> = task::spawn(async {
        Memory {
            used: fetch_info::get_used_memory().await,
            total: fetch_info::get_total_memory().await,
        }
    });
    let disks_future: JoinHandle<Vec<Disk>> = task::spawn(fetch_info::get_disks());
    let local_ip_future: JoinHandle<String> = task::spawn(fetch_info::get_local_ip_address());
    let public_ip_future: JoinHandle<String> = task::spawn(fetch_info::get_public_ip_address());

    let cpu: String = cpu_future.await.unwrap();
    let distro: String = distro_future.await.unwrap();
    let motherboard: String = motherboard_future.await.unwrap();
    let kernel: String = kernel_future.await.unwrap();
    let gpus: Vec<String> = gpus_future.await.unwrap();
    let memory: Memory = memory_future.await.unwrap();
    let disks: Vec<Disk> = disks_future.await.unwrap();
    let local_ip: String = local_ip_future.await.unwrap();
    let public_ip: String = public_ip_future.await.unwrap();

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

    let distro: String = "Distro: ".to_owned() + &*system_info.distro;
    let cpu: String = "CPU: ".to_owned() + &*system_info.cpu;
    let motherboard: String = "Motherboard: ".to_owned() + &*system_info.motherboard;
    let kernel: String = "Kernel: ".to_owned() + &*system_info.kernel;
    let gpus: String = "GPU: ".to_owned() + &*system_info.gpus.join("\n");
    let total_memory_parsed: f64 = system_info.memory.total as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_memory_parsed: f64 = system_info.memory.used as f64 / 1024.0 / 1024.0 / 1024.0;
    let memory: String = "".to_owned()
        + "Memory: "
        + &*format!("{:.2} GiB/{:.2} GiB", used_memory_parsed, total_memory_parsed);
    let disks: String = system_info.disks.iter().cloned().map(|disk| {
        let used_parsed: f64 = disk.used as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_parsed: f64 = disk.total as f64 / 1024.0 / 1024.0 / 1024.0;
        format!("Disk: {}: {:.2} GiB/{:.2} GiB", disk.name, used_parsed, total_parsed)
    }).collect::<Vec<String>>().join("\n");
    let local_ip: String = "Local IP: ".to_owned() + &*system_info.local_ip;
    let public_ip: String = "Public IP: ".to_owned() + &*system_info.public_ip;


    let final_fetch: String = "".to_owned()
        + &*distro + "\n"
        + &*cpu + "\n"
        + &*motherboard + "\n"
        + &*kernel + "\n"
        + &*gpus + "\n"
        + &*memory + "\n"
        + &*disks + "\n"
        + &*local_ip + "\n"
        + &*public_ip;

    println!("{}", final_fetch);
}