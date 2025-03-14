use image::{Rgba, RgbaImage};

/// Encodes bytes into an image using LSB steganography.
///
/// # Arguments
/// * `image` - Mutable reference to the image.
/// * `data` - The bytes to encode.
/// * `lsb_mode` - Number of LSBs to modify (1 or 2).
///
/// # Returns
/// Result<(), String> indicating success or error.
pub fn encode_image(image: &mut RgbaImage, data: &[u8], lsb_mode: u8) -> Result<(), String> {
    if lsb_mode != 1 && lsb_mode != 2 {
        return Err("Invalid LSB mode. Must be 1 or 2.".to_string());
    }

    let has_alpha = image.pixels().next().map_or(false, |p| p.0.len() == 4);

    if has_alpha {
        encode_rgba(image, data, lsb_mode)
    } else {
        encode_rgb(image, data, lsb_mode)
    }
}

/// Encodes data into an RGB image (ignores alpha channel if present).
fn encode_rgb(image: &mut RgbaImage, data: &[u8], lsb_mode: u8) -> Result<(), String> {
    let mut bit_index = 0;
    let mut byte_index = 0;

    for pixel in image.pixels_mut() {
        for channel in pixel.0.iter_mut().take(3) { // Modify only RGB
            if byte_index >= data.len() {
                return Ok(()); // Done encoding
            }

            let bit_mask = (1 << lsb_mode) - 1;
            let data_bits = (data[byte_index] >> (8 - lsb_mode - bit_index)) & bit_mask;

            *channel = (*channel & !bit_mask) | data_bits;

            bit_index += lsb_mode;
            if bit_index >= 8 {
                bit_index = 0;
                byte_index += 1;
            }
        }
    }

    Err("Not enough pixels to encode all data.".to_string())
}

/// Encodes data into an RGBA image (modifies all four channels, including alpha).
fn encode_rgba(image: &mut RgbaImage, data: &[u8], lsb_mode: u8) -> Result<(), String> {
    let mut bit_index = 0;
    let mut byte_index = 0;

    for pixel in image.pixels_mut() {
        for channel in pixel.0.iter_mut() { // Modify all RGBA channels
            if byte_index >= data.len() {
                return Ok(());
            }

            let bit_mask = (1 | lsb_mode);
            let data_bits = (data[byte_index] >> (8 - lsb_mode - bit_index)) & bit_mask;

            *channel = (*channel & !bit_mask) | data_bits;

            bit_index += lsb_mode;
            if bit_index >= 8 {
                bit_index = 0;
                byte_index += 1;
            }
        }
    }

    Err("Not enough pixels to encode all data.".to_string())
}