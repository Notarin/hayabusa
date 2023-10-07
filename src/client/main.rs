use std::borrow::Cow;
use std::io::Read;
use interprocess::local_socket::LocalSocketStream;
use crate::{ascii_art, SOCKET_PATH};
use crate::client::{lua, polish_fetch};
use crate::config::toml::{AsciiSize, TOML_CONFIG_OBJECT, TomlConfig};
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
    let result: String = lua::execute_lua(system_info.clone());

    let fetch: String = polish_fetch::main(system_info, result);

    println!("{}", fetch);
}

pub(crate) fn get_ascii_art(distro: String) -> String {
    let config: TomlConfig = TOML_CONFIG_OBJECT.clone();
    let art_distro = match distro.as_str() {
        "Arch Linux" => ascii_art::main::ALL_ART.arch,
        "Windows" => ascii_art::main::ALL_ART.windows,
        _ => ascii_art::main::ALL_ART.fallback,
    };
    match config.ascii_art.size {
        AsciiSize::Big => art_distro.big,
        AsciiSize::Small => art_distro.small,
    }.to_string()
}
