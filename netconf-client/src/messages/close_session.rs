use serde::Serialize;

use crate::{common::XMLNS, types::SimpleResponse};

use super::NetconfRequest;

/// Request for \<close-session\> operation.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "CloseSessionRequestRpc")]
pub struct CloseSessionRequest {
    pub message_id: String,
    pub xmlns: String,
}

impl NetconfRequest for CloseSessionRequest {
    type Response = CloseSessionResponse;
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

pub type CloseSessionResponse = SimpleResponse;