use crate::client::main::get_ascii_art;
use crate::config::toml::{
    Alignment, ArtPlacement, BorderChars, Padding, TomlConfig, TOML_CONFIG_OBJECT,
};
use crate::daemon::fetch_info::SystemInfo;
use lazy_static::lazy_static;
use regex::Regex;
use unicode_width::UnicodeWidthStr;

use super::kitty_backend::get_kitty_image;

pub(crate) fn main(system_info: &SystemInfo, mut fetch: String) -> String {
    // oh boy, there is a lot of string manipulation here, I'm sorry to anyone who has to read this
    let config: &TomlConfig = &TOML_CONFIG_OBJECT;
    let mut ascii_art: String;
    match config.ascii_art.backend.engine {
        crate::config::toml::Engine::Ascii => {
            ascii_art = get_ascii_art(&system_info.distro);
        }
        crate::config::toml::Engine::Kitty => {
            ascii_art = get_kitty_image().unwrap_or(get_ascii_art(&system_info.distro));
        }
        crate::config::toml::Engine::None => {
            ascii_art = String::new();
        }
    }
    terminate_styling(&mut ascii_art);
    terminate_styling(&mut fetch);
    parse_internal_ansi_codes(&mut ascii_art);
    parse_internal_ansi_codes(&mut fetch);
    // fuck it, normalize() spam
    normalize(&mut ascii_art);
    normalize(&mut fetch);
    // align ascii art and fetch
    align(vec![&mut ascii_art, &mut fetch]);
    // add ascii art
    let mut full_fetch: String = add_ascii_art(&ascii_art, fetch);
    normalize(&mut full_fetch);
    // add inner padding
    add_padding(&mut full_fetch, &config.spacing.inner_padding);
    // add border
    if config.border.enabled {
        full_fetch = add_border(&full_fetch, &config.border.border_chars);
    }
    // add outer padding
    add_padding(&mut full_fetch, &config.spacing.outer_padding);
    reset_formatting_on_cr(&full_fetch)
}

fn terminate_styling(string: &mut String) {
    let mut lines: Vec<String> = string.lines().map(|s| s.to_string()).collect();
    for line in lines.iter_mut() {
        line.push_str("{{reset}}");
    }
    *string = lines.join("\n");
}

fn add_border(string: &str, border_chars: &BorderChars) -> String {
    let lines: Vec<&str> = string.lines().collect();
    let ansi_free_lines: Vec<String> = lines.iter().map(|s| remove_ansi_escape_codes(s)).collect();
    let max_len: usize = ansi_free_lines
        .iter()
        .map(|s| UnicodeWidthStr::width(s.as_str()))
        .max()
        .unwrap_or(0);
    let config: &TomlConfig = &TOML_CONFIG_OBJECT;
    let mut ansi_color: String = config.border.ansi_color.clone();
    parse_internal_ansi_codes(&mut ansi_color);
    let mut color_reset: String = "{{reset}}".to_string();
    parse_internal_ansi_codes(&mut color_reset);

    let top_horizontal_border = format!(
        "{}{}{}{}{}",
        &ansi_color,
        &border_chars.top_left,
        &border_chars.horizontal.to_string().repeat(max_len),
        &border_chars.top_right,
        &color_reset,
    );

    let bottom_horizontal_border = format!(
        "{}{}{}{}{}",
        &ansi_color,
        &border_chars.bottom_left,
        &border_chars.horizontal.to_string().repeat(max_len),
        &border_chars.bottom_right,
        &color_reset,
    );

    let estimated_capacity = (max_len + ansi_color.len() + color_reset.len() + 10) * lines.len()
        + top_horizontal_border.len()
        + bottom_horizontal_border.len();
    let mut bordered_string = String::with_capacity(estimated_capacity);

    bordered_string.push_str(&top_horizontal_border);
    bordered_string.push('\n');

    for i in 0..lines.len() {
        let line = &lines[i];
        let ansi_free_line = &ansi_free_lines[i];

        let padding_len = max_len - UnicodeWidthStr::width(ansi_free_line.as_str());

        bordered_string.push_str(&ansi_color);
        bordered_string.push(border_chars.vertical);
        bordered_string.push_str(&color_reset);
        bordered_string.push_str(line);
        for _ in 0..padding_len {
            bordered_string.push(' ');
        }
        bordered_string.push_str(&ansi_color);
        bordered_string.push(border_chars.vertical);
        bordered_string.push_str(&color_reset);
        bordered_string.push('\n');
    }

    // Corrected this section to only add the bottom border
    bordered_string.push_str(&bottom_horizontal_border);

    bordered_string
}

fn add_padding(string: &mut String, padding: &Padding) {
    add_upper_padding(string, padding.top);
    add_lower_padding(string, padding.bottom);
    add_left_padding(string, padding.left);
    add_right_padding(string, padding.right);
}

fn calculate_padding(string: &str, padding: u8) -> String {
    if padding == 0 {
        return String::new();
    }
    let max_width = remove_ansi_escape_codes(string)
        .lines()
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    vec![" ".repeat(max_width); padding as usize].join("\n")
}

fn add_upper_padding(string: &mut String, padding: u8) {
    let upper_padding: String = calculate_padding(string, padding);
    if !upper_padding.is_empty() {
        *string = format!("{}\n{}", upper_padding, string);
    }
}

fn add_lower_padding(string: &mut String, padding: u8) {
    let lower_padding: String = calculate_padding(string, padding);
    if !lower_padding.is_empty() {
        *string = format!("{}\n{}", string, lower_padding);
    }
}

fn add_left_padding(string: &mut String, padding: u8) {
    if padding > 0 {
        *string = string
            .lines()
            .map(|line| format!("{}{}", " ".repeat(padding as usize), line))
            .collect::<Vec<String>>()
            .join("\n");
    };
}

fn add_right_padding(string: &mut String, padding: u8) {
    if padding > 0 {
        *string = string
            .lines()
            .map(|line| format!("{}{}", line, " ".repeat(padding as usize)))
            .collect::<Vec<String>>()
            .join("\n");
    };
}

fn reset_formatting_on_cr(string: &str) -> String {
    string.replace('\r', "\x1b[0m\r")
}

fn add_ascii_art(ascii_art: &str, fetch: String) -> String {
    let toml_config: &TomlConfig = &TOML_CONFIG_OBJECT;
    let placement: &ArtPlacement = &toml_config.ascii_art.placement;
    let padding: u8 = toml_config.spacing.middle_padding;
    let binding: String = " ".repeat(padding as usize);

    let result: String = match placement {
        ArtPlacement::Left => {
            let blocks: Vec<&str> = vec![ascii_art, binding.as_str(), &fetch];
            place_blocks_adjacent(blocks)
        }
        ArtPlacement::Right => {
            let blocks: Vec<&str> = vec![&fetch, binding.as_str(), ascii_art];
            place_blocks_adjacent(blocks)
        }
        ArtPlacement::Top => {
            let transposed_binding: String = transpose_string(binding);
            let blocks: Vec<&str> = vec![ascii_art, transposed_binding.as_str(), &fetch];
            blocks.join("\n")
        }
        ArtPlacement::Bottom => {
            let transposed_binding: String = transpose_string(binding);
            let blocks: Vec<&str> = vec![&fetch, transposed_binding.as_str(), ascii_art];
            blocks.join("\n")
        }
    };
    result
}

fn align(mut blocks: Vec<&mut String>) {
    let toml_config: &TomlConfig = &TOML_CONFIG_OBJECT;

    for block in blocks.iter_mut() {
        normalize(block);
    }

    let max_width: usize = blocks
        .iter()
        .map(|block| {
            remove_ansi_escape_codes(block)
                .lines()
                .map(UnicodeWidthStr::width)
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0);
    let max_height: usize = blocks
        .iter()
        .map(|block| block.lines().count())
        .max()
        .unwrap_or(0);
    let alignment: &Alignment = &toml_config.ascii_art.alignment;

    match toml_config.ascii_art.placement {
        ArtPlacement::Top | ArtPlacement::Bottom => {
            for block in blocks {
                let block_width = remove_ansi_escape_codes(block)
                    .lines()
                    .map(UnicodeWidthStr::width)
                    .max()
                    .unwrap_or(0);

                let difference: usize = max_width - block_width;

                match alignment {
                    Alignment::Left => {
                        *block = format!("{}{}", block, " ".repeat(difference));
                        normalize(&mut *block);
                    }
                    Alignment::Right => {
                        let lines: Vec<String> = block.lines().map(String::from).collect();
                        let padded_lines: Vec<String> = lines
                            .iter()
                            .map(|line| format!("{}{}", " ".repeat(difference), line))
                            .collect();
                        *block = padded_lines.join("\n");
                    }
                    _ => {
                        let left_difference: usize = difference / 2;
                        let right_difference: usize = difference - left_difference;
                        let lines: Vec<String> = block.lines().map(String::from).collect();
                        let padded_lines: Vec<String> = lines
                            .iter()
                            .map(|line| {
                                format!(
                                    "{}{}{}",
                                    " ".repeat(left_difference),
                                    line,
                                    " ".repeat(right_difference)
                                )
                            })
                            .collect();
                        *block = padded_lines.join("\n");
                    }
                }
            }
        }
        ArtPlacement::Left | ArtPlacement::Right => {
            for block in blocks {
                let mut lines: Vec<String> = block.lines().map(String::from).collect();

                match alignment {
                    Alignment::Top => {
                        for line in lines.iter_mut() {
                            let current_width: usize = UnicodeWidthStr::width(line.as_str());
                            if current_width < max_width {
                                line.push(' ');
                            }
                        }
                    }
                    Alignment::Bottom => {
                        let num_lines: usize = lines.len();
                        if num_lines < max_height {
                            let empty_lines_to_add: usize = max_height - num_lines;

                            let empty_lines: Vec<String> =
                                vec![" ".repeat(max_width); empty_lines_to_add]
                                    .into_iter()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>();

                            lines.splice(0..0, empty_lines.into_iter());
                        }
                    }
                    _ => {
                        let num_lines = lines.len();
                        if num_lines < max_height {
                            let total_spaces_to_add = max_height - num_lines;
                            let spaces_above = total_spaces_to_add / 2;
                            let spaces_below = total_spaces_to_add - spaces_above;

                            for _ in 0..spaces_above {
                                lines.insert(0, String::from(" "));
                            }
                            for _ in 0..spaces_below {
                                lines.push(String::from(" "));
                            }
                        }
                    }
                }

                *block = lines.join("\n");
                normalize(&mut *block);
            }
        }
    }
}

fn normalize(block: &mut String) {
    let block_lines: Vec<String> = block.lines().map(|s| s.to_string()).collect();
    let ansi_free_block: String = remove_ansi_escape_codes(block);
    let ansi_free_block_lines: Vec<String> =
        ansi_free_block.lines().map(|s| s.to_string()).collect();

    let target_width: usize = ansi_free_block_lines
        .iter()
        .map(|line| UnicodeWidthStr::width(line.as_str()))
        .max()
        .unwrap_or(0);

    let mut normalized_lines: Vec<String> = Vec::new();

    for (line, ansi_free_line) in block_lines.iter().zip(ansi_free_block_lines.iter()) {
        let current_width: usize = UnicodeWidthStr::width(ansi_free_line.as_str());
        let mut new_line = line.clone();
        if current_width < target_width {
            let difference: usize = target_width - current_width;
            new_line.push_str(&" ".repeat(difference));
        }
        normalized_lines.push(new_line);
    }

    *block = normalized_lines.join("\n");
}

fn transpose_string(string: String) -> String {
    let mut transposed = String::new();
    for (i, c) in string.chars().enumerate() {
        if i > 0 {
            transposed.push('\n');
        }
        transposed.push(c);
    }
    transposed
}

fn place_blocks_adjacent(blocks: Vec<&str>) -> String {
    let normalized_blocks: Vec<String> = vertically_normalize(blocks);
    let mut lines: Vec<Vec<String>> = Vec::new();

    for block in normalized_blocks {
        let block_lines: Vec<String> = block
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if lines.is_empty() {
            lines = vec![vec![]; block_lines.len()];
        }
        for (i, line) in block_lines.iter().enumerate() {
            lines[i].push(line.to_string());
        }
    }

    let mut result: String = String::new();
    for line_group in lines {
        let joined_line: String = line_group.join("");
        result.push_str(&joined_line);
        result.push('\n');
    }

    result
}

fn vertically_normalize(blocks: Vec<&str>) -> Vec<String> {
    let max_height: usize = blocks
        .iter()
        .map(|block| block.lines().count())
        .max()
        .unwrap_or(0);

    let mut normalized_blocks: Vec<String> = Vec::new();

    for block in blocks {
        let mut lines: Vec<String> = block.lines().map(|s| s.to_string()).collect();
        let width: usize = if let Some(first_line) = lines.first() {
            UnicodeWidthStr::width(remove_ansi_escape_codes(&first_line.clone()).as_str())
        } else {
            0
        };

        while lines.len() < max_height {
            lines.push(" ".repeat(width));
        }

        normalized_blocks.push(lines.join("\n"));
    }

    normalized_blocks
}

lazy_static! {
    static ref ANSI_ESCAPE_CODE_REGEX: Regex =
        Regex::new(r"\x1B(?:\[[0-?]*[- /]*[@-~]|_[^\\]*;[^\\]*\\)").unwrap();
};

fn remove_ansi_escape_codes(s: &str) -> String {
    let re: &Regex = &ANSI_ESCAPE_CODE_REGEX;
    re.replace_all(s, "").to_string()
}

pub(crate) fn parse_internal_ansi_codes(ascii_art: &mut String) {
    while let Some(start_index) = ascii_art.find("{{color") {
        if let Some(end_index) = ascii_art[start_index..].find("}}") {
            // Adjust for the relative offset
            let relative_end_index: usize = start_index + end_index + 2;
            let tag_content: &str = &ascii_art[start_index + 7..relative_end_index - 2];

            if let Ok(value) = tag_content.parse::<u8>() {
                let ansi_code: String = format!("\x1b[38;5;{}m", value);
                *ascii_art =
                    ascii_art.replace(&ascii_art[start_index..relative_end_index], &ansi_code);
            } else {
                // Failed to parse the number, move to the next tag
                continue;
            }
        } else {
            // No closing "}}" found after "{{color", so break
            break;
        }
    }
    *ascii_art = ascii_art.replace("{{reset}}", "\x1b[0m");
}
