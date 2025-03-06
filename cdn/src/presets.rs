use libvips::VipsImage;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::errors::Result;

const AVATAR_WIDTH: i32 = 256;
const AVATAR_HEIGHT: i32 = 256;

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
    pub fn process(self, img: VipsImage) -> Result<VipsImage> {
        match self {
            Self::Avatar => {
                let width = img.get_width();
                let height = img.get_height();

                let hshrink = width / AVATAR_WIDTH;
                let vshrink = height / AVATAR_HEIGHT;

                let img = libvips::ops::shrinkh(&img, hshrink)?;
                let img = libvips::ops::shrinkv(&img, vshrink)?;

                Ok(img)
            },
            Self::FeedThumbnail => {
                let width = img.get_width();
                let height = img.get_height();

                let hshrink = width / AVATAR_WIDTH;
                let vshrink = height / AVATAR_HEIGHT;

                let img = libvips::ops::shrinkh(&img, hshrink)?;
                let img = libvips::ops::shrinkv(&img, vshrink)?;

                Ok(img)
            },
            _ => Ok(img),
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

impl ImageFormat {
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn to_mime_type(self) -> &'static str {
        match self {
            ImageFormat::JPEG => "image/jpeg",
            ImageFormat::PNG => "image/png",
            ImageFormat::WEBP => "image/webp",
        }
    }
}
