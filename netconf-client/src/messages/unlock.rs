use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::validate_datastore_capability,
    types::{tag_wrapper::TagWrapper, Capability, Datastore, SimpleResponse},
};

use super::NetconfRequest;

/// Representation of NETCONF `<unlock>` operation request - for releasing the short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "UnlockRequestRpc")]
pub struct UnlockRequest {
    message_id: String,
    target: Datastore,
}

impl UnlockRequest {
    /// Creates new instance of NETCONF `<unlock>` operation request.
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self { message_id, target }
    }
}

impl NetconfRequest for UnlockRequest {
    type Response = UnlockResponse;

    fn validate_request(
        &self,
        server_capabilities: &[crate::types::Capability],
    ) -> anyhow::Result<()> {
        validate_datastore_capability(
            &self.target,
            &Datastore::Candidate,
            &Capability::Candidate,
            server_capabilities,
        )
    }
}

/// Private representation of `<lock>` RPC used for serialization.
#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct UnlockRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    unlock: UnlockRpc,
}

#[derive(Debug, Serialize)]
struct UnlockRpc {
    target: TagWrapper<Datastore>,
}

impl From<UnlockRequest> for UnlockRequestRpc {
    fn from(request: UnlockRequest) -> Self {
        UnlockRequestRpc {
            xmlns: XMLNS.to_string(),
            message_id: request.message_id,
            unlock: UnlockRpc {
                target: request.target.into(),
            },
        }
    }
}

/// Representation of a server response to [`UnlockRequest`].
pub type UnlockResponse = SimpleResponse;
