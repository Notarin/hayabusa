use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::config::config::load_toml_config;

lazy_static!(
    pub(crate) static ref TOML_CONFIG_OBJECT: TomlConfig = load_toml_config();
);

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct TomlConfig {
    pub(crate) spacing: Spacing,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Spacing {
    pub(crate) middle_margin: u8,
}

pub(crate) const DEFAULT_TOML_CONFIG: TomlConfig = TomlConfig {
    spacing: Spacing {
        middle_margin: 4,
    },
};