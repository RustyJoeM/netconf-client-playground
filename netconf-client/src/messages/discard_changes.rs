use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::validate_capability_presence,
    types::{Capability, SimpleResponse},
};

use super::NetconfRequest;

/// Representation of NETCONF `<discard-changes>` operation request -
/// to revert the candidate configuration to the current running configuration.
#[derive(Debug, Serialize, Clone)]
#[serde(into = "DiscardChangesRequestRpc")]
pub struct DiscardChangesRequest {
    message_id: String,
    xmlns: String,
}

impl NetconfRequest for DiscardChangesRequest {
    type Response = DiscardChangesResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        validate_capability_presence(&Capability::Candidate, server_capabilities, "")
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
    /// Creates new instance of NETCONF `<discard-changes>` operation request.
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

/// Representation of a server response to [`DiscardChangesRequest`].
pub type DiscardChangesResponse = SimpleResponse;
