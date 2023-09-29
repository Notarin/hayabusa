use serde::{Deserialize, Serialize};

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