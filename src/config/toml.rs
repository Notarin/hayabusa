use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::config::config::load_toml_config;

lazy_static!(
    pub(crate) static ref TOML_CONFIG_OBJECT: TomlConfig = load_toml_config();
);

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct TomlConfig {
    pub(crate) spacing: Spacing,
    pub(crate) border: Border,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Spacing {
    pub(crate) middle_margin: u8,
    pub(crate) inner_padding: Padding,
    pub(crate) outer_padding: Padding,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Padding {
    pub(crate) top: u8,
    pub(crate) bottom: u8,
    pub(crate) left: u8,
    pub(crate) right: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Border {
    pub(crate) enabled: bool,
    pub(crate) top_left: char,
    pub(crate) top_right: char,
    pub(crate) bottom_left: char,
    pub(crate) bottom_right: char,
    pub(crate) horizontal: char,
    pub(crate) vertical: char,
}

pub(crate) const DEFAULT_TOML_CONFIG: TomlConfig = TomlConfig {
    spacing: Spacing {
        middle_margin: 4,
        inner_padding: Padding {
            top: 2,
            bottom: 2,
            left: 2,
            right: 2,
        },
        outer_padding: Padding {
            top: 2,
            bottom: 2,
            left: 2,
            right: 2,
        },
    },
    border: Border {
        enabled: false,
        top_left: '╭',
        top_right: '╮',
        bottom_left: '╰',
        bottom_right: '╯',
        horizontal: '─',
        vertical: '│',
    },
};