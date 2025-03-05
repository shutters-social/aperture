use image::{DynamicImage, imageops::FilterType};
use serde::{Deserialize, Serialize};
use tracing::instrument;

const AVATAR_WIDTH: u32 = 256;
const AVATAR_HEIGHT: u32 = 256;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImagePreset {
    #[serde(rename = "feed_thumbnail")]
    FeedThumbnail,

    #[serde(rename = "feed_fullsize")]
    FeedFullsize,

    #[serde(rename = "avatar")]
    Avatar,
}

impl ImagePreset {
    #[instrument(skip(img))]
    pub fn process(self, img: DynamicImage) -> DynamicImage {
        match self {
            Self::Avatar => img.resize_to_fill(AVATAR_WIDTH, AVATAR_HEIGHT, FilterType::Gaussian),
            Self::FeedThumbnail => img.resize_to_fill(512, 512, FilterType::Gaussian),
            _ => img,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageFormat {
    #[serde(rename = "jpeg")]
    JPEG,

    #[serde(rename = "png")]
    PNG,

    #[serde(rename = "webp")]
    WEBP,
}

impl Into<image::ImageFormat> for ImageFormat {
    #[tracing::instrument(level = "trace", skip(self))]
    fn into(self) -> image::ImageFormat {
        match self {
            ImageFormat::JPEG => image::ImageFormat::Jpeg,
            ImageFormat::PNG => image::ImageFormat::Png,
            ImageFormat::WEBP => image::ImageFormat::WebP,
        }
    }
}
