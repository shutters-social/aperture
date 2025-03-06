use atrium_api::com::atproto::sync::get_blob;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CdnError {
    #[error("did: failed to resolve did doc")]
    DidDocResolveError(#[from] atrium_identity::Error),

    #[error("did: no pds endpoint on record")]
    NoPdsEndpointError,

    #[error("blob: failed to fetch blob from pds")]
    GetBlobError(#[from] atrium_xrpc::Error<get_blob::Error>),

    #[error("image: blob was not a valid image")]
    InvalidImage,

    #[error("image: blob was not a valid image (unknown image type)")]
    UnknownImageType(#[from] image::ImageError),

    #[error("image: failure to process image")]
    VipsError(#[from] libvips::error::Error),

    #[error("internal server error")]
    ServerError,
}

pub type Result<T, E = CdnError> = anyhow::Result<T, E>;

impl IntoResponse for CdnError {
    #[tracing::instrument(level = "trace", skip(self))]
    fn into_response(self) -> Response {
        let err_string = self.to_string();

        match self {
            CdnError::DidDocResolveError(err) => (
                StatusCode::BAD_REQUEST,
                Json(CdnErrorResponse {
                    error: err_string,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            CdnError::NoPdsEndpointError => (
                StatusCode::NOT_FOUND,
                Json(CdnErrorResponse {
                    error: err_string,
                    reason: None,
                }),
            ).into_response(),

            CdnError::GetBlobError(err) => (
                StatusCode::BAD_REQUEST,
                Json(CdnErrorResponse {
                    error: err_string,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            CdnError::InvalidImage => (
                StatusCode::BAD_REQUEST,
                Json(CdnErrorResponse {
                    error: err_string,
                    reason: None,
                })
            ).into_response(),

            CdnError::UnknownImageType(err) => (
                StatusCode::BAD_REQUEST,
                Json(CdnErrorResponse {
                    error: err_string,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            CdnError::VipsError(err) => (
                StatusCode::BAD_REQUEST,
                Json(CdnErrorResponse {
                    error: err_string,
                    reason: Some(err.to_string()),
                })
            ).into_response(),

            CdnError::ServerError => (
                StatusCode::BAD_REQUEST,
                Json(CdnErrorResponse {
                    error: err_string,
                    reason: None,
                })
            ).into_response(),
        }
    }
}

#[derive(Serialize)]
pub struct CdnErrorResponse {
    pub error: String,
    pub reason: Option<String>,
}
