use std::fs;
use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;

use crate::config::toml::TOML_CONFIG_OBJECT;

pub(crate) fn get_kitty_image() -> Result<String, String> {
    let toml_config: &TOML_CONFIG_OBJECT = &TOML_CONFIG_OBJECT;
    let escape_character: char = '\x1B';
    let mut control_data: String = String::new();
    let payload: Vec<u8> = get_image()?;

    let (image_width, image_height): (u16, u16) = preload_image_resolution()?;
    let target_rows: u16 = get_target_rows(image_width, image_height)?;
    control_data.push_str("f=100,");
    control_data.push_str("a=T,");
    control_data.push_str("z=1,");
    let binding: String = toml_config.ascii_art.backend.image_width.to_string();
    let columns: &str = binding.as_str();
    control_data.push_str(&("c=".to_string() + columns + ","));
    control_data.push_str(&("r=".to_string() + &target_rows.to_string() + ","));
    control_data.push_str("C=1,");

    let base_64_payload: String = general_purpose::STANDARD.encode(payload);
    let chunked_payload: Vec<String> = chunk_data(base_64_payload, 4096);
    let mut chunked_image: Vec<String> = Vec::new();

    for (i, chunk) in chunked_payload.iter().enumerate() {
        // Check if it's the last chunk
        let m_value: &str = if i == chunked_payload.len() - 1 { "0" } else { "1" };
        let current_control_data: &str = if i != 0 {
            ""
        } else {
            &control_data
        };

        let image_string = format!(
            "{}_G{}m={};{}{}\\",
            escape_character, current_control_data, m_value, chunk, escape_character
        );
        chunked_image.push(image_string);
    }

    let result: String = chunked_image.join("") +
        build_space_matrix(toml_config.ascii_art.backend.image_width, target_rows).as_str();
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
    let mut image: Vec<u8> = fs::read(&config.ascii_art.backend.image_path).expect("Failed to read image file");
    image = scale_and_center_image(image)?;
    Ok(image)
}

fn scale_and_center_image(image: Vec<u8>) -> Result<Vec<u8>, String> {
    let image: DynamicImage = image::load_from_memory(&image).expect("Failed to load image");
    //get image size requirements
    //start by getting cell size
    let terminal_size: TerminalSize = get_cell_size()?;
    let target_cell_width: &u16 = &TOML_CONFIG_OBJECT.ascii_art.backend.image_width;
    //next figure the current image resolution
    let image_width: u32 = image.width();
    let image_height: u32 = image.height();
    let target_width: u32 = (*target_cell_width * terminal_size.cell_width) as u32;
    let target_height: u32 = (target_width as f32 / image_width as f32 * image_height as f32) as u32;
    //scale image
    let scaled_image: DynamicImage =
        image.resize_exact(
            target_width,
            target_height,
            image::imageops::FilterType::Nearest
        );
    //divide the target height by the cell height to get the number of rows
    //then round up to the nearest whole number
    //then multiply by the cell height
    //thats our canvas height
    //make our canvas, then center the image on the canvas
    let canvas_height: u32 = (target_height as f32 / terminal_size.cell_height as f32).ceil() as u32 * terminal_size.cell_height as u32;
    let canvas_width: u32 = target_width;
    let mut canvas: DynamicImage = DynamicImage::new_rgba8(canvas_width, canvas_height);
    let canvas_x: u32 = (canvas_width - target_width) / 2;
    let canvas_y: u32 = (canvas_height - target_height) / 2;
    image::imageops::overlay(&mut canvas, &scaled_image, canvas_x as i64, canvas_y as i64);
    //convert the canvas to a vec of u8
    let mut canvas_bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    canvas.write_to(&mut canvas_bytes, image::ImageOutputFormat::Png).expect("Failed to write canvas to vec");

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
    use nix::{ioctl_read, libc};
    use nix::libc::ioctl;

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
        let error_message: String = "Failed to retrieve terminal info, ensure you're using a compliant terminal."
            .to_string();
        return Err(error_message);
    }

    Ok(terminal_size)
}

#[cfg(target_os = "windows")]
fn get_cell_size() -> Result<TerminalSize, String> {
    Err("Windows does not support the proper protocols".to_string())
}