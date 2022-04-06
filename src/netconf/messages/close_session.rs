use serde::{Deserialize, Serialize};

use crate::netconf::{
    common::XMLNS,
    types::{RpcError, RpcReply},
};

/// Request RPC for "close-session".
#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
pub struct CloseSessionRequest {
    xmlns: String,
    #[serde(rename = "message-id")]
    message_id: String,
    #[serde(rename = "close-session")]
    close_session: CloseSession,
}

#[derive(Debug, Serialize, Default)]
pub struct CloseSession {}

impl CloseSessionRequest {
    pub fn new(message_id: String) -> Self {
        Self {
            xmlns: XMLNS.to_string(),
            message_id,
            close_session: CloseSession {},
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "CloseSessionResponseRpc")]
pub struct CloseSessionResponse {
    message_id: String,
    xmlns: String,
    reply: RpcReply,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "rpc")]
struct CloseSessionResponseRpc {
    #[serde(rename = "message-id")]
    pub message_id: String,
    pub xmlns: String,
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<RpcError>,
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
