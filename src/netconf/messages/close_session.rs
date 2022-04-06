use serde::{Deserialize, Serialize};

use crate::netconf::{common::XMLNS, types::RpcReply};

/// Request RPC for "close-session".
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct CloseSessionRequest {
    xmlns: String,
    #[serde(rename = "message-id")]
    message_id: String,
    #[serde(rename = "close-session")]
    close_session: CloseSession,
}

#[derive(Debug, Serialize, PartialEq, Default)]
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
#[serde(rename = "rpc")]
pub struct CloseSessionResponse {
    #[serde(rename = "message-id")]
    message_id: String,
    xmlns: String,
    #[serde(rename = "$value")]
    reply: RpcReply,
}
