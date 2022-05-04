use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, Datastore, SimpleResponse},
};

use super::NetconfRequest;

/// The \<unlock\> request for releasing the short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "UnlockRequestRpc")]
pub struct UnlockRequest {
    message_id: String,
    target: Datastore,
}

impl UnlockRequest {
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self { message_id, target }
    }
}

impl NetconfRequest for UnlockRequest {
    type Response = UnlockResponse;
}

/// Private representation of \<lock\> RPC used for serialization.
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

pub type UnlockResponse = SimpleResponse;
