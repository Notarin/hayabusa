use rlua::{Context, Lua, Table};
use crate::client::client_info::main::environmental_variable_table;
use crate::config::config::load_lua_config;
use crate::daemon::fetch_info::SystemInfo;

//noinspection SpellCheckingInspection
pub(crate) fn execute_lua(system_info: SystemInfo) -> String {
    let lua_config: String = load_lua_config();
    let lua: Lua = Lua::new();
    let mut fetch: String = "".to_string();

    lua.context(|lua_ctx| {
        let globals: Table = lua_ctx.globals();
        let simple_table = system_info_table(system_info, lua_ctx);
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

fn system_info_table(system_info: SystemInfo, lua_ctx: Context) -> Table {
    let table: Table = lua_ctx.create_table().unwrap();
    table.set("distro", &*system_info.distro).unwrap();
    table.set("cpu", &*system_info.cpu).unwrap();
    table.set("motherboard", &*system_info.motherboard).unwrap();
    table.set("kernel", &*system_info.kernel).unwrap();
    let gpus_table: Table = gpu_table(system_info.clone(), lua_ctx);
    table.set("gpus", gpus_table).unwrap();
    let memory_table: Table = lua_ctx.create_table().unwrap();
    memory_table.set("used", system_info.memory.used).unwrap();
    memory_table.set("total", system_info.memory.total).unwrap();
    table.set("memory", memory_table).unwrap();
    let disks_table = disk_table(system_info.clone(), lua_ctx);
    table.set("disks", disks_table).unwrap();
    table.set("local_ip", &*system_info.local_ip).unwrap();
    table.set("public_ip", &*system_info.public_ip).unwrap();
    table.set("hostname", &*system_info.hostname).unwrap();
    table
}

fn disk_table(system_info: SystemInfo, lua_ctx: Context) -> Table {
    let disks_table: Table = lua_ctx.create_table().unwrap();
    for (index, disk) in system_info.disks.iter().enumerate() {
        let disk_table: Table = lua_ctx.create_table().unwrap();
        disk_table.set("name", disk.name.clone()).unwrap();
        disk_table.set("used", disk.used).unwrap();
        disk_table.set("total", disk.total).unwrap();
        disks_table.set(index + 1, disk_table).unwrap();
    }
    disks_table
}

fn gpu_table(system_info: SystemInfo, lua_ctx: Context) -> Table {
    let gpus_table: Table = lua_ctx.create_table().unwrap();
    for (index, gpu) in system_info.gpus.iter().enumerate() {
        gpus_table.set(index + 1, gpu.clone()).unwrap();
    }
    gpus_table
}
