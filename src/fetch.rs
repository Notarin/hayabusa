use std::borrow::Cow;
use std::io::Read;
use std::sync::MutexGuard;
use interprocess::local_socket::LocalSocketStream;
use rlua::{Lua, Table};
use crate::SOCKET_PATH;
use crate::config::load_lua_config;
use crate::fetch_info::SystemInfo;

pub(crate) fn main() {
    let socket_path: String;
    {
        let socket_path_mutex: MutexGuard<String> = SOCKET_PATH.lock()
            .expect("Failed to lock socket path mutex");
        socket_path = socket_path_mutex.clone();
    }
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
    let result: String = execute_lua(system_info);
    println!("{}", result);
}

//noinspection SpellCheckingInspection
fn execute_lua(system_info: SystemInfo) -> String {
    let lua_config: String = load_lua_config();
    let lua = Lua::new();
    let mut fetch: String = "".to_string();

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