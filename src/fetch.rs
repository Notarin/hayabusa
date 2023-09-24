use std::borrow::Cow;
use std::io::{Read, Write};
use std::sync::MutexGuard;
use interprocess::local_socket::LocalSocketStream;
use crate::SOCKET_PATH;
use crate::config::load_lua_config;

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
    let lua_config: String = load_lua_config();
    client.write_all(lua_config.as_bytes()).expect("Failed to send lua!");
    let mut buffer: Vec<u8> = vec![0u8; 65536];
    let bytes_read: usize = client.read(&mut buffer).expect("Failed to read from socket");
    buffer.truncate(bytes_read);
    let string: Cow<str> = String::from_utf8_lossy(&buffer);
    println!("{}", string);
}
