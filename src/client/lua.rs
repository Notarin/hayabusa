use rlua::{Context, Lua, Table};
use crate::client::client_info::main::environmental_variable_table;
use crate::config::main::load_lua_config;
use crate::daemon::fetch_info::{Disk, SystemInfo};
use crate::daemon::package_managers::Packages;

//noinspection SpellCheckingInspection
pub(crate) fn execute_lua(system_info: SystemInfo) -> String {
    let lua_config: String = load_lua_config();
    let lua: Lua = Lua::new();
    let mut fetch: String = "".to_string();

    lua.context(|lua_ctx| {
        let globals: Table = lua_ctx.globals();
        let simple_table: Table = system_info_table(system_info, lua_ctx);
        globals.set("system_info", simple_table).unwrap();
        globals.set("environmental_variables", environmental_variable_table(lua_ctx)).unwrap();

        let result: String = match lua_ctx.load(&lua_config).exec() {
            Ok(_) => globals.get("result").unwrap(),
            Err(e) => "Failed to execute lua script: ".to_string() + &e.to_string(),
        };
        fetch = result;
    });
    fetch
}

fn system_info_table(
    SystemInfo {
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
        packages
    }: SystemInfo,
    lua_ctx: Context
) -> Table {
    let table: Table = lua_ctx.create_table().unwrap();
    table.set("distro", &*distro).unwrap();
    table.set("cpu", &*cpu).unwrap();
    table.set("motherboard", &*motherboard).unwrap();
    table.set("kernel", &*kernel).unwrap();
    let gpus_table: Table = gpu_table(gpus, lua_ctx);
    table.set("gpus", gpus_table).unwrap();
    let memory_table: Table = lua_ctx.create_table().unwrap();
    memory_table.set("used", memory.used).unwrap();
    memory_table.set("total", memory.total).unwrap();
    table.set("memory", memory_table).unwrap();
    let disks_table = disk_table(disks, lua_ctx);
    table.set("disks", disks_table).unwrap();
    table.set("local_ip", &*local_ip).unwrap();
    table.set("public_ip", &*public_ip).unwrap();
    table.set("hostname", &*hostname).unwrap();
    table.set("boot_time", boot_time).unwrap();
    let packages_table: Table = packages_table(packages, lua_ctx);
    table.set("packages", packages_table).unwrap();
    table
}

fn packages_table(packages: Packages, lua_ctx: Context) -> Table {
    let packages_table: Table = lua_ctx.create_table().unwrap();
    packages_table.set("pacman", packages.pacman).unwrap();
    packages_table.set("winget", packages.winget).unwrap();
    packages_table.set("dnf", packages.dnf).unwrap();
    packages_table
}

fn disk_table(disks: Vec<Disk>, lua_ctx: Context) -> Table {
    let disks_table: Table = lua_ctx.create_table().unwrap();
    for (index, disk) in disks.iter().enumerate() {
        let disk_table: Table = lua_ctx.create_table().unwrap();
        disk_table.set("name", disk.name.clone()).unwrap();
        disk_table.set("used", disk.used).unwrap();
        disk_table.set("total", disk.total).unwrap();
        disks_table.set(index + 1, disk_table).unwrap();
    }
    disks_table
}

fn gpu_table(gpus: Vec<String>, lua_ctx: Context) -> Table {
    let gpus_table: Table = lua_ctx.create_table().unwrap();
    for (index, gpu) in gpus.iter().enumerate() {
        gpus_table.set(index + 1, gpu.clone()).unwrap();
    }
    gpus_table
}
