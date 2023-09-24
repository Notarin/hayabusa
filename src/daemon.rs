use std::io::{Read, Write};
use std::sync::{Mutex, MutexGuard};
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use lazy_static::lazy_static;
use sysinfo::{System, SystemExt};
use crate::{fetch_info, SOCKET_PATH};
use crate::fetch_info::{compile_fetch, loop_update_system_info, SYS, SystemInfo};

lazy_static!(
    pub(crate) static ref SYSTEM_INFO_MUTEX: Mutex<Option<SystemInfo>> = Mutex::new(None);
);

pub(crate) async fn main() {

    initialize_system_info().await;

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

    tokio::spawn(loop_update_system_info());

    // The listener is the IPC server that listens for connections from the fetch client
    let listener: LocalSocketListener = LocalSocketListener::bind(socket_path.clone())
        .expect("Failed to bind to socket");
    println!("Listening on {}", socket_path);

    // Here is the infinite loop that listens for connections from the fetch client
    for stream in listener.incoming() {
        let mut client: LocalSocketStream = stream.expect("Failed to connect to client");
        let mut received: Vec<u8> = vec![0u8; 65536];
        let bytes_read: usize = client.read(&mut received).expect("Failed to read from socket");
        received.truncate(bytes_read);
        let lua: String = String::from_utf8_lossy(&received).to_string();
        let fetch: String = compile_fetch(lua);
        client.write_all(
            fetch.as_bytes()
        ).expect("Failed to send message!");
        println!("Sent fetch!");
    }
    // This will never be reached, due to the fact that listener.incoming() is an infinite loop
    // It will thread block until a new connection is made
    unreachable!("Unexpected termination: Listener loop exited");
}

async fn initialize_system_info() {
    {
        // The system_info crate requires that the sys object be refreshed at least once before
        // any info is available
        let mut sys: MutexGuard<System> = SYS.lock().expect("Failed to lock sys-info mutex");
        sys.refresh_all();
        // Don't forget to always drop the lock on mutexes ASAP
    }
    let system_info: SystemInfo = fetch_info::fetch_all().await;
    {
        // Here is where we initialize the system info struct we've defined ourselves
        let mut system_info_mutex_guard: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX.lock()
            .expect("Failed to lock system info mutex");
        *system_info_mutex_guard = Option::from(Some(system_info))
            .expect("Failed to initialize system info");
    }
}
