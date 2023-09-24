use std::{env, fs};
use std::path::Path;

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

pub(crate) fn get_config_location() -> String {
    let config_dir: String = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| env::var("HOME").expect("Failed to get $HOME") + "/.config");
    let hayabusa_dir: String = config_dir + "/hayabusa/";
    let lua_file: String = hayabusa_dir + "config.lua";
    lua_file
}