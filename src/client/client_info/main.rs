use std::collections::HashMap;
use rlua::{Context, Table};

pub(crate) fn get_environmental_variables() -> HashMap<String, String> {
    let mut env_vars: HashMap<String, String> = HashMap::new();
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
        env_vars.insert(key, value);
    }
    env_vars
}

pub(crate) fn environmental_variable_table(lua_ctx: Context) -> Table {
    let env_vars: HashMap<String, String> = get_environmental_variables();
    let env_vars_table: Table = lua_ctx.create_table().unwrap();
    for (key, value) in env_vars {
        env_vars_table.set(key, value).unwrap();
    }
    env_vars_table
}