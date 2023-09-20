use std::io::Write;
use std::sync::MutexGuard;
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use sysinfo::{System, SystemExt};
use crate::{fetch_info, SOCKET_PATH};
use crate::fetch_info::{SYS, SystemInfo};

pub(crate) async fn main() {
    {
        let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
        sys.refresh_all();
    }

    let system_info: SystemInfo = fetch_info::fetch_all().await;
    
    let final_fetch: String = fetch_info::compile_fetch(system_info);

    let socket_path: String;
    {
        let socket_path_mutex: MutexGuard<String> = SOCKET_PATH.lock()
            .expect("Failed to lock socket path mutex");
        socket_path = socket_path_mutex.clone();
    }

    #[cfg(target_os = "linux")]
    if std::path::Path::new(&socket_path).exists() {
        std::fs::remove_file(&socket_path).expect("Failed to remove socket");
    }

    let listener: LocalSocketListener = LocalSocketListener::bind(socket_path.clone())
        .expect("Failed to bind to socket");
    println!("Listening on {}", socket_path);
    for stream in listener.incoming() {
        let mut client: LocalSocketStream = stream.expect("Failed to connect to client");
        client.write_all(final_fetch.as_bytes()).expect("Failed to send message!");
        println!("Sent fetch!");
    }
    unreachable!("Unexpected termination: Listener loop exited");
}