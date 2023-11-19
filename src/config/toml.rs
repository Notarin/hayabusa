use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::config::main::load_toml_config;

lazy_static!(
    pub(crate) static ref TOML_CONFIG_OBJECT: TomlConfig = load_toml_config();
);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct TomlConfig {
    pub(crate) spacing: Spacing,
    pub(crate) border: Border,
    pub(crate) ascii_art: AsciiArt,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Spacing {
    pub(crate) middle_padding: u8,
    pub(crate) inner_padding: Padding,
    pub(crate) outer_padding: Padding,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Padding {
    pub(crate) top: u8,
    pub(crate) bottom: u8,
    pub(crate) left: u8,
    pub(crate) right: u8,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Border {
    pub(crate) enabled: bool,
    pub(crate) ansi_color: String,
    pub(crate) border_chars: BorderChars,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct BorderChars {
    pub(crate) top_left: char,
    pub(crate) top_right: char,
    pub(crate) bottom_left: char,
    pub(crate) bottom_right: char,
    pub(crate) horizontal: char,
    pub(crate) vertical: char,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct AsciiArt {
    pub(crate) size: AsciiSize,
    pub(crate) placement: ArtPlacement,
    pub(crate) alignment: Alignment,
    pub(crate) backend: AsciiArtBackend,
    pub(crate) ascii_art_file: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct AsciiArtBackend {
    pub(crate) engine: Engine,
    pub(crate) image_path: String,
    pub(crate) image_width: u16,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) enum Engine {
    Ascii,
    Kitty,
    None,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) enum Alignment {
    Left,
    Center,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) enum ArtPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) enum AsciiSize {
    Small,
    Big,
}

pub(crate) fn build_default_toml() -> TomlConfig {
    TomlConfig {
        spacing: Spacing {
            middle_padding: 4,
            inner_padding: Padding {
                top: 1,
                bottom: 1,
                left: 2,
                right: 2,
            },
            outer_padding: Padding {
                top: 0,
                bottom: 0,
                left: 0,
                right: 0,
            },
        },
        border: Border {
            enabled: true,
            ansi_color: String::from("{{color11}}"),
            border_chars: BorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
            },
        },
        ascii_art: AsciiArt {
            size: AsciiSize::Big,
            placement: ArtPlacement::Left,
            alignment: Alignment::Center,
            backend: AsciiArtBackend {
                engine: Engine::Ascii,
                image_path: String::from(""),
                image_width: 50,
            },
            ascii_art_file: String::from(""),
        },
    }
}