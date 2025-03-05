use std::io::Cursor;

use anyhow::Result;
use image::{DynamicImage, ImageReader};

use crate::presets::ImageFormat;

#[tracing::instrument(skip(image_data))]
pub fn read_image(image_data: &[u8]) -> Result<DynamicImage> {
    Ok(ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()?
        .decode()?)
}

#[tracing::instrument(skip(image))]
pub fn get_image_bytes(image: DynamicImage, format: ImageFormat) -> Result<Vec<u8>> {
    let mut out_data: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut out_data), format.into())?;
    Ok(out_data)
}
