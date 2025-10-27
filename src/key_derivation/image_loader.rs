use image::ImageReader;
use std::path::Path;
use anyhow::{anyhow, Context, Result};

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<Vec<u8>>{
    let path_ref = path.as_ref();

    let img = ImageReader::open(path_ref)
        .with_context(|| format!("Failed to open image: {}", path_ref.display()))?
        .decode()
        .with_context(|| format!("Failed to decode image: {}", path_ref.display()))?;

    let rgb = img.to_rgb8();
    Ok(rgb.into_raw())
}

pub fn process_image(image_data: &[u8]) -> Result<Vec<u8>> {
    // Load the image from memory
    let img = image::load_from_memory(image_data)
        .map_err(|e|anyhow!("Failed to load image: {}", e))?;

    // Convert the image to RGB8 format
    let rgb = img.to_rgb8();

    // Return the raw RGB data
    Ok(rgb.into_raw())
}