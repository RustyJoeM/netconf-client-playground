use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::XMLNS,
    types::{Datastore, RpcErrorRpc, RpcReply},
};

/// The \<lock\> request for short-lived restriction of datastore access.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "UnlockRequestRpc")]
pub struct UnlockRequest {
    #[serde(rename = "message-id")]
    message_id: String,
    target: Datastore,
}

impl UnlockRequest {
    pub fn new(message_id: String, target: Datastore) -> Self {
        Self { message_id, target }
    }
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
    target: TargetRpc,
}

#[derive(Debug, Serialize)]
struct TargetRpc {
    #[serde(rename = "$value")]
    datastore: Datastore,
}

impl From<UnlockRequest> for UnlockRequestRpc {
    fn from(request: UnlockRequest) -> Self {
        UnlockRequestRpc {
            xmlns: XMLNS.to_string(),
            message_id: request.message_id,
            unlock: UnlockRpc {
                target: TargetRpc {
                    datastore: request.target,
                },
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "UnlockResponseRpc")]
pub struct UnlockResponse {
    message_id: String,
    xmlns: String,
    reply: RpcReply,
}

#[derive(Debug, Deserialize)]
pub struct UnlockResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<RpcErrorRpc>,
}

impl From<UnlockResponseRpc> for UnlockResponse {
    fn from(rpc: UnlockResponseRpc) -> Self {
        UnlockResponse {
            message_id: rpc.message_id,
            xmlns: rpc.xmlns,
            reply: match rpc.ok.is_some() {
                true => RpcReply::Ok,
                false => RpcReply::Error(rpc.rpc_error.unwrap().into()),
            },
        }
    }
}
