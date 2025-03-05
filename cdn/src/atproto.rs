use std::str::FromStr;

use atrium_api::{
    client::AtpServiceClient,
    com::atproto::sync::get_blob,
    types::string::{Cid, Did},
};
use atrium_common::resolver::Resolver;
use atrium_identity::did::{CommonDidResolver, CommonDidResolverConfig};
use atrium_xrpc_client::reqwest::ReqwestClient;
use cid::Cid as RawCid;
use multihash_codetable::{Code, MultihashDigest};

use crate::errors::{CdnError, Result};

#[tracing::instrument(level = "info")]
pub async fn resolve_pds_endpoint(did: Did) -> Result<String> {
    let did_resolver = CommonDidResolver::new(CommonDidResolverConfig {
        plc_directory_url: String::from("https://plc.directory"),
        http_client: ReqwestClient::new("").into(),
    });

    let resolved = did_resolver.resolve(&did).await?;
    let pds_endpoint = resolved
        .get_pds_endpoint()
        .ok_or(CdnError::NoPdsEndpointError)?;

    Ok(pds_endpoint)
}

#[tracing::instrument]
pub async fn get_blob_from_pds(pds_endpoint: String, did: Did, cid: Cid) -> Result<Vec<u8>> {
    let http_client = ReqwestClient::new(&pds_endpoint);
    let client = AtpServiceClient::new(http_client);

    let blob_res = &client
        .service
        .com
        .atproto
        .sync
        .get_blob(
            get_blob::ParametersData {
                did: did.clone(),
                cid: cid.clone(),
            }
            .into(),
        )
        .await?;

    Ok(blob_res.to_owned())
}

#[tracing::instrument(skip(data))]
pub fn verify_cid(cid: Cid, data: &[u8]) -> Result<()> {

    let req_cid = serde_json::to_string(&cid).unwrap().replace("\"", "");
    let req_cid_codec = RawCid::from_str(&req_cid).unwrap().codec();
    let pds_multihash = Code::Sha2_256.digest(data);
    let pds_cid = RawCid::new_v1(req_cid_codec, pds_multihash);

    match pds_cid.to_string() == req_cid {
        true => Ok(()),
        false => Err(CdnError::InvalidImage),
    }
}
