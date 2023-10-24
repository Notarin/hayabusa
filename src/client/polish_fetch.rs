use lazy_static::lazy_static;
use regex::Regex;
use unicode_width::UnicodeWidthStr;
use crate::client::main::get_ascii_art;
use crate::config::toml::{BorderChars, Padding, TOML_CONFIG_OBJECT, TomlConfig};
use crate::daemon::fetch_info::SystemInfo;

pub(crate) fn main(system_info: &SystemInfo, fetch: String) -> String {
    let config: &TomlConfig = &TOML_CONFIG_OBJECT;
    let mut ascii_art: String = get_ascii_art(&system_info.distro);
    ascii_art = parse_ascii_art(&ascii_art);
    ascii_art = normalize(&ascii_art);
    let mut full_fetch: String = add_ascii_art(&ascii_art, fetch);
    full_fetch = normalize(&full_fetch);
    // add inner padding
    full_fetch = add_padding(&full_fetch, &config.spacing.inner_padding);
    // add border
    if config.border.enabled {
        full_fetch = add_border(&full_fetch, &config.border.border_chars);
    }
    // add outer padding
    full_fetch = add_padding(&full_fetch, &config.spacing.outer_padding);
    reset_formatting_on_cr(&full_fetch)
}

fn add_border(string: &str, border_chars: &BorderChars) -> String {
    let lines: Vec<&str> = string.lines().collect();
    let ansi_free_lines: Vec<String> = lines.iter().map(|s| remove_ansi_escape_codes((*s))).collect();
    let max_len: usize = ansi_free_lines.iter().map(|s| UnicodeWidthStr::width(s.as_str())).max().unwrap_or(0);
    let config: &TomlConfig = &TOML_CONFIG_OBJECT;
    let ansi_color: String = parse_ascii_art(&(config.border.ansi_color.clone()));
    let color_reset: String = parse_ascii_art(&"{{reset}}".to_string());

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

    let estimated_capacity = (max_len + ansi_color.len() + color_reset.len() + 10) * lines.len() + top_horizontal_border.len() + bottom_horizontal_border.len();
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

fn add_padding(string: &str, padding: &Padding) -> String {
    let mut result: String = string.to_string();
    result = add_upper_padding(&result, padding.top);
    result = add_lower_padding(&result, padding.bottom);
    result = add_left_padding(&result, padding.left);
    result = add_right_padding(&result, padding.right);
    result
}

fn calculate_padding(string: &str, padding: u8) -> String {
    if padding == 0 {
        return String::new();
    }
    let max_width = remove_ansi_escape_codes(string).lines()
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    vec![" ".repeat(max_width); padding as usize].join("\n")
}

fn add_upper_padding(string: &str, padding: u8) -> String {
    let upper_padding = calculate_padding(string, padding);
    format!("{}\n{}", upper_padding, string)
}

fn add_lower_padding(string: &str, padding: u8) -> String {
    let lower_padding = calculate_padding(string, padding);
    format!("{}\n{}", string, lower_padding)
}
fn add_left_padding(string: &str, padding: u8) -> String {
    if padding > 0 {
        string.lines()
            .map(|line| format!("{}{}", " ".repeat(padding as usize), line))
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        string.to_string()
    }
}

fn add_right_padding(string: &str, padding: u8) -> String {
    if padding > 0 {
        string.lines()
            .map(|line| format!("{}{}", line, " ".repeat(padding as usize)))
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        string.to_string()
    }
}

fn reset_formatting_on_cr(string: &str) -> String {
    string.replace('\r', "\x1b[0m\r")
}

fn add_ascii_art(ascii_art: &str, fetch: String) -> String {
    let toml_config: &TomlConfig = &TOML_CONFIG_OBJECT;
    let padding: u8 = toml_config.spacing.middle_padding;
    let binding = " ".repeat(padding as usize);
    let blocks: Vec<&str> = vec![
        ascii_art,
        binding.as_str(),
        &fetch
    ];
    place_blocks_adjacent(blocks)
}

fn normalize(block: &str) -> String {
    let block_lines: Vec<String> = block.lines().map(|s| s.to_string()).collect();
    let ansi_free_block: String = remove_ansi_escape_codes(block);
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

fn place_blocks_adjacent(blocks: Vec<&str>) -> String {
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

fn vertically_normalize(blocks: Vec<&str>) -> Vec<String> {
    let max_height: usize = blocks.iter()
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

lazy_static!(
    static ref ANSI_ESCAPE_CODE_REGEX: Regex = Regex::new(r"\x1B\[[0-?]*[- /]*[@-~]").unwrap();
);

fn remove_ansi_escape_codes(s: &str) -> String {
    let re: &Regex = &ANSI_ESCAPE_CODE_REGEX;
    re.replace_all(s, "").to_string()
}

pub(crate) fn parse_ascii_art(ascii_art: &String) -> String {
    let mut result: String = ascii_art.to_string();
    let color0: &str = "\x1b[38;5;0m";
    let color1: &str = "\x1b[38;5;1m";
    let color2: &str = "\x1b[38;5;2m";
    let color3: &str = "\x1b[38;5;3m";
    let color4: &str = "\x1b[38;5;4m";
    let color5: &str = "\x1b[38;5;5m";
    let color6: &str = "\x1b[38;5;6m";
    let color7: &str = "\x1b[38;5;7m";
    let color8: &str = "\x1b[38;5;8m";
    let color9: &str = "\x1b[38;5;9m";
    let color10: &str = "\x1b[38;5;10m";
    let color11: &str = "\x1b[38;5;11m";
    let color12: &str = "\x1b[38;5;12m";
    let color13: &str = "\x1b[38;5;13m";
    let color14: &str = "\x1b[38;5;14m";
    let color15: &str = "\x1b[38;5;15m";
    let reset: &str = "\x1b[0m";
    result = result.replace("{{color0}}", color0);
    result = result.replace("{{color1}}", color1);
    result = result.replace("{{color2}}", color2);
    result = result.replace("{{color3}}", color3);
    result = result.replace("{{color4}}", color4);
    result = result.replace("{{color5}}", color5);
    result = result.replace("{{color6}}", color6);
    result = result.replace("{{color7}}", color7);
    result = result.replace("{{color8}}", color8);
    result = result.replace("{{color9}}", color9);
    result = result.replace("{{color10}}", color10);
    result = result.replace("{{color11}}", color11);
    result = result.replace("{{color12}}", color12);
    result = result.replace("{{color13}}", color13);
    result = result.replace("{{color14}}", color14);
    result = result.replace("{{color15}}", color15);
    result = result.replace("{{reset}}", reset);
    result
}
