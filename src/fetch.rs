use std::borrow::Cow;
use std::io::Read;
use std::sync::MutexGuard;
use interprocess::local_socket::LocalSocketStream;
use crate::SOCKET_PATH;

pub(crate) fn main() {
    let socket_path: String;
    {
        let socket_path_mutex: MutexGuard<String> = SOCKET_PATH.lock().unwrap();
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
    client.read(&mut buffer).unwrap();
    let string: Cow<str> = String::from_utf8_lossy(&buffer);
    println!("{}", string);
}
