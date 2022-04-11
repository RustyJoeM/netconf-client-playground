use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::XMLNS,
    types::{Datastore, RpcErrorRpc, RpcReply},
};

/// The \<lock\> request for short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "LockRequestRpc")]
pub struct LockRequest {
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

/// Response for the \<lock\> operation.
#[derive(Debug, Deserialize)]
#[serde(from = "LockResponseRpc")]
pub struct LockResponse {
    pub message_id: String,
    pub xmlns: String,
    pub reply: RpcReply,
}

#[derive(Debug, Deserialize)]
struct LockResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    ok: Option<()>,
    #[serde(rename = "rpc-error")]
    rpc_error: Option<RpcErrorRpc>,
}

impl From<LockResponseRpc> for LockResponse {
    fn from(rpc: LockResponseRpc) -> Self {
        LockResponse {
            message_id: rpc.message_id,
            xmlns: rpc.xmlns,
            reply: match rpc.ok.is_some() {
                true => RpcReply::Ok,
                false => RpcReply::Error(rpc.rpc_error.unwrap().into()),
            },
        }
    }
}
