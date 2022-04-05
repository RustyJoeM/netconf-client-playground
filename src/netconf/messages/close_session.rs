use serde::{Deserialize, Serialize};

use crate::netconf::common::XMLNS;

/// Request RPC for "close-session".
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct CloseSessionRequest {
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "close-session")]
    pub close_session: CloseSession,
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct CloseSession {}

impl CloseSessionRequest {
    pub fn new(message_id: u32) -> Self {
        Self {
            xmlns: XMLNS.to_string(),
            message_id,
            close_session: CloseSession {},
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "rpc-reply")]
pub struct CloseSessionResponse {
    xmlns: String,
    #[serde(rename = "message-id")]
    message_id: u32,
    #[serde(flatten)]
    response: CloseSessionResponseType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum CloseSessionResponseType {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "rpc-error")]
    RpcError(String),
}
