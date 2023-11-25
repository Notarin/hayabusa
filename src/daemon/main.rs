use crate::daemon::fetch_info::{loop_update_system_info, serialize_fetch, SystemInfo, SYS};
use crate::{daemon::fetch_info, SOCKET_PATH};
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use lazy_static::lazy_static;
use std::fs::Permissions;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Mutex, MutexGuard};
use sysinfo::{System, SystemExt};

lazy_static! {
    pub(crate) static ref SYSTEM_INFO_MUTEX: Mutex<Option<SystemInfo>> = Mutex::new(None);
};

pub(crate) async fn main() {
    println!("Running as daemon");
    initialize_system_info().await;

    let socket_path: String = SOCKET_PATH.clone();

    #[cfg(target_os = "linux")]
    if std::path::Path::new(&socket_path).exists() {
        std::fs::remove_file(&socket_path).expect("Failed to remove socket");
    }

    tokio::spawn(loop_update_system_info());

    // The listener is the IPC server that listens for connections from the fetch client
    let listener: LocalSocketListener =
        LocalSocketListener::bind(socket_path.clone()).expect("Failed to bind to socket");

    // If other users don't have read and write permissions, then the fetch client won't be able
    // to connect to the socket
    let permissions = Permissions::from_mode(0o666); // Read and write for everyone
    std::fs::set_permissions(&socket_path, permissions).expect("Failed to set permissions");

    println!("Listening on {}", socket_path);

    // Here is the infinite loop that listens for connections from the fetch client
    for stream in listener.incoming() {
        let mut client: LocalSocketStream = stream.expect("Failed to connect to client");
        let fetch: String = serialize_fetch();
        client
            .write_all(fetch.as_bytes())
            .expect("Failed to send message!");
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
        let mut system_info_mutex_guard: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        *system_info_mutex_guard =
            Option::from(Some(system_info)).expect("Failed to initialize system info");
    }
}
