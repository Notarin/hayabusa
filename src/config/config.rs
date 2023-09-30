use std::{env, fs};
use std::collections::BTreeMap;
use std::path::Path;
use crate::config::toml::{DEFAULT_TOML_CONFIG, TomlConfig};
use toml::{from_str, to_string, Value};

const LUA_SCRIPT: &str = include_str!("default.lua");

pub(crate) fn load_lua_config() -> String {
    let lua_file_location: String = get_config_location();
    fs::read_to_string(lua_file_location).unwrap_or_else(|_| {
        write_default_lua();
        LUA_SCRIPT.to_string()
    })
}

fn write_default_lua() {
    let lua_file_location: String = get_config_location();
    let path: &Path = Path::new(&lua_file_location);
    let parent_dir: &Path = path.parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).expect("Failed to create config directory");
    }
    fs::write(lua_file_location, LUA_SCRIPT).expect("Failed to write default config.lua");
}

#[cfg(target_os = "linux")]
pub(crate) fn get_config_location() -> String {
    let config_dir: String = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| env::var("HOME").expect("Failed to get $HOME") + "/.config");
    format!("{}/hayabusa/config.lua", config_dir)
}

#[cfg(target_os = "windows")]
pub(crate) fn get_config_location() -> String {
    let config_dir: String = env::var("APPDATA").expect("Failed to get %APPDATA%");
    format!("{}\\hayabusa\\config.lua", config_dir)
}

pub(crate) fn load_toml_config() -> TomlConfig {
    let toml_file_location: String = get_toml_config_location();
    let file_contents: String = fs::read_to_string(&toml_file_location).unwrap_or_else(|_| {
        write_default_toml();
        to_string(&DEFAULT_TOML_CONFIG).unwrap()
    });

    let mut loaded_config: BTreeMap<String, Value> = from_str(&file_contents).unwrap();
    let default_config: BTreeMap<String, Value> = from_str(&to_string(&DEFAULT_TOML_CONFIG).unwrap()).unwrap();

    let mut is_updated = false;
    for (key, value) in default_config {
        if let std::collections::btree_map::Entry::Vacant(e) = loaded_config.entry(key) {
            e.insert(value);
            is_updated = true;
        }
    }

    if is_updated {
        let new_config_str: String = to_string(&loaded_config).unwrap();
        fs::write(toml_file_location, new_config_str).expect("Failed to update config.toml");
    }

    from_str(&to_string(&loaded_config).unwrap()).unwrap()
}

fn write_default_toml() {
    let toml_file_location: String = get_toml_config_location();
    let path: &Path = Path::new(&toml_file_location);
    let parent_dir: &Path = path.parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).expect("Failed to create config directory");
    }
    let contents = to_string(&DEFAULT_TOML_CONFIG).unwrap();
    fs::write(toml_file_location, contents)
        .expect("Failed to write default config.toml");
}

#[cfg(target_os = "linux")]
pub(crate) fn get_toml_config_location() -> String {
    let config_dir: String = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| env::var("HOME").expect("Failed to get $HOME") + "/.config");
    format!("{}/hayabusa/config.toml", config_dir)
}

#[cfg(target_os = "windows")]
pub(crate) fn get_toml_config_location() -> String {
    let config_dir: String = env::var("APPDATA").expect("Failed to get %APPDATA%");
    format!("{}\\hayabusa\\config.toml", config_dir)
}