use std::fmt::Debug;

use serde::Serialize;

use crate::{common::XMLNS, types::SimpleResponse};

#[derive(Debug, Serialize, Clone)]
#[serde(into = "DiscardChangesRequestRpc")]
pub struct DiscardChangesRequest {
    pub message_id: String,
    pub xmlns: String,
}

impl super::NetconfRequest for DiscardChangesRequest {
    type Response = DiscardChangesResponse;

    fn to_netconf_rpc(&self) -> anyhow::Result<String> {
        let res = quick_xml::se::to_string(&self)?;
        Ok(res)
    }
}

impl From<DiscardChangesRequest> for DiscardChangesRequestRpc {
    fn from(request: DiscardChangesRequest) -> Self {
        DiscardChangesRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            discard_changes: DiscardChangesRpc {},
        }
    }
}

impl DiscardChangesRequest {
    pub fn new(message_id: String) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct DiscardChangesRpc;

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct DiscardChangesRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "discard-changes")]
    discard_changes: DiscardChangesRpc,
}

pub type DiscardChangesResponse = SimpleResponse;
