use atrium_api::types::string::{Cid, Did};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    atproto::{get_blob_from_pds, resolve_pds_endpoint},
    errors::{CdnError, Result},
    manip::{get_image_bytes, read_image},
    presets::{ImageFormat, ImagePreset},
};

pub async fn get_blob(
    Path((preset, did, cid, format)): Path<(ImagePreset, Did, Cid, ImageFormat)>,
) -> Result<impl IntoResponse> {
    info!("loading preset: {:?}", preset);
    info!("image format: {:?}", format);

    let pds_endpoint = resolve_pds_endpoint(did.clone()).await?;
    let blob_res = get_blob_from_pds(pds_endpoint, did, cid).await?;

    let img = read_image(&blob_res).map_err(|_| CdnError::InvalidImage)?;
    let out_bytes = get_image_bytes(img, format).map_err(|_| CdnError::InvalidImage)?;
    let img_format = image::guess_format(&out_bytes)?;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", img_format.to_mime_type().parse().unwrap());

    Ok((StatusCode::OK, headers, out_bytes))
}
