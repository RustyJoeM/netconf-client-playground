use serde::Serialize;

use crate::{
    common::XMLNS,
    types::{tag_wrapper::TagWrapper, SimpleResponse},
};

use super::NetconfRequest;

/// Representation of NETCONF `<kill-session>` operation request.
/// Force the termination of a NETCONF session.
#[derive(Debug, Serialize, Clone)]
#[serde(into = "KillSessionRequestRpc")]
pub struct KillSessionRequest {
    message_id: String,
    xmlns: String,
    session_id: u32,
}

impl NetconfRequest for KillSessionRequest {
    type Response = KillSessionResponse;
}

impl KillSessionRequest {
    /// Create new instance of `<kill-session>` request.
    pub fn new(message_id: String, session_id: u32) -> Self {
        Self {
            message_id,
            xmlns: XMLNS.to_string(),
            session_id,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "rpc")]
#[serde(rename_all = "kebab-case")]
struct KillSessionRequestRpc {
    message_id: String,
    xmlns: String,
    kill_session: KillSessionRpc,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct KillSessionRpc {
    session_id: TagWrapper<u32>,
}

impl From<KillSessionRequest> for KillSessionRequestRpc {
    fn from(request: KillSessionRequest) -> Self {
        KillSessionRequestRpc {
            message_id: request.message_id,
            xmlns: request.xmlns,
            kill_session: KillSessionRpc {
                session_id: request.session_id.into(),
            },
        }
    }
}

/// Representation of a server response to [`KillSessionRequest`].
pub type KillSessionResponse = SimpleResponse;
