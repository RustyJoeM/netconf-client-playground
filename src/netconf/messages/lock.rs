use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::XMLNS,
    types::{Datastore, RpcReply},
};

/// The \<lock\> request for short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "LockRequestRpc")]
pub struct LockRequest {
    #[serde(rename = "message-id")]
    message_id: String,
    target: Datastore,
}

impl LockRequest {
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self { message_id, target }
    }
}

/// Private representation of \<lock\> RPC used for serialization.
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
    target: TargetRpc,
}

#[derive(Debug, Serialize)]
struct TargetRpc {
    #[serde(rename = "$value")]
    datastore: Datastore,
}

impl From<LockRequest> for LockRequestRpc {
    fn from(request: LockRequest) -> Self {
        LockRequestRpc {
            xmlns: XMLNS.to_string(),
            message_id: request.message_id,
            lock: LockRpc {
                target: TargetRpc {
                    datastore: request.target,
                },
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LockResponse {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "$value")]
    reply: RpcReply,
}
