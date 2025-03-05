use std::env;

use atrium_api::types::string::{Cid, Did};
use aws_config::SdkConfig;
use aws_sdk_s3::operation::get_object::GetObjectError;
use lazy_static::lazy_static;

use crate::{errors::{CdnError, Result}, presets::{ImageFormat, ImagePreset}};

lazy_static! {
    static ref AWS_BLOB_BUCKET: String = env::var("AWS_BLOB_BUCKET").unwrap_or(String::from("")).to_string();
}

#[derive(Clone, Debug)]
pub struct CacheParameters {
    pub pds_endpoint: String,
    pub preset: ImagePreset,
    pub format: ImageFormat,
    pub did: Did,
    pub cid: Cid,
}

impl CacheParameters {
    pub fn to_s3_key(&self) -> String {
        format!(
            "{}/{}/{}.{}",
            serde_json::to_string_pretty(&self.preset).unwrap().replace("\"", ""),
            self.did.to_string(),
            serde_json::to_string_pretty(&self.cid).unwrap().replace("\"", ""),
            serde_json::to_string_pretty(&self.format).unwrap().replace("\"", ""),
        )
    }
}

#[derive(Clone, Debug)]
pub struct AwsWrapper(aws_config::SdkConfig);

impl AwsWrapper {
    pub fn new(config: SdkConfig) -> Self {
        AwsWrapper(config)
    }

    fn s3_client(&self) -> aws_sdk_s3::Client {
        aws_sdk_s3::Client::new(&self.0)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_cached_blob(&self, params: CacheParameters) -> Result<Option<Vec<u8>>> {
        let result = self.s3_client().get_object()
            .bucket(AWS_BLOB_BUCKET.clone())
            .key(params.to_s3_key())
            .send()
            .await;

        match result {
            Ok(object) => {
                let body = object.body.collect().await.map_err(|err| {
                    tracing::error!("object read err: {}", err);
                    CdnError::ServerError
                })?;
                Ok(Some(body.to_vec()))
            }

            Err(err) => match err.into_service_error() {
                GetObjectError::NoSuchKey(_) => Ok(None),
                err => {
                    tracing::error!("aws error: {}", err);
                    Err(CdnError::ServerError)
                }
            }
        }
    }

    #[tracing::instrument(skip(self, blob))]
    pub async fn store_blob(&self, params: CacheParameters, blob: Vec<u8>) -> Result<()> {
        let result = self.s3_client().put_object()
            .bucket(AWS_BLOB_BUCKET.clone())
            .key(params.to_s3_key())
            .body(blob.into())
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err.into_service_error() {
                err => {
                    tracing::error!("aws error: {}", err);
                    Err(CdnError::ServerError)
                }
            }
        }
    }
}
