use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::validate_capability_presence,
    types::{tag_wrapper::TagWrapper, Capability, SimpleResponse},
};

use super::NetconfRequest;

#[derive(Debug, Serialize, Clone)]
#[serde(into = "CancelCommitRequestRpc")]
pub struct CancelCommitRequest {
    pub message_id: String,
    pub xmlns: String,
    pub persist_id: Option<u32>,
}

impl NetconfRequest for CancelCommitRequest {
    type Response = CancelCommitResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        validate_capability_presence(
            &Capability::ConfirmedCommit,
            server_capabilities,
            " Cannot perform <cancel-commit> operation.",
        )
    }
}

impl From<CancelCommitRequest> for CancelCommitRequestRpc {
    fn from(request: CancelCommitRequest) -> Self {
        CancelCommitRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            cancel_commit: CancelCommitRpc {
                persist_id: request.persist_id.map(TagWrapper::new),
            },
        }
    }
}

impl CancelCommitRequest {
    pub fn new(message_id: String, persist_id: Option<u32>) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            persist_id,
        }
    }
}

#[derive(Debug, Serialize)]
struct CancelCommitRpc {
    persist_id: Option<TagWrapper<u32>>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct CancelCommitRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "cancel-commit")]
    cancel_commit: CancelCommitRpc,
}

pub type CancelCommitResponse = SimpleResponse;
