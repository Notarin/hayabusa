use regex::Regex;
use unicode_width::UnicodeWidthStr;
use crate::client::main::get_ascii_art;
use crate::config::toml::{BorderChars, Padding, TOML_CONFIG_OBJECT, TomlConfig};
use crate::daemon::fetch_info::SystemInfo;

pub(crate) fn main(system_info: SystemInfo, mut fetch: String) -> String {
    let config: TomlConfig = TOML_CONFIG_OBJECT.clone();
    let mut ascii_art: String = get_ascii_art(system_info.distro.clone());
    ascii_art = parse_ascii_art(ascii_art);
    ascii_art = normalize(ascii_art);
    fetch = normalize(fetch);
    let mut full_fetch: String = add_ascii_art(ascii_art, fetch);
    full_fetch = normalize(full_fetch);
    // add inner padding
    full_fetch = add_padding(full_fetch, config.spacing.inner_padding);
    // add border
    if config.border.enabled {
        full_fetch = add_border(full_fetch, &config.border.border_chars);
    }
    // add outer padding
    full_fetch = add_padding(full_fetch, config.spacing.outer_padding);
    reset_formatting_on_cr(full_fetch)
}

fn add_border(string: String, border_chars: &BorderChars) -> String {
    let lines: Vec<&str> = string.lines().collect();
    let ansi_free_lines: Vec<String> = lines.iter().map(|s| remove_ansi_escape_codes((*s).to_string())).collect();
    for ansi_free_line in &ansi_free_lines {
    }
    let max_len: usize = ansi_free_lines.iter().map(|s| UnicodeWidthStr::width(s.as_str())).max().unwrap_or(0);

    let horizontal_border = format!(
        "{}{}{}",
        border_chars.top_left,
        border_chars.horizontal.to_string().repeat(max_len),
        border_chars.top_right
    );

    let mut bordered_string = horizontal_border.clone() + "\n";

    for (line, ansi_free_line) in lines.iter().zip(ansi_free_lines.iter()) {
        let padding = " ".repeat(max_len - UnicodeWidthStr::width(ansi_free_line.as_str()));
        bordered_string += &format!(
            "{}{}{}{}\n",
            border_chars.vertical,
            line,
            padding,
            border_chars.vertical
        );
    }

    bordered_string += &format!(
        "{}{}{}",
        border_chars.bottom_left,
        border_chars.horizontal.to_string().repeat(max_len),
        border_chars.bottom_right
    );

    bordered_string
}

fn add_padding(string: String, padding: Padding) -> String {
    let mut result: String = string;
    result = add_upper_padding(result, padding.top);
    result = add_lower_padding(result, padding.bottom);
    result = add_left_padding(result, padding.left);
    result = add_right_padding(result, padding.right);
    result
}

fn add_upper_padding(string: String, padding: u8) -> String {
    let max_width: usize = remove_ansi_escape_codes(string.clone()).lines()
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let mut result: String = String::new();
    for _ in 0..padding {
        result.push_str(&" ".repeat(max_width));
        result.push('\n');
    }
    result.push_str(&string);
    result
}

fn add_lower_padding(string: String, padding: u8) -> String {
    let max_width: usize = remove_ansi_escape_codes(string.clone()).lines()
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let mut result: String = string;
    for _ in 0..padding {
        result.push('\n');
        result.push_str(&" ".repeat(max_width));
    }
    result
}

fn add_left_padding(string: String, padding: u8) -> String {
    let mut result: String = String::new();
    for line in string.lines() {
        result.push_str(&" ".repeat(padding as usize));
        result.push_str(line);
        result.push('\n');
    }
    result
}

fn add_right_padding(string: String, padding: u8) -> String {
    let mut result: String = String::new();
    for line in string.lines() {
        result.push_str(line);
        result.push_str(&" ".repeat(padding as usize));
        result.push('\n');
    }
    result
}

fn reset_formatting_on_cr(string: String) -> String {
    string.replace('\r', "\x1b[0m\r")
}

fn add_ascii_art(ascii_art: String, fetch: String) -> String {
    let toml_config: TomlConfig = TOML_CONFIG_OBJECT.clone();
    let padding: u8 = toml_config.spacing.middle_padding;
    let blocks: Vec<String> = vec![
        ascii_art,
        " ".repeat(padding as usize),
        fetch
    ];
    place_blocks_adjacent(blocks)
}

fn normalize(block: String) -> String {
    let block_lines: Vec<String> = block.lines().map(|s| s.to_string()).collect();
    let ansi_free_block: String = remove_ansi_escape_codes(block.clone());
    let ansi_free_block_lines: Vec<String> = ansi_free_block.lines().map(|s| s.to_string()).collect();

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

    normalized_lines.join("\n")
}

fn place_blocks_adjacent(blocks: Vec<String>) -> String {
    let normalized_blocks: Vec<String> = vertically_normalize(blocks);
    let mut lines: Vec<Vec<String>> = Vec::new();

    for block in normalized_blocks {
        let block_lines: Vec<String> = block.lines()
            .map(|s| s.to_string()).collect::<Vec<String>>();
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

fn vertically_normalize(blocks: Vec<String>) -> Vec<String> {
    let max_height: usize = blocks.iter()
        .map(|block| block.lines().count())
        .max()
        .unwrap_or(0);

    let mut normalized_blocks: Vec<String> = Vec::new();

    for block in blocks {
        let mut lines: Vec<String> = block.lines().map(|s| s.to_string()).collect();
        let width: usize = if let Some(first_line) = lines.first() {
            UnicodeWidthStr::width(remove_ansi_escape_codes(first_line.clone()).as_str())
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

fn remove_ansi_escape_codes(s: String) -> String {
    let re: Regex = Regex::new(r"\x1B\[[0-?]*[- /]*[@-~]").unwrap();
    re.replace_all(&s, "").to_string()
}

pub(crate) fn parse_ascii_art(ascii_art: String) -> String {
    let mut result: String = ascii_art;
    let color0: String = "\x1b[38;5;0m".to_string();
    let color1: String = "\x1b[38;5;1m".to_string();
    let color2: String = "\x1b[38;5;2m".to_string();
    let color3: String = "\x1b[38;5;3m".to_string();
    let color4: String = "\x1b[38;5;4m".to_string();
    let color5: String = "\x1b[38;5;5m".to_string();
    let color6: String = "\x1b[38;5;6m".to_string();
    let color7: String = "\x1b[38;5;7m".to_string();
    let color8: String = "\x1b[38;5;8m".to_string();
    let color9: String = "\x1b[38;5;9m".to_string();
    let color10: String = "\x1b[38;5;10m".to_string();
    let color11: String = "\x1b[38;5;11m".to_string();
    let color12: String = "\x1b[38;5;12m".to_string();
    let color13: String = "\x1b[38;5;13m".to_string();
    let color14: String = "\x1b[38;5;14m".to_string();
    let color15: String = "\x1b[38;5;15m".to_string();
    let reset: String = "\x1b[0m".to_string();
    result = result.replace("{{color0}}", &color0);
    result = result.replace("{{color1}}", &color1);
    result = result.replace("{{color2}}", &color2);
    result = result.replace("{{color3}}", &color3);
    result = result.replace("{{color4}}", &color4);
    result = result.replace("{{color5}}", &color5);
    result = result.replace("{{color6}}", &color6);
    result = result.replace("{{color7}}", &color7);
    result = result.replace("{{color8}}", &color8);
    result = result.replace("{{color9}}", &color9);
    result = result.replace("{{color10}}", &color10);
    result = result.replace("{{color11}}", &color11);
    result = result.replace("{{color12}}", &color12);
    result = result.replace("{{color13}}", &color13);
    result = result.replace("{{color14}}", &color14);
    result = result.replace("{{color15}}", &color15);
    result = result.replace("{{reset}}", &reset);
    result
}
