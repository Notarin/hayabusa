use std::borrow::Cow;
use std::io::Read;
use interprocess::local_socket::LocalSocketStream;
use rlua::{Lua, Table};
use crate::{ascii_art, SOCKET_PATH};
use crate::client::polish_fetch;
use crate::config::config::{load_lua_config};
use crate::daemon::fetch_info::SystemInfo;

pub(crate) fn main() {
    let socket_path: String = SOCKET_PATH.clone();
    let mut client: LocalSocketStream = LocalSocketStream::connect(socket_path.clone())
        .unwrap_or_else(|_| {
            eprintln!(
                "Failed to connect to the {} socket, have you started the system service?",
                socket_path.clone()
            );
            std::process::exit(1);
        });
    let mut buffer: Vec<u8> = vec![0u8; 65536];
    let bytes_read: usize = client.read(&mut buffer).expect("Failed to read from socket");
    buffer.truncate(bytes_read);
    let string: Cow<str> = String::from_utf8_lossy(&buffer);
    let system_info: SystemInfo = serde_yaml::from_str(&string)
        .expect("Failed to deserialize system info");
    let result: String = execute_lua(system_info.clone());

    let fetch: String = polish_fetch::main(system_info, result);

    println!("{}", fetch);
}

//noinspection SpellCheckingInspection
fn execute_lua(system_info: SystemInfo) -> String {
    let lua_config: String = load_lua_config();
    let lua = Lua::new();
    let mut fetch: String = "".to_string();

    lua.context(|lua_ctx| {
        let globals: Table = lua_ctx.globals();
        globals.set("distro", system_info.distro).unwrap();
        globals.set("cpu", &*system_info.cpu).unwrap();
        globals.set("motherboard", &*system_info.motherboard).unwrap();
        globals.set("kernel", &*system_info.kernel).unwrap();
        let gpus_table: Table = lua_ctx.create_table().unwrap();
        for (index, gpu) in system_info.gpus.iter().enumerate() {
            gpus_table.set(index + 1, gpu.clone()).unwrap();
        }
        globals.set("gpus", gpus_table).unwrap();
        let memory_table: Table = lua_ctx.create_table().unwrap();
        memory_table.set("used", system_info.memory.used).unwrap();
        memory_table.set("total", system_info.memory.total).unwrap();
        globals.set("memory", memory_table).unwrap();
        let disks_table: Table = lua_ctx.create_table().unwrap();
        for (index, disk) in system_info.disks.iter().enumerate() {
            let disk_table: Table = lua_ctx.create_table().unwrap();
            disk_table.set("name", disk.name.clone()).unwrap();
            disk_table.set("used", disk.used).unwrap();
            disk_table.set("total", disk.total).unwrap();
            disks_table.set(index + 1, disk_table).unwrap();
        }
        globals.set("disks", disks_table).unwrap();
        globals.set("local_ip", &*system_info.local_ip).unwrap();
        globals.set("public_ip", &*system_info.public_ip).unwrap();

        let result: String = match lua_ctx.load(&lua_config).exec() {
            Ok(_) => globals.get("result").unwrap(),
            Err(e) => "Failed to execute lua script: ".to_string() + &e.to_string(),
        };
        fetch = result;
    });
    fetch
}

pub(crate) fn get_ascii_art(distro: String) -> String {
    match distro.as_str() {
        "Arch Linux" => ascii_art::main::ALL_ART.arch.big,
        "Windows" => ascii_art::main::ALL_ART.windows.big,
        _ => "none",
    }.to_string()
}
