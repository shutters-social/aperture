use atrium_api::types::string::{Cid, Did};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::{
    atproto::{get_blob_from_pds, resolve_pds_endpoint, verify_cid},
    aws::CacheParameters,
    errors::{CdnError, Result},
    manip::{get_image_bytes, read_image},
    presets::{ImageFormat, ImagePreset},
    state::AppState,
};

pub async fn get_blob(
    State(app_state): State<AppState>,
    Path((preset, did, cid, format)): Path<(ImagePreset, Did, Cid, ImageFormat)>,
) -> Result<impl IntoResponse> {
    let pds_endpoint = resolve_pds_endpoint(did.clone()).await?;

    let cache_params = CacheParameters {
        pds_endpoint: pds_endpoint.clone(),
        preset: preset.clone(),
        format: format.clone(),
        did: did.clone(),
        cid: cid.clone(),
    };

    let blob = match app_state.clone()
        .aws()
        .get_cached_blob(cache_params.clone())
        .await?
    {
        Some(blob_data) => {
            tracing::debug!("blob cache hit");
            blob_data
        },
        None => {
            tracing::debug!("blob cache miss");
            let blob_data = get_blob_from_pds(pds_endpoint.clone(), did.clone(), cid.clone()).await?;
            verify_cid(cid, &blob_data.clone())?;
            let img = preset.process(read_image(&blob_data).map_err(|_| CdnError::InvalidImage)?);
            let out_bytes = get_image_bytes(img.clone(), format).map_err(|_| CdnError::InvalidImage)?;
            app_state.aws().store_blob(cache_params, out_bytes.clone()).await?;
            out_bytes
        },
    };

    let img_format = image::guess_format(&blob)?;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", img_format.to_mime_type().parse().unwrap());

    Ok((StatusCode::OK, headers, blob))
}
