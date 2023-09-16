use sysinfo::{Cpu, CpuExt, System, SystemExt};

struct SystemInfo<'a> {
    cpu: &'a Cpu,
    distro: String,
}

pub(crate) fn main() {
    let mut sys: System = System::new_all();
    sys.refresh_all();

    let cpu: &Cpu = get_cpu(&sys);
    let distro: String = get_distro(&sys);

    let system_info: SystemInfo = SystemInfo {
        cpu,
        distro,
    };

    let distro = "Distro: ".to_owned() + &*system_info.distro;
    let cpu = "CPU: ".to_owned() + system_info.cpu.brand();
    let final_fetch = distro + "\n" + &*cpu;

    println!("{}", final_fetch);
}

fn get_cpu(sys: &System) -> &Cpu {
    sys.global_cpu_info()
}

fn get_distro(sys: &System) -> String {
    sys.name().unwrap_or(String::from("Unknown"))
}