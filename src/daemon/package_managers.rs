use std::process::{Command, Output};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Packages {
    pub(crate) pacman: u64,
    pub(crate) winget: u64,
    pub(crate) dnf: u64,
}

pub(crate) async fn get_package_count() -> Packages {
    let pacman: u64 = get_pacman_package_count().await.unwrap_or(0);
    let winget: u64 = get_winget_package_count().await.unwrap_or(0);
    let dnf: u64 = get_dnf_package_count().await.unwrap_or(0);

    Packages {
        pacman,
        winget,
        dnf,
    }
}

pub(crate) async fn get_winget_package_count() -> Result<u64, String> {
    let output: Output = Command::new("winget")
        .arg("list")
        .output()
        .map_err(|e| e.to_string())?;

    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let count: u64 = stdout.lines().count() as u64;

    Ok(count)
}

pub(crate) async fn get_pacman_package_count() -> Result<u64, String> {
    let output: Output = Command::new("pacman")
        .arg("-Q")
        .output()
        .map_err(|e| e.to_string())?;

    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let count: u64 = stdout.lines().count() as u64;

    Ok(count)
}

pub(crate) async fn get_dnf_package_count() -> Result<u64, String> {
    let output: Output = Command::new("dnf")
        .arg("list")
        .output()
        .map_err(|e| e.to_string())?;

    let stdout: String = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let mut count: u64 = stdout.lines().count() as u64;
    count -= 1; // remove the first line, its a metadata line

    Ok(count)
}