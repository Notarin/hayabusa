use std::{env, fs};
use std::collections::BTreeMap;
use std::path::Path;
use crate::config::toml::{build_default_toml, TomlConfig};
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

    // Read the configuration file, if it fails, write the default toml.
    let file_contents: String = fs::read_to_string(&toml_file_location)
        .or_else(|_| {
            write_default_toml();
            to_string(&build_default_toml())
        })
        .expect("Failed to handle TOML config file.");

    // Try to parse the read contents directly into the struct.
    if let Ok(config) = from_str::<TomlConfig>(&file_contents) {
        config
    } else {
        // If parsing fails, merge with default and retry.
        let mut loaded_config: BTreeMap<String, Value> = from_str(&file_contents)
            .expect("Failed to parse loaded config to BTreeMap.");
        let default_config_map: BTreeMap<String, Value> = from_str(&to_string(&build_default_toml())
            .expect("Failed to serialize default TOML."))
            .expect("Failed to parse default config to BTreeMap.");

        let was_merged: bool = merge_maps(&mut loaded_config, &default_config_map);
        if was_merged {
            let new_config_str: String = to_string(&loaded_config)
                .expect("Failed to serialize merged config.");
            fs::write(&toml_file_location, new_config_str)
                .expect("Failed to update config.toml after merging.");
        }

        from_str(&to_string(&loaded_config)
            .expect("Failed to serialize merged config for final struct."))
            .expect("Failed to parse final config to TomlConfig.")
    }
}

fn merge_maps(a: &mut BTreeMap<String, Value>, b: &BTreeMap<String, Value>) -> bool {
    let mut was_merged = false;

    for (key, value) in b.iter() {
        match a.entry(key.clone()) {
            std::collections::btree_map::Entry::Vacant(e) => {
                was_merged = true;
                e.insert(value.clone());
            },
            std::collections::btree_map::Entry::Occupied(mut e) => {
                if let Value::Table(a_inner) = e.get_mut() {
                    if let Value::Table(ref b_inner) = value {
                        let mut a_btree: BTreeMap<String, Value> = a_inner.clone().into_iter().collect();
                        let b_btree: BTreeMap<String, Value> = b_inner.clone().into_iter().collect();
                        if merge_maps(&mut a_btree, &b_btree) {
                            was_merged = true;
                            *a_inner = a_btree.into_iter().collect();
                        }
                    }
                }
            },
        }
    }

    was_merged
}

fn write_default_toml() {
    let toml_file_location: String = get_toml_config_location();
    let path: &Path = Path::new(&toml_file_location);
    let parent_dir: &Path = path.parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).expect("Failed to create config directory");
    }
    let contents = to_string(&build_default_toml()).unwrap();
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