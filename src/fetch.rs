use std::borrow::Cow;
use std::io::Read;
use interprocess::local_socket::LocalSocketStream;
use rlua::{Lua, Table};
use regex::Regex;
use unicode_width::UnicodeWidthStr;
use crate::{ascii_art, SOCKET_PATH};
use crate::config::load_lua_config;
use crate::fetch_info::SystemInfo;

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
    let ascii_art: String = get_ascii_art(system_info.distro.clone());
    let final_fetch: String = put_ascii_left(ascii_art, result);
    println!("{}", final_fetch);
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

fn get_ascii_art(distro: String) -> String {
    match distro.as_str() {
        "Arch Linux" => ascii_art::ALL_ART.arch.big,
        "Windows" => ascii_art::ALL_ART.windows.big,
        _ => "none",
    }.to_string()
}

fn put_ascii_left(ascii_art: String, fetch: String) -> String {
    let parsed_art: String = parse_ascii_art(ascii_art.clone());
    let ansi_free_art: String = remove_ansi(ascii_art.clone());
    let width: usize = ansi_free_art
        .lines()
        .map(|line| UnicodeWidthStr::width(line))
        .max()
        .unwrap_or(0);
    let height: usize = ansi_free_art.lines().count();
    let fetch_height = fetch.lines().count();

    let full_height = std::cmp::max(height, fetch_height);

    let mut result: String = "".to_string();
    result.push_str(&parsed_art);
    result.push_str(&format!("\x1b[{}A", height-1));
    let fetch_modified = fetch.replace('\n', &format!("\n\x1b[{}C", width));
    result.push_str(&fetch_modified);
    result.push_str(&format!("\x1b[{}B", full_height));
    result.push_str("\x1b[0D");

    result
}

fn parse_ascii_art(ascii_art: String) -> String {
    let mut result: String = ascii_art;
    let color0: String = "\x1b[38;5;0m".to_string();
    let color1: String = "\x1b[38;5;1m".to_string();
    let color2: String = "\x1b[38;5;2m".to_string();
    let color3: String = "\x1b[38;5;3m".to_string();
    let color4: String = "\x1b[38;5;4m".to_string();
    let color5: String = "\x1b[38;5;5m".to_string();
    let color6: String = "\x1b[38;5;6m".to_string();
    let color7: String = "\x1b[38;5;7m".to_string();
    let color8: String = "\x1b[38;5;8m".to_string();
    let color9: String = "\x1b[38;5;9m".to_string();
    let color10: String = "\x1b[38;5;10m".to_string();
    let color11: String = "\x1b[38;5;11m".to_string();
    let color12: String = "\x1b[38;5;12m".to_string();
    let color13: String = "\x1b[38;5;13m".to_string();
    let color14: String = "\x1b[38;5;14m".to_string();
    let color15: String = "\x1b[38;5;15m".to_string();
    let reset: String = "\x1b[0m".to_string();
    result = result.replace("{{color0}}", &color0);
    result = result.replace("{{color1}}", &color1);
    result = result.replace("{{color2}}", &color2);
    result = result.replace("{{color3}}", &color3);
    result = result.replace("{{color4}}", &color4);
    result = result.replace("{{color5}}", &color5);
    result = result.replace("{{color6}}", &color6);
    result = result.replace("{{color7}}", &color7);
    result = result.replace("{{color8}}", &color8);
    result = result.replace("{{color9}}", &color9);
    result = result.replace("{{color10}}", &color10);
    result = result.replace("{{color11}}", &color11);
    result = result.replace("{{color12}}", &color12);
    result = result.replace("{{color13}}", &color13);
    result = result.replace("{{color14}}", &color14);
    result = result.replace("{{color15}}", &color15);
    result = result.replace("{{reset}}", &reset);
    result
}

fn remove_ansi(ascii_art: String) -> String {
    let regex: Regex = Regex::new(r"(\{\{.*?}})").unwrap();
    let result: String = regex.replace_all(&ascii_art, "").to_string();
    result
}