use crate::errors::Result;
use libvips::{
    VipsImage,
    ops::{jpegsave_buffer, pngsave_buffer, webpsave_buffer},
};

use crate::presets::ImageFormat;

#[tracing::instrument(skip(image_data))]
pub fn read_image(image_data: &[u8]) -> Result<VipsImage> {
    Ok(VipsImage::new_from_buffer(image_data, "")?)
}

#[tracing::instrument(skip(image))]
pub fn get_image_bytes(image: VipsImage, format: ImageFormat) -> Result<Vec<u8>> {
    let bytes = match format {
        ImageFormat::PNG => pngsave_buffer(&image),
        ImageFormat::JPEG => jpegsave_buffer(&image),
        ImageFormat::WEBP => webpsave_buffer(&image),
    }?;

    Ok(bytes)
}
