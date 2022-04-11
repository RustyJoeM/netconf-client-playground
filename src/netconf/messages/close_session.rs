use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::XMLNS,
    types::{RpcError, RpcReply},
};

/// Request for \<close-session\> operation.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "CloseSessionRequestRpc")]
pub struct CloseSessionRequest {
    message_id: String,
    xmlns: String,
}

/// Private RPC representation of \<close-session\> request.
#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
struct CloseSessionRequestRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "close-session")]
    close_session: CloseSession,
}

#[derive(Debug, Serialize, Default)]
struct CloseSession {}

impl CloseSessionRequest {
    pub fn new(message_id: String) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
        }
    }
}

impl From<CloseSessionRequest> for CloseSessionRequestRpc {
    fn from(request: CloseSessionRequest) -> Self {
        CloseSessionRequestRpc {
            xmlns: request.xmlns,
            message_id: request.message_id,
            close_session: CloseSession {},
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "CloseSessionResponseRpc")]
pub struct CloseSessionResponse {
    pub message_id: String,
    pub xmlns: String,
    pub reply: RpcReply,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "rpc")]
struct CloseSessionResponseRpc {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    ok: Option<()>,
    #[serde(rename = "rpc-error")]
    rpc_error: Option<RpcError>,
}

impl From<CloseSessionResponseRpc> for CloseSessionResponse {
    fn from(rpc: CloseSessionResponseRpc) -> Self {
        CloseSessionResponse {
            message_id: rpc.message_id,
            xmlns: rpc.xmlns,
            reply: match rpc.ok.is_some() {
                true => RpcReply::Ok,
                false => RpcReply::Error(rpc.rpc_error.unwrap()),
            },
        }
    }
}
