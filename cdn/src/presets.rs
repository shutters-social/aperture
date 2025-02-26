use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ImagePreset {
    #[serde(rename = "feed_thumbnail")]
    FeedThumbnail,

    #[serde(rename = "feed_fullsize")]
    FeedFullsize,

    #[serde(rename = "avatar")]
    Avatar,
}

#[derive(Deserialize, Debug)]
pub enum ImageFormat {
    #[serde(rename = "jpeg")]
    JPEG,

    #[serde(rename = "png")]
    PNG,

    #[serde(rename = "webp")]
    WEBP,
}

impl Into<image::ImageFormat> for ImageFormat {
    fn into(self) -> image::ImageFormat {
        match self {
            ImageFormat::JPEG => image::ImageFormat::Jpeg,
            ImageFormat::PNG => image::ImageFormat::Png,
            ImageFormat::WEBP => image::ImageFormat::WebP,
        }
    }
}
