use crate::ascii_art::main::AllArt;
use crate::client::{lua, polish_fetch};
use crate::config::toml::{AsciiSize, TomlConfig, TOML_CONFIG_OBJECT};
use crate::daemon::fetch_info::SystemInfo;
use crate::{ascii_art, SOCKET_PATH};
use interprocess::local_socket::LocalSocketStream;
use std::borrow::Cow;
use std::io::Read;

pub(crate) fn main() {
    let socket_path: String = SOCKET_PATH.clone();
    let mut client: LocalSocketStream = LocalSocketStream::connect(socket_path.clone())
        .unwrap_or_else(|_| {
            // I should really set up some automatic way to set up the system service
            // either that or I'll defer it to pre-runtime
            // update: I'm deferring it to pre-runtime, a makefile
            eprintln!(
                "Failed to connect to the {} socket, is the system service running?",
                socket_path.clone()
            );
            std::process::exit(1);
        });
    let mut buffer: Vec<u8> = vec![0u8; 65536];
    let bytes_read: usize = client
        .read(&mut buffer)
        .expect("Failed to read from socket");
    buffer.truncate(bytes_read);
    let string: Cow<str> = String::from_utf8_lossy(&buffer);
    let system_info: SystemInfo =
        serde_yaml::from_str(&string).expect("Failed to deserialize system info");

    let result: String = lua::execute_lua(system_info.clone());

    let fetch: String = polish_fetch::main(&system_info, result);

    println!("{}", fetch);
}

pub(crate) fn get_ascii_art(distro: &str) -> String {
    let config: TomlConfig = TOML_CONFIG_OBJECT.clone();
    if !config.ascii_art.ascii_art_file.is_empty() {
        return get_ascii_file(config.ascii_art.ascii_art_file);
    }

    // This is a hack to ensure the developer(me) doesn't make a dumb mistake
    // and forget to actually add the ascii art when creating a new logo
    this_does_nothing(ascii_art::main::ALL_ART);

    let art_distro = match distro {
        "Arch Linux" => ascii_art::main::ALL_ART.arch,
        "Windows" => ascii_art::main::ALL_ART.windows,
        "Ubuntu" => ascii_art::main::ALL_ART.ubuntu,
        "Gentoo" => ascii_art::main::ALL_ART.gentoo,
        _ => ascii_art::main::ALL_ART.fallback,
    };
    match config.ascii_art.size {
        AsciiSize::Big => art_distro.big,
        AsciiSize::Small => art_distro.small,
    }
    .to_string()
}

fn this_does_nothing(
    AllArt {
        arch,
        windows,
        ubuntu,
        fallback,
        gentoo,
    }: AllArt,
) -> AllArt {
    AllArt {
        arch,
        windows,
        ubuntu,
        fallback,
        gentoo,
    }
}

fn get_ascii_file(location: String) -> String {
    let mut file: String = String::new();
    let mut file_handle: std::fs::File =
        std::fs::File::open(location).expect("Failed to open file");
    file_handle
        .read_to_string(&mut file)
        .expect("Failed to read file");
    file
}
