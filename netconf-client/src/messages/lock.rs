use serde::Serialize;

use crate::{
    common::XMLNS,
    message_validation::validate_datastore_capability,
    types::{tag_wrapper::TagWrapper, Capability, Datastore, SimpleResponse},
};

use super::NetconfRequest;

/// Representation of NETCONF `<lock>` operation request - for short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "LockRequestRpc")]
pub struct LockRequest {
    message_id: String,
    target: Datastore,
}

impl LockRequest {
    /// Creates new instance of NETCONF `<lock>` operation request.
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self { message_id, target }
    }
}

impl NetconfRequest for LockRequest {
    type Response = LockResponse;

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
struct LockRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    lock: LockRpc,
}

#[derive(Debug, Serialize)]
struct LockRpc {
    target: TagWrapper<Datastore>,
}

impl From<LockRequest> for LockRequestRpc {
    fn from(request: LockRequest) -> Self {
        LockRequestRpc {
            xmlns: XMLNS.to_string(),
            message_id: request.message_id,
            lock: LockRpc {
                target: request.target.into(),
            },
        }
    }
}

/// Representation of a server response to [`LockRequest`].
pub type LockResponse = SimpleResponse;
