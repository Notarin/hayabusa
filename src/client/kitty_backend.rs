use std::fs;
use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;

use crate::config::toml::TOML_CONFIG_OBJECT;

pub(crate) fn get_kitty_image() -> Result<String, String> {
    let toml_config: &TOML_CONFIG_OBJECT = &TOML_CONFIG_OBJECT;
    let escape_character: char = '\x1B';
    let mut control_data: String = String::new();
    let image_payload: Vec<u8> = get_image()?;

    // I realized that I'd need the image resolution this early
    // and I was way too deep in to go back, so I'm just preloading it, bite me
    let (image_width, image_height): (u16, u16) = preload_image_resolution()?;
    let target_rows: u16 = get_target_rows(image_width, image_height)?;

    let will_be_png_data: &str = "f=100,";
    let actually_display_the_image_please: &str = "a=T,";
    let display_over_text: &str = "z=1,";
    let binding: String = toml_config.ascii_art.backend.image_width.to_string();
    let columns: &str = binding.as_str();
    let amount_of_columns_for_the_image_to_span: &String = &("c=".to_string() + columns + ",");
    let amount_of_rows_for_the_image_to_span: &String =
        &("r=".to_string() + &target_rows.to_string() + ",");
    let reset_cursor_position_after_printing: &str = "C=1,";
    control_data.push_str(will_be_png_data);
    control_data.push_str(actually_display_the_image_please);
    control_data.push_str(display_over_text);
    control_data.push_str(amount_of_columns_for_the_image_to_span);
    control_data.push_str(amount_of_rows_for_the_image_to_span);
    control_data.push_str(reset_cursor_position_after_printing);

    let base_64_payload: String = general_purpose::STANDARD.encode(image_payload);
    let chunked_payload: Vec<String> = chunk_data(base_64_payload, 4096);

    let mut chunked_image: Vec<String> = Vec::new();
    for (i, chunk) in chunked_payload.iter().enumerate() {
        // Check if it's the last chunk
        let m_value: &str = if i == chunked_payload.len() - 1 {
            "0"
        } else {
            "1"
        };
        let current_control_data: &str = if i != 0 { "" } else { &control_data };

        let image_string: String = format!(
            "{}_G{}m={};{}{}\\",
            escape_character, current_control_data, m_value, chunk, escape_character
        );
        chunked_image.push(image_string);
    }

    // In order to properly place the image, I'm literally just going to build a matrix of spaces
    // it works
    let result: String = chunked_image.join("")
        + build_space_matrix(toml_config.ascii_art.backend.image_width, target_rows).as_str();
    Ok(result)
}

fn build_space_matrix(width: u16, height: u16) -> String {
    let mut space_matrix: String = String::new();
    for i in 0..height {
        for _ in 0..width {
            space_matrix.push(' ');
        }
        if i < height - 1 {
            space_matrix.push('\n');
        }
    }
    space_matrix
}

fn chunk_data(data: String, chunk_size: usize) -> Vec<String> {
    data.as_bytes()
        .chunks(chunk_size)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect()
}

fn get_target_rows(width: u16, height: u16) -> Result<u16, String> {
    // Hope you payed attention in math class
    let config: &TOML_CONFIG_OBJECT = &TOML_CONFIG_OBJECT;
    let target_columns: u16 = config.ascii_art.backend.image_width;
    let terminal_size: TerminalSize = get_cell_size()?;
    let target_width: u16 = target_columns * terminal_size.cell_width;
    let aspect_correction: f32 = target_width as f32 / width as f32;
    let corrected_height: u16 = (height as f32 * aspect_correction) as u16;
    let target_rows: u16 = (corrected_height / terminal_size.cell_height) + 1;
    Ok(target_rows)
}

fn preload_image_resolution() -> Result<(u16, u16), String> {
    // Yes, I know, duplicate code, but I'm not going to refactor this right now
    let config: &TOML_CONFIG_OBJECT = &TOML_CONFIG_OBJECT;
    let mut path: String = config.ascii_art.backend.image_path.clone();
    if path.starts_with('~') {
        let home_dir: String = env!("HOME").to_string();
        path = path.replace('~', &home_dir);
    }
    let image: Vec<u8> = fs::read(path).expect("Failed to read image file");
    let image: DynamicImage = image::load_from_memory(&image).expect("Failed to load image");
    let image_width: u32 = image.width();
    let image_height: u32 = image.height();
    Ok((image_width as u16, image_height as u16))
}

fn get_image() -> Result<Vec<u8>, String> {
    let config: &TOML_CONFIG_OBJECT = &TOML_CONFIG_OBJECT;
    let mut path: String = config.ascii_art.backend.image_path.clone();
    if path.starts_with('~') {
        let home_dir: String = env!("HOME").to_string();
        path = path.replace('~', &home_dir);
    }
    let mut image: Vec<u8> = fs::read(path).expect("Failed to read image file");
    image = scale_and_center_image(image)?;
    Ok(image)
}

fn scale_and_center_image(image: Vec<u8>) -> Result<Vec<u8>, String> {
    let image: DynamicImage = image::load_from_memory(&image).expect("Failed to load image");

    let terminal_size: TerminalSize = get_cell_size()?;
    let target_cell_width: &u16 = &TOML_CONFIG_OBJECT.ascii_art.backend.image_width;

    let image_width: u32 = image.width();
    let image_height: u32 = image.height();
    let target_width: u32 = (*target_cell_width * terminal_size.cell_width) as u32;
    let target_height: u32 =
        (target_width as f32 / image_width as f32 * image_height as f32) as u32;

    let scaled_image: DynamicImage = image.resize_exact(
        target_width,
        target_height,
        image::imageops::FilterType::Nearest,
    );

    // Placing the image on a canvas so its centered, yes, its just a few pixels, but it looks better
    let canvas_height: u32 = (target_height as f32 / terminal_size.cell_height as f32).ceil()
        as u32
        * terminal_size.cell_height as u32;
    let canvas_width: u32 = target_width;
    let mut canvas: DynamicImage = DynamicImage::new_rgba8(canvas_width, canvas_height);
    let canvas_x: u32 = (canvas_width - target_width) / 2;
    let canvas_y: u32 = (canvas_height - target_height) / 2;
    image::imageops::overlay(&mut canvas, &scaled_image, canvas_x as i64, canvas_y as i64);

    let mut canvas_bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    canvas
        .write_to(&mut canvas_bytes, image::ImageOutputFormat::Png)
        .expect("Failed to write canvas to vec");

    Ok(canvas_bytes.into_inner())
}

#[derive(Debug)]
#[cfg(target_os = "linux")]
struct TerminalSize {
    width: u16,
    height: u16,
    cell_width: u16,
    cell_height: u16,
}

#[derive(Debug)]
#[cfg(target_os = "windows")]
struct TerminalSize {
    cell_width: u16,
    cell_height: u16,
}

#[cfg(target_os = "linux")]
fn get_cell_size() -> Result<TerminalSize, String> {
    use nix::libc::ioctl;
    use nix::{ioctl_read, libc};

    ioctl_read!(get_winsize, libc::TIOCGWINSZ, 0, libc::winsize);

    let mut terminal_size = TerminalSize {
        width: 0,
        height: 0,
        cell_width: 0,
        cell_height: 0,
    };

    let mut winsize = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    // unsafe code? literal bruh moment
    // If someone can figure out how to do this without unsafe code, please do
    // 1: Don't create dependencies
    // 2: Don't use STDIN/STDOUT garbage
    // 3: Don't break support for terminals that already work
    unsafe {
        if ioctl(0, libc::TIOCGWINSZ, &mut winsize) == -1 {
            return Err("Failed to get terminal size".to_string());
        }
    }

    terminal_size.width = winsize.ws_col;
    terminal_size.height = winsize.ws_row;
    terminal_size.cell_width = winsize.ws_xpixel / terminal_size.width;
    terminal_size.cell_height = winsize.ws_ypixel / terminal_size.height;

    if terminal_size.cell_width == 0 || terminal_size.cell_height == 0 {
        let error_message: String =
            "Failed to retrieve terminal info, ensure you're using a compliant terminal."
                .to_string();
        return Err(error_message);
    }

    Ok(terminal_size)
}

#[cfg(target_os = "windows")]
fn get_cell_size() -> Result<TerminalSize, String> {
    // Hey, heads up, don't fix this for windows
    // Proper compatibilty check is not yet made
    // and if you fix this it'll start spewing escape code garbage into the windows terminal.
    // Also, windows terminals do not support the proper protocols, so it's not even worth it.
    Err("Windows does not support the proper protocols".to_string())
}
