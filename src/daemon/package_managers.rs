use crate::daemon::fetch_info::SystemInfo;
use crate::daemon::main::SYSTEM_INFO_MUTEX;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Command, Output};
use std::sync::MutexGuard;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Packages {
    pub(crate) pacman: u64,
    pub(crate) winget: u64,
    pub(crate) dnf: u64,
    pub(crate) apt: u64,
    pub(crate) brew: u64,
    pub(crate) emerge: u64,
}

pub(crate) async fn get_package_count() -> Packages {
    let pacman = get_pacman_package_count();
    let winget = get_winget_package_count();
    let dnf = get_dnf_package_count();

    let pacman: u64 = pacman.await.unwrap_or(0);
    let winget: u64 = winget.await.unwrap_or(0);
    let dnf: u64 = dnf.await.unwrap_or(0);
    let apt: u64 = get_apt_package_count().await.unwrap_or(0);
    let brew: u64 = get_brew_package_count().await.unwrap_or(0);
    let emerge: u64 = get_emerge_package_count().await.unwrap_or(0);

    let packages = Packages {
        pacman,
        winget,
        dnf,
        apt,
        brew,
        emerge,
    };

    {
        let mut option: MutexGuard<Option<SystemInfo>> = SYSTEM_INFO_MUTEX
            .lock()
            .expect("Failed to lock system info mutex");
        let system_info_option: Option<&mut SystemInfo> = option.as_mut();
        if let Some(system_info) = system_info_option {
            system_info.packages = packages.clone();
        }
    }
    packages
}

pub(crate) async fn get_winget_package_count() -> Result<u64, String> {
    let output: Output = Command::new("winget")
        .arg("list")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("non-zero exit".to_string());
    }

    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let count: u64 = stdout.lines().count() as u64;

    Ok(count)
}

pub(crate) async fn get_apt_package_count() -> Result<u64, String> {
    let output: Output = Command::new("apt")
        .arg("list")
        .arg("--installed")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("non-zero exit".to_string());
    }
    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let mut count: u64 = stdout.lines().count() as u64;
    count -= 1; // remove the first line, its a metadata line

    Ok(count)
}

pub(crate) async fn get_brew_package_count() -> Result<u64, String> {
    let output: Output = Command::new("brew")
        .arg("list")
        .arg("-1l")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("non-zero exit".to_string());
    }
    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let mut count: u64 = stdout.lines().count() as u64;
    count -= 5; // remove 5 lines, they are metadata

    Ok(count)
}

pub(crate) async fn get_pacman_package_count() -> Result<u64, String> {
    let output: Output = Command::new("pacman")
        .arg("-Q")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("non-zero exit".to_string());
    }

    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let count: u64 = stdout.lines().count() as u64;

    Ok(count)
}

pub(crate) async fn get_dnf_package_count() -> Result<u64, String> {
    let output: Output = Command::new("dnf")
        .arg("list")
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("non-zero exit".to_string());
    }

    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let mut count: u64 = stdout.lines().count() as u64;
    count -= 1; // remove the first line, its a metadata line

    Ok(count)
}

pub(crate) async fn get_emerge_package_count() -> Result<u64, String> {
    let pkg_dir = "/var/db/pkg/";

    match fs::read_dir(pkg_dir) {
        Ok(entries) => {
            let mut count = 0u64;
            for entry in entries.filter_map(Result::ok) {
                if entry.path().is_dir() {
                    count += fs::read_dir(entry.path()).map_or(0, |d| d.count() as u64);
                }
            }
            Ok(count)
        }
        Err(e) => Err(e.to_string()),
    }
}
