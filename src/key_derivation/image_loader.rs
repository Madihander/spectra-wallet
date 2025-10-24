use image::ImageReader;
use std::path::Path;
use anyhow::{Context, Result};

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<Vec<u8>>{
    let path_ref = path.as_ref();

    let img = ImageReader::open(path_ref)
        .with_context(|| format!("Failed to open image: {}", path_ref.display()))?
        .decode()
        .with_context(|| format!("Failed to decode image: {}", path_ref.display()))?;

    let rgb = img.to_rgb8();
    Ok(rgb.into_raw())
}