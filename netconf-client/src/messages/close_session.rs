use serde::Serialize;

use crate::{common::XMLNS, types::SimpleResponse};

use super::NetconfRequest;

/// Representation of NETCONF `<close-session>` operation request - for graceful termination of a NETCONF session.
#[derive(Debug, Clone, Serialize)]
#[serde(into = "CloseSessionRequestRpc")]
pub struct CloseSessionRequest {
    message_id: String,
    xmlns: String,
}

impl NetconfRequest for CloseSessionRequest {
    type Response = CloseSessionResponse;
}

/// Private RPC representation of `<close-session>` request.
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
    /// Creates new instance of `<close-session>` operation request..
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

/// Representation of a server response to [`CloseSessionRequest`].
pub type CloseSessionResponse = SimpleResponse;
